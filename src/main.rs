// ============================================================
// agent-codemap: AI 代码索引生成器
// ============================================================

mod cli;
mod detector;
mod extractor;
mod output;
mod scanner;
mod symbol;

use anyhow::Result;
use cli::Cli;
use symbol::FileMap;

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    let result = run(&cli)?;
    print!("{}", result);
    Ok(())
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
