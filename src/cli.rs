// ============================================================
// CLI: 命令行参数定义
// ============================================================

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "agent-codemap")]
#[command(about = "AI-friendly source code index generator")]
#[command(version)]
pub struct Cli {
    /// 输入目录
    #[arg(default_value = ".")]
    pub input: PathBuf,

    /// 输出目录 (必填)
    #[arg(long, short)]
    pub output: PathBuf,

    /// Watch 模式 (监听文件变化)
    #[arg(long, short)]
    pub watch: bool,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

// ============================================================
// 单元测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_default_input() {
        let cli = Cli {
            input: PathBuf::from("."),
            output: PathBuf::from("out"),
            watch: false,
        };
        assert_eq!(cli.input, PathBuf::from("."));
    }
}
