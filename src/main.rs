// ============================================================
// agent-codemap: AI 代码索引生成器
// ============================================================

mod cli;
mod detector;
mod extractor;
mod output;
mod scanner;
mod symbol;
mod watch;

use anyhow::Result;
use cli::Cli;
use rayon::prelude::*;

fn main() -> Result<()> {
    let cli = Cli::parse_args();

    // Watch 模式
    if cli.watch {
        return watch::run(&cli.input, &cli.output);
    }

    // 普通模式
    run_once(&cli.input, &cli.output)
}

/// 单次运行: 扫描 → 解析 → 输出
pub fn run_once(input: &std::path::Path, output: &std::path::Path) -> Result<()> {
    let files = scanner::scan(input, output)?;

    // 并行处理文件
    files.par_iter().for_each(|path| {
        if let Some(lang) = detector::detect(path) {
            match extractor::extract(path, &lang) {
                Ok(map) => {
                    let out_path = scanner::output_path(input, path, output);
                    if let Err(e) = output::write_single(&map, &out_path) {
                        eprintln!("Warning: failed to write {}: {}", out_path.display(), e);
                    }
                }
                Err(e) => {
                    eprintln!("Warning: failed to parse {}: {}", path.display(), e);
                }
            }
        }
    });

    Ok(())
}
