// ============================================================
// Watch: 文件监听模式 (增量更新)
// ============================================================

use crate::detector;
use crate::extractor;
use crate::output;
use crate::scanner;
use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};

// ------------------------------------------------------------
// 内存缓存条目
// ------------------------------------------------------------
struct CacheEntry {
    mtime: u64,
    size: u64,
}

// ------------------------------------------------------------
// Watch 状态
// ------------------------------------------------------------
struct WatchState {
    cache: HashMap<PathBuf, CacheEntry>,
    input: PathBuf,
    output: PathBuf,
}

impl WatchState {
    fn new(input: &Path, output: &Path) -> Self {
        Self {
            cache: HashMap::new(),
            input: input.canonicalize().unwrap_or_else(|_| input.to_path_buf()),
            output: output.to_path_buf(),
        }
    }

    fn get_file_meta(path: &Path) -> Option<(u64, u64)> {
        let meta = fs::metadata(path).ok()?;
        let mtime = meta.modified().ok()?
            .duration_since(SystemTime::UNIX_EPOCH).ok()?
            .as_secs();
        Some((mtime, meta.len()))
    }

    fn needs_reparse(&self, path: &Path) -> bool {
        if let Some(entry) = self.cache.get(path) {
            if let Some((mtime, size)) = Self::get_file_meta(path) {
                return entry.mtime != mtime || entry.size != size;
            }
        }
        true
    }

    /// 解析并写入输出文件
    fn parse_and_write(&mut self, abs_path: &Path) -> Option<()> {
        let lang = detector::detect(abs_path)?;

        let rel_path = abs_path.strip_prefix(&self.input)
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| abs_path.to_path_buf());

        match extractor::extract(abs_path, &lang) {
            Ok(mut file_map) => {
                file_map.path = rel_path.to_string_lossy().to_string();

                // 写入输出文件
                let out_path = scanner::output_path(&self.input, abs_path, &self.output);
                if let Err(e) = output::write_single(&file_map, &out_path) {
                    eprintln!("Warning: failed to write {}: {}", out_path.display(), e);
                    return None;
                }

                // 更新缓存
                if let Some((mtime, size)) = Self::get_file_meta(abs_path) {
                    self.cache.insert(abs_path.to_path_buf(), CacheEntry {
                        mtime,
                        size,
                    });
                }
                Some(())
            }
            Err(e) => {
                eprintln!("Warning: failed to parse {}: {}", rel_path.display(), e);
                None
            }
        }
    }

    /// 删除输出文件
    fn remove(&mut self, abs_path: &Path) {
        self.cache.remove(abs_path);
        let out_path = scanner::output_path(&self.input, abs_path, &self.output);
        let _ = fs::remove_file(&out_path);
    }

    /// 全量扫描
    fn full_scan(&mut self) -> Result<()> {
        let files = scanner::scan(&self.input, &self.output)?;

        let mut parsed = 0;
        let total = files.len();

        for path in files {
            let abs_path = path.canonicalize().unwrap_or(path);
            if self.parse_and_write(&abs_path).is_some() {
                parsed += 1;
            }
        }

        eprintln!("Parsed {}/{} files", parsed, total);
        Ok(())
    }

    /// 增量更新
    fn incremental_update(&mut self, changed_paths: &[PathBuf]) -> Result<usize> {
        let mut updated = 0;

        for path in changed_paths {
            let display_path = path.strip_prefix(&self.input)
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|_| path.clone());

            // 文件删除
            if !path.exists() {
                self.remove(path);
                eprintln!("  Removed: {}", display_path.display());
                updated += 1;
                continue;
            }

            // 跳过输出目录
            if path.starts_with(&self.output) {
                continue;
            }

            // 跳过不支持的文件
            if detector::detect(path).is_none() {
                continue;
            }

            // 需要重新解析
            if self.needs_reparse(path) {
                if self.parse_and_write(path).is_some() {
                    eprintln!("  Updated: {}", display_path.display());
                    updated += 1;
                }
            }
        }

        // 检查新文件
        let files = scanner::scan(&self.input, &self.output)?;
        for path in files {
            let abs_path = path.canonicalize().unwrap_or(path);
            if !self.cache.contains_key(&abs_path) {
                let display_path = abs_path.strip_prefix(&self.input)
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|_| abs_path.clone());
                if self.parse_and_write(&abs_path).is_some() {
                    eprintln!("  New: {}", display_path.display());
                    updated += 1;
                }
            }
        }

        Ok(updated)
    }
}

// ------------------------------------------------------------
// 公共接口
// ------------------------------------------------------------

pub fn run(input: &Path, output: &Path) -> Result<()> {
    eprintln!("Watching: {}", input.display());
    eprintln!("Output: {}", output.display());
    eprintln!("Press Ctrl+C to exit\n");

    let mut state = WatchState::new(input, output);
    state.full_scan()?;

    let abs_input = input.canonicalize().unwrap_or_else(|_| input.to_path_buf());
    let output_abs = output.canonicalize().unwrap_or_else(|_| {
        std::env::current_dir().unwrap_or_default().join(output)
    });

    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        Config::default().with_poll_interval(Duration::from_secs(1)),
    )?;

    watcher.watch(&abs_input, RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(event) => {
                if should_process_event(&event, &output_abs) {
                    match state.incremental_update(&event.paths) {
                        Ok(updated) if updated > 0 => {
                            eprintln!("\nUpdated {} files", updated);
                        }
                        Ok(_) => {}
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn should_process_event(event: &notify::Event, output_dir: &Path) -> bool {
    use notify::EventKind;

    // 忽略输出目录的变化
    for path in &event.paths {
        if path.starts_with(output_dir) {
            return false;
        }
    }

    matches!(
        event.kind,
        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
    )
}

// ============================================================
// 单元测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use notify::{Event, EventKind, event::{CreateKind, ModifyKind, AccessKind}};
    use tempfile::TempDir;

    #[test]
    fn test_watch_state_new() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        let state = WatchState::new(tmp.path(), &out);
        assert!(state.cache.is_empty());
    }

    #[test]
    fn test_get_file_meta() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("test.txt");
        fs::write(&file, "hello").unwrap();

        let meta = WatchState::get_file_meta(&file);
        assert!(meta.is_some());
        let (mtime, size) = meta.unwrap();
        assert!(mtime > 0);
        assert_eq!(size, 5);
    }

    #[test]
    fn test_needs_reparse_not_in_cache() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        let file = tmp.path().join("test.rs");
        fs::write(&file, "fn main() {}").unwrap();

        let state = WatchState::new(tmp.path(), &out);
        assert!(state.needs_reparse(&file));
    }

    #[test]
    fn test_should_process_event_create() {
        let output = PathBuf::from("/out");
        let event = Event {
            kind: EventKind::Create(CreateKind::File),
            paths: vec![PathBuf::from("/test.rs")],
            attrs: Default::default(),
        };
        assert!(should_process_event(&event, &output));
    }

    #[test]
    fn test_should_process_event_output_ignored() {
        let output = PathBuf::from("/out");
        let event = Event {
            kind: EventKind::Modify(ModifyKind::Data(notify::event::DataChange::Content)),
            paths: vec![PathBuf::from("/out/test.md")],
            attrs: Default::default(),
        };
        assert!(!should_process_event(&event, &output));
    }

    #[test]
    fn test_should_process_event_access_ignored() {
        let output = PathBuf::from("/out");
        let event = Event {
            kind: EventKind::Access(AccessKind::Read),
            paths: vec![PathBuf::from("/test.rs")],
            attrs: Default::default(),
        };
        assert!(!should_process_event(&event, &output));
    }

    #[test]
    fn test_should_process_event_remove() {
        let output = PathBuf::from("/out");
        let event = Event {
            kind: EventKind::Remove(notify::event::RemoveKind::File),
            paths: vec![PathBuf::from("/test.rs")],
            attrs: Default::default(),
        };
        assert!(should_process_event(&event, &output));
    }

    #[test]
    fn test_needs_reparse_in_cache_unchanged() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        let file = tmp.path().join("test.rs");
        fs::write(&file, "fn main() {}").unwrap();

        let mut state = WatchState::new(tmp.path(), &out);
        let (mtime, size) = WatchState::get_file_meta(&file).unwrap();
        state.cache.insert(file.clone(), CacheEntry { mtime, size });

        assert!(!state.needs_reparse(&file));
    }

    #[test]
    fn test_needs_reparse_size_changed() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        let file = tmp.path().join("test.rs");
        fs::write(&file, "fn main() {}").unwrap();

        let mut state = WatchState::new(tmp.path(), &out);
        let (mtime, _) = WatchState::get_file_meta(&file).unwrap();
        state.cache.insert(file.clone(), CacheEntry { mtime, size: 999 });

        assert!(state.needs_reparse(&file));
    }

    #[test]
    fn test_parse_and_write_success() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        let file = tmp.path().join("test.rs");
        fs::write(&file, "fn hello() {}").unwrap();

        let mut state = WatchState::new(tmp.path(), &out);
        let result = state.parse_and_write(&file);
        assert!(result.is_some());
        assert!(state.cache.contains_key(&file));
        assert!(out.join("test.rs.md").exists());
    }

    #[test]
    fn test_parse_and_write_unsupported() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        let file = tmp.path().join("test.txt");
        fs::write(&file, "hello").unwrap();

        let mut state = WatchState::new(tmp.path(), &out);
        let result = state.parse_and_write(&file);
        assert!(result.is_none());
    }

    #[test]
    fn test_remove() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        let file = tmp.path().join("test.rs");
        fs::write(&file, "fn hello() {}").unwrap();

        let mut state = WatchState::new(tmp.path(), &out);
        state.parse_and_write(&file);
        assert!(state.cache.contains_key(&file));

        state.remove(&file);
        assert!(!state.cache.contains_key(&file));
    }

    #[test]
    fn test_full_scan() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");

        fs::write(tmp.path().join("a.rs"), "fn a() {}").unwrap();
        fs::write(tmp.path().join("b.rs"), "fn b() {}").unwrap();

        let mut state = WatchState::new(tmp.path(), &out);
        let result = state.full_scan();
        assert!(result.is_ok());
        assert_eq!(state.cache.len(), 2);
    }

    #[test]
    fn test_incremental_update_new_file() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        let mut state = WatchState::new(tmp.path(), &out);

        // Create a new file
        let file = tmp.path().join("new.rs");
        fs::write(&file, "fn new() {}").unwrap();

        let result = state.incremental_update(&[file.clone()]);
        assert!(result.is_ok());
        assert!(state.cache.contains_key(&file));
    }

    #[test]
    fn test_incremental_update_modified_file() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        let file = tmp.path().join("test.rs");
        fs::write(&file, "fn old() {}").unwrap();

        let mut state = WatchState::new(tmp.path(), &out);
        state.parse_and_write(&file);

        // Modify file (change size to trigger reparse)
        fs::write(&file, "fn new_longer() {}").unwrap();

        let result = state.incremental_update(&[file.clone()]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_incremental_update_deleted_file() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        let file = tmp.path().join("test.rs");
        fs::write(&file, "fn hello() {}").unwrap();

        let mut state = WatchState::new(tmp.path(), &out);
        state.parse_and_write(&file);

        // Delete file
        fs::remove_file(&file).unwrap();

        let result = state.incremental_update(&[file.clone()]);
        assert!(result.is_ok());
        assert!(!state.cache.contains_key(&file));
    }

    #[test]
    fn test_incremental_update_skip_output_dir() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        let mut state = WatchState::new(tmp.path(), &out);

        // File in output dir should be skipped
        let out_file = out.join("test.md");
        fs::write(&out_file, "# test").unwrap();

        let result = state.incremental_update(&[out_file]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0);
    }

    #[test]
    fn test_incremental_update_skip_unsupported() {
        let tmp = TempDir::new().unwrap();
        let out = tmp.path().join("out");
        fs::create_dir(&out).unwrap();

        let mut state = WatchState::new(tmp.path(), &out);

        let file = tmp.path().join("readme.txt");
        fs::write(&file, "hello").unwrap();

        let result = state.incremental_update(&[file]);
        assert!(result.is_ok());
    }
}
