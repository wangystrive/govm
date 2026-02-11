use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "govm")]
#[command(about = "Go Version Manager - 管理多版本 Go")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 列出所有已安装的 Go 版本
    List,
    
    /// 列出所有可下载的远程 Go 版本
    ListRemote,
    
    /// 安装指定版本的 Go
    Install {
        /// 版本号，例如 1.21.5
        version: String,
    },
    
    /// 卸载指定版本的 Go
    Uninstall {
        /// 版本号，例如 1.21.5
        version: String,
    },
    
    /// 切换到指定版本的 Go
    Use {
        /// 版本号，例如 1.21.5
        version: String,
    },
    
    /// 显示当前使用的 Go 版本
    Current,
    
    /// 清理下载缓存
    Clean,
    
    /// 导入系统已安装的 Go
    Import,
    
    /// 显示系统 Go 信息
    System,
}
