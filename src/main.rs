// ============================================================
// agent-codemap: AI 代码索引生成器
// ============================================================

mod cli;
mod detector;
mod extractor;
mod output;
mod scanner;
mod symbol;
mod update;

use anyhow::Result;
use cli::Cli;
use symbol::FileMap;

fn main() -> Result<()> {
    // 后台检查更新 (24h 一次)
    let update_handle = update::base_dir().ok().and_then(|dir| {
        if update::should_check(&dir) {
            Some(update::spawn_background_check(dir))
        } else {
            None
        }
    });

    let cli = Cli::parse_args();
    let result = if cli.update {
        update::run_update()
    } else {
        let output = run(&cli)?;
        print!("{}", output);
        Ok(())
    };

    // 等待后台检查完成
    if let Some(handle) = update_handle {
        let _ = handle.join();
    }

    result
}

/// 扫描 → 解析 → 渲染
fn run(cli: &Cli) -> Result<String> {
    let files = scanner::scan(&cli.input)?;

    // 收集所有 FileMap
    let maps: Vec<FileMap> = files
        .iter()
        .filter_map(|path| {
            let lang = detector::detect(path)?;
            match extractor::extract(path, &lang) {
                Ok(mut map) => {
                    // 设置相对路径
                    let rel = scanner::relative_path(&cli.input, path);
                    map.path = rel.to_string_lossy().to_string();
                    Some(map)
                }
                Err(e) => {
                    eprintln!("Warning: failed to parse {}: {}", path.display(), e);
                    None
                }
            }
        })
        .collect();

    Ok(output::render_all(&maps, cli.format))
}
