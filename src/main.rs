mod cli;
mod version;
mod download;
mod install;
mod switch;
mod utils;
mod system_go;

use anyhow::{Result, Context};
use clap::Parser;
use colored::Colorize;
use reqwest::Client;
use std::io::Write;

use cli::{Cli, Commands};
use version::GoVersion;
use install::{install_version, uninstall_version, is_version_installed, get_installed_versions};
use switch::{switch_version, get_current_version, get_current_go_path};
use system_go::{detect_system_go, import_system_go, show_path_conflict_warning, get_import_hint};
use utils::get_cache_dir;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 确保必要的目录存在
    let _ = get_cache_dir()?;

    match cli.command {
        Commands::List => {
            list_installed_versions().await?;
        }
        Commands::ListRemote => {
            list_remote_versions().await?;
        }
        Commands::Install { version } => {
            install_go_version(&version).await?;
        }
        Commands::Uninstall { version } => {
            uninstall_go_version(&version).await?;
        }
        Commands::Use { version } => {
            use_go_version(&version).await?;
        }
        Commands::Current => {
            show_current_version().await?;
        }
        Commands::Clean => {
            utils::clean_cache()?;
        }
        Commands::Import => {
            import_system_go()?;
        }
        Commands::System => {
            show_system_go_info().await?;
        }
    }

    Ok(())
}

/// 显示系统 Go 信息
async fn show_system_go_info() -> Result<()> {
    match detect_system_go()? {
        Some(system) => {
            println!("{}", "System Go Information".bold().green());
            println!();
            println!("  Version:     {}", system.version.to_string().cyan());
            println!("  Binary:      {}", system.bin_path.display());
            println!("  GOROOT:      {}", system.path.display());
            println!("  Source:      {}", system.source.to_string().dimmed());
            
            // 检查 PATH 优先级
            match system_go::is_system_go_first_in_path() {
                Ok(true) => {
                    println!();
                    println!("{}", "  ⚠️  System Go has priority over GoVM in PATH".red());
                }
                Ok(false) => {
                    println!();
                    println!("{}", "  ✓ GoVM has priority in PATH".green());
                }
                _ => {}
            }

            // 检查是否已导入
            let versions = get_installed_versions()?;
            if versions.contains(&system.version) {
                println!();
                println!("{}", "  This version is managed by GoVM".green());
            } else {
                println!();
                println!("{}", "  Run 'govm import' to manage this version with GoVM".yellow());
            }
        }
        None => {
            println!("{}", "No system Go installation detected.".yellow());
            println!("Go does not appear to be in your PATH.");
        }
    }

    Ok(())
}

/// 列出已安装的版本
async fn list_installed_versions() -> Result<()> {
    // 检查 PATH 冲突
    show_path_conflict_warning();

    let versions = get_installed_versions()?;
    let current = get_current_version()?;
    let system_go = detect_system_go()?;

    // 显示系统安装的 Go
    if let Some(ref system) = system_go {
        println!("{}", "System Go installation:".bold().blue());
        println!("  Version:  {}", system.version.to_string().cyan());
        println!("  Location: {}", system.path.display().to_string().dimmed());
        println!("  Source:   {}", system.source.to_string().dimmed());
        println!();
        
        if !versions.iter().any(|v| v == &system.version) {
            println!("{} Run 'govm import' to manage this version.", "Tip:".yellow());
            println!();
        }
    }

    if versions.is_empty() {
        println!("{}", "No Go versions installed by GoVM.".yellow());
        println!("Run 'govm install <version>' to install a version,");
        println!("or 'govm import' to import the system Go installation.");
        return Ok(());
    }

    println!("{}", "Installed Go versions:".bold().green());
    println!();

    for version in &versions {
        let is_current = current.as_ref() == Some(version);
        let is_system = system_go.as_ref().map(|s| &s.version) == Some(version);
        
        let marker = if is_current {
            " * ".green().bold()
        } else if is_system {
            " = ".blue()
        } else {
            "   ".normal()
        };
        
        let version_str = format!("{:<12}", version.to_string());
        
        let mut statuses = Vec::new();
        if is_current {
            statuses.push("current".green());
        }
        if is_system {
            statuses.push("system".blue());
        }
        
        let status_str = if statuses.is_empty() {
            "".to_string()
        } else {
            format!("({})", statuses.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", "))
        };

        println!("{}{} {}", marker, version_str.cyan(), status_str);
    }

    println!();
    println!("{} indicates current version, {} indicates system version", 
        "*".green().bold(), "=".blue());

    Ok(())
}

/// 列出远程可用版本
async fn list_remote_versions() -> Result<()> {
    println!("{}", "Fetching remote versions...".blue());
    
    let client = Client::new();
    let versions = download::fetch_remote_versions(&client).await?;
    let installed = get_installed_versions()?;

    println!("\n{}", "Available Go versions:".bold().green());
    println!();

    // 只显示最新的 30 个版本
    for version in versions.iter().take(30) {
        let marker = if installed.contains(version) {
            " ✓ ".green()
        } else {
            "   ".normal()
        };
        
        let version_str = format!("{:<12}", version.to_string());
        let status = if installed.contains(version) {
            "(installed)".green()
        } else {
            "".normal()
        };

        println!("{}{} {}", marker, version_str.cyan(), status);
    }

    println!();
    println!("{} indicates installed version", "✓".green());

    Ok(())
}

/// 安装 Go 版本
async fn install_go_version(version_str: &str) -> Result<()> {
    let version = GoVersion::parse(version_str)
        .context("Invalid version format")?;

    // 检查是否已安装
    if is_version_installed(&version)? {
        println!("{}", format!("Go {} is already installed.", version).yellow());
        println!("Run 'govm use {}' to switch to this version.", version);
        return Ok(());
    }

    println!("{}", format!("Installing Go {}...", version).blue());

    // 创建 HTTP 客户端
    let client = Client::new();

    // 下载
    let cache_dir = get_cache_dir()?;
    let archive_path = download::download_go(&client, &version, &cache_dir).await?;

    // 安装
    install_version(&archive_path, &version)?;

    println!("\n{}", format!("Go {} installed successfully!", version).green().bold());
    println!("Run 'govm use {}' to switch to this version.", version);

    // 如果有系统 Go，提示用户
    if let Some(hint) = get_import_hint() {
        println!();
        println!("{}", hint.dimmed());
    }

    Ok(())
}

/// 卸载 Go 版本
async fn uninstall_go_version(version_str: &str) -> Result<()> {
    let version = GoVersion::parse(version_str)
        .context("Invalid version format")?;

    // 检查是否是当前版本
    if let Some(current) = get_current_version()? {
        if current == version {
            println!("{}", format!("Warning: Go {} is the current version.", version).yellow());
            print!("Are you sure you want to uninstall? [y/N] ");
            std::io::stdout().flush()?;
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            
            if !input.trim().eq_ignore_ascii_case("y") {
                println!("Uninstall cancelled.");
                return Ok(());
            }
        }
    }

    uninstall_version(&version)?;

    Ok(())
}

/// 切换 Go 版本
async fn use_go_version(version_str: &str) -> Result<()> {
    let version = GoVersion::parse(version_str)
        .context("Invalid version format")?;

    switch_version(&version)?;
    
    // 检查并提示 PATH 冲突
    show_path_conflict_warning();

    Ok(())
}

/// 显示当前版本
async fn show_current_version() -> Result<()> {
    show_path_conflict_warning();
    
    let current = get_current_version()?;

    match current {
        Some(version) => {
            println!("{}", format!("Current Go version: {}", version).green().bold());
            
            // 显示当前 Go 的路径
            if let Some(path) = get_current_go_path()? {
                println!("Location: {}", path.display());
            }
        }
        None => {
            println!("{}", "No Go version is currently active.".yellow());
            println!("Run 'govm use <version>' to activate a version.");
        }
    }

    Ok(())
}
