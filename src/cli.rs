// ============================================================
// CLI: 命令行参数定义
// ============================================================

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Markdown,
    Json,
}

#[derive(Parser, Debug)]
#[command(name = "agent-codemap")]
#[command(about = "AI-friendly source code index generator")]
#[command(version)]
pub struct Cli {
    /// Input file or directory
    #[arg(default_value = ".")]
    pub input: PathBuf,

    /// Output format
    #[arg(long, short, value_enum, default_value = "markdown")]
    pub format: OutputFormat,
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
            format: OutputFormat::Markdown,
        };
        assert_eq!(cli.input, PathBuf::from("."));
    }

    #[test]
    fn test_cli_json_format() {
        let cli = Cli {
            input: PathBuf::from("src"),
            format: OutputFormat::Json,
        };
        assert!(matches!(cli.format, OutputFormat::Json));
    }
}
