use anyhow::{Result, anyhow};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::version::GoVersion;

/// 系统安装的 Go 信息
#[derive(Debug, Clone)]
pub struct SystemGo {
    pub version: GoVersion,
    pub path: PathBuf,
    pub bin_path: PathBuf,
    pub source: GoSource,
}

/// Go 安装来源
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoSource {
    /// 从官网下载的发行版
    Official,
    /// 包管理器安装 (brew, apt, yum 等)
    PackageManager,
    /// 未知来源
    Unknown,
}

impl std::fmt::Display for GoSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoSource::Official => write!(f, "official"),
            GoSource::PackageManager => write!(f, "package manager"),
            GoSource::Unknown => write!(f, "unknown"),
        }
    }
}

/// 检测系统中是否已安装 Go
pub fn detect_system_go() -> Result<Option<SystemGo>> {
    // 尝试执行 go version
    let output = Command::new("go")
        .arg("version")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version_output = String::from_utf8_lossy(&output.stdout);
            parse_go_version_output(&version_output)
        }
        _ => Ok(None),
    }
}

/// 解析 go version 的输出
fn parse_go_version_output(output: &str) -> Result<Option<SystemGo>> {
    // 输出格式: go version go1.21.5 linux/amd64
    let parts: Vec<&str> = output.trim().split_whitespace().collect();
    
    if parts.len() < 3 || parts[0] != "go" || parts[1] != "version" {
        return Ok(None);
    }

    let version_str = parts[2]; // e.g., "go1.21.5"
    let version = match GoVersion::parse(version_str) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    // 获取 go 可执行文件的路径
    let bin_path = get_go_binary_path()?;
    let path = bin_path.parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| bin_path.clone());

    // 检测来源
    let source = detect_go_source(&path);

    Ok(Some(SystemGo {
        version,
        path,
        bin_path,
        source,
    }))
}

/// 获取 go 可执行文件的完整路径
fn get_go_binary_path() -> Result<PathBuf> {
    #[cfg(windows)]
    let go_exe = "go.exe";
    #[cfg(not(windows))]
    let go_exe = "go";

    // 尝试使用 `which` (Unix) 或 `where` (Windows)
    #[cfg(windows)]
    let which_cmd = "where";
    #[cfg(not(windows))]
    let which_cmd = "which";

    let output = Command::new(which_cmd)
        .arg(go_exe)
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let path_str = String::from_utf8_lossy(&output.stdout);
            let path = path_str.trim().lines().next().unwrap_or("");
            if !path.is_empty() {
                return Ok(PathBuf::from(path));
            }
        }
        _ => {}
    }

    // 备选：检查 PATH 环境变量
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let go_path = path.join(go_exe);
            if go_path.exists() {
                return Ok(go_path);
            }
        }
    }

    Err(anyhow!("Could not find go binary in PATH"))
}

/// 检测 Go 安装的来源
fn detect_go_source(go_root: &Path) -> GoSource {
    let path_str = go_root.to_string_lossy().to_lowercase();

    // 检测常见的包管理器安装路径
    #[cfg(target_os = "macos")]
    if path_str.contains("homebrew") || path_str.contains("opt/go") || path_str.contains("cellar") {
        return GoSource::PackageManager;
    }

    #[cfg(target_os = "linux")]
    if path_str.contains("/usr/lib/go") || 
       path_str.contains("/usr/local/go") && path_str != "/usr/local/go" {
        return GoSource::PackageManager;
    }

    #[cfg(windows)]
    if path_str.contains("\\program files\\go") || 
       path_str.contains("\\program files (x86)\\go") {
        return GoSource::PackageManager;
    }

    // 如果路径是标准的 /usr/local/go 或 C:\Go，可能是官方安装包
    if path_str == "/usr/local/go" || 
       path_str == "c:\\go" ||
       path_str.ends_with("\\go") && !path_str.contains("govm") {
        return GoSource::Official;
    }

    GoSource::Unknown
}

/// 检查系统 Go 是否在 PATH 中优先级高于 GoVM
pub fn is_system_go_first_in_path() -> Result<bool> {
    let system_go = match detect_system_go()? {
        Some(go) => go,
        None => return Ok(false),
    };

    // 检查 GoVM 的 current 目录
    let govm_current = crate::utils::get_current_dir()?;
    let govm_bin = govm_current.join("bin");

    // 获取 PATH
    let path_var = env::var("PATH").unwrap_or_default();
    
    #[cfg(windows)]
    let separator = ';';
    #[cfg(not(windows))]
    let separator = ':';

    let paths: Vec<&str> = path_var.split(separator).collect();

    let mut found_system = false;
    let mut found_govm = false;

    for path in paths {
        let path_buf = PathBuf::from(path.trim());
        
        if !found_system && path_buf == system_go.bin_path.parent().unwrap_or(&path_buf) {
            found_system = true;
        }
        
        if !found_govm && path_buf == govm_bin {
            found_govm = true;
        }

        // 如果都找到了，检查谁先出现
        if found_system && !found_govm {
            return Ok(true);
        }
        if found_govm && !found_system {
            return Ok(false);
        }
    }

    // 都没找到或只找到一个
    Ok(found_system && !found_govm)
}

/// 导入系统安装的 Go 到 GoVM
pub fn import_system_go() -> Result<PathBuf> {
    let system_go = detect_system_go()?.ok_or_else(|| {
        anyhow!("No system Go installation found")
    })?;

    println!("Found system Go:", );
    println!("  Version: {}", system_go.version);
    println!("  Location: {}", system_go.path.display());
    println!("  Source: {}", system_go.source);

    // 检查是否已经在 GoVM 中
    let versions_dir = crate::utils::get_versions_dir()?;
    let target_dir = versions_dir.join(system_go.version.to_dir_name());

    if target_dir.exists() {
        return Err(anyhow!(
            "Go {} is already managed by GoVM at {}",
            system_go.version,
            target_dir.display()
        ));
    }

    // 创建符号链接或复制文件
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(&system_go.path, &target_dir)?;
        println!("✅ Imported Go {} as symlink", system_go.version);
    }

    #[cfg(windows)]
    {
        // Windows 上创建目录连接需要管理员权限，这里改为复制
        println!("Copying files from {} to {}...", 
            system_go.path.display(), 
            target_dir.display()
        );
        copy_dir_all(&system_go.path, &target_dir)?;
        println!("✅ Imported Go {} (copied)", system_go.version);
    }

    Ok(target_dir)
}

/// 递归复制目录
#[cfg(windows)]
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    use walkdir::WalkDir;
    
    std::fs::create_dir_all(&dst)?;
    
    for entry in WalkDir::new(&src) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(&src)?;
        let dest_path = dst.as_ref().join(relative);

        if path.is_dir() {
            std::fs::create_dir_all(&dest_path)?;
        } else {
            std::fs::copy(path, dest_path)?;
        }
    }
    
    Ok(())
}

/// 获取导入 Go 的提示信息
pub fn get_import_hint() -> Option<String> {
    match detect_system_go() {
        Ok(Some(go)) => {
            Some(format!(
                "System Go {} detected at {}. Run 'govm import' to manage it with GoVM.",
                go.version,
                go.path.display()
            ))
        }
        _ => None,
    }
}

/// 显示 PATH 冲突警告
pub fn show_path_conflict_warning() {
    if let Ok(true) = is_system_go_first_in_path() {
        eprintln!("⚠️  WARNING: System Go appears before GoVM in your PATH.");
        eprintln!("   GoVM's version switching will not take effect.");
        eprintln!();
        eprintln!("   To fix this, ensure GoVM's bin directory is BEFORE system Go in PATH:");
        eprintln!("   - Windows: %LOCALAPPDATA%\\govm\\current\\bin");
        eprintln!("   - Unix: $HOME/.govm/current/bin");
        eprintln!();
    }
}
