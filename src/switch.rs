use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};

use crate::version::GoVersion;
use crate::utils::{get_versions_dir, get_current_dir};

/// 切换 Go 版本
/// 在 Windows 上通过创建批处理文件实现
/// 在 Unix 系统上通过创建符号链接实现
pub fn switch_version(version: &GoVersion) -> Result<()> {
    let versions_dir = get_versions_dir()?;
    let version_dir = versions_dir.join(version.to_dir_name());

    if !version_dir.exists() {
        return Err(anyhow!(
            "Go {} is not installed. Run 'govm install {}' first.",
            version,
            version
        ));
    }

    let go_bin_dir = version_dir.join("bin");
    
    if !go_bin_dir.exists() {
        return Err(anyhow!(
            "Invalid Go installation: bin directory not found at {}",
            go_bin_dir.display()
        ));
    }

    // 创建/更新 current 目录
    let current_dir = get_current_dir()?;
    
    // 确保 current 目录存在
    if current_dir.exists() {
        fs::remove_dir_all(&current_dir)?;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(&version_dir, &current_dir)?;
    }

    #[cfg(windows)]
    {
        // Windows 使用目录链接
        copy_dir_all(&version_dir, &current_dir)?;
    }

    // 创建激活脚本
    create_activation_script(&go_bin_dir)?;

    println!("✅ Switched to Go {}", version);
    println!("\nNote: Add the following to your PATH to use this version:");
    
    #[cfg(windows)]
    {
        println!("  {}", go_bin_dir.display());
        println!("\nOr add to your shell profile:");
        println!("  Windows: %LOCALAPPDATA%\\govm\\current\\bin");
    }
    
    #[cfg(unix)]
    {
        println!("  {}", go_bin_dir.display());
        println!("\nOr add to your shell profile:");
        println!("  export PATH=\"$HOME/.govm/current/bin:$PATH\"");
    }

    Ok(())
}

/// 获取当前激活的 Go 版本
pub fn get_current_version() -> Result<Option<GoVersion>> {
    let current_dir = get_current_dir()?;
    
    if !current_dir.exists() {
        return Ok(None);
    }

    // 检查 current 目录指向的版本
    #[cfg(unix)]
    let go_bin = current_dir.join("bin").join("go");
    
    #[cfg(windows)]
    let go_bin = current_dir.join("bin").join("go.exe");

    if !go_bin.exists() {
        return Ok(None);
    }

    // 尝试执行 go version
    let output = std::process::Command::new(&go_bin)
        .arg("version")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            // 解析 "go version go1.21.5 linux/amd64" 这样的输出
            if let Some(version_part) = version_str.split_whitespace().nth(2) {
                if let Ok(version) = GoVersion::parse(version_part) {
                    return Ok(Some(version));
                }
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

/// 创建激活脚本
fn create_activation_script(go_bin_dir: &Path) -> Result<()> {
    let current_dir = get_current_dir()?;
    
    #[cfg(unix)]
    {
        // 创建 shell 脚本
        let script_path = current_dir.join("activate.sh");
        let script = format!(
            r#"#!/bin/bash
# GoVM activation script
export PATH="{}:$PATH"
echo "Go environment activated. Go binary: $(which go)"
go version
"#,
            go_bin_dir.display()
        );
        fs::write(&script_path, script)?;
        
        // 设置可执行权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms)?;
        }
    }

    #[cfg(windows)]
    {
        // 创建批处理文件
        let script_path = current_dir.join("activate.bat");
        let script = format!(
            r#"@echo off
REM GoVM activation script
set "PATH={};%PATH%"
echo Go environment activated.
go version
"#,
            go_bin_dir.display()
        );
        fs::write(&script_path, script)?;

        // 创建 PowerShell 脚本
        let ps_script_path = current_dir.join("activate.ps1");
        let ps_script = format!(
            r#"# GoVM activation script
$env:PATH = "{};" + $env:PATH
Write-Host "Go environment activated." -ForegroundColor Green
& go version
"#,
            go_bin_dir.display()
        );
        fs::write(&ps_script_path, ps_script)?;
    }

    Ok(())
}

/// 打印环境变量设置命令
pub fn print_env_commands(version: &GoVersion) -> Result<()> {
    let versions_dir = get_versions_dir()?;
    let version_dir = versions_dir.join(version.to_dir_name());
    let go_bin_dir = version_dir.join("bin");

    if !go_bin_dir.exists() {
        return Err(anyhow!("Go {} is not installed", version));
    }

    #[cfg(unix)]
    {
        println!("export PATH=\"{}:$PATH\"", go_bin_dir.display());
    }

    #[cfg(windows)]
    {
        println!("$env:PATH = \"{};$env:PATH\"", go_bin_dir.display());
    }

    Ok(())
}

/// 获取当前 Go 的 bin 路径
pub fn get_current_go_path() -> Result<Option<PathBuf>> {
    let current_dir = get_current_dir()?;
    let go_bin = current_dir.join("bin");
    
    if go_bin.exists() {
        Ok(Some(go_bin))
    } else {
        Ok(None)
    }
}

/// 复制目录（用于 Windows，因为 Windows 的符号链接需要管理员权限）
#[cfg(windows)]
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    
    for entry in walkdir::WalkDir::new(&src) {
        let entry = entry?;
        let path = entry.path();
        let relative = path.strip_prefix(&src)?;
        let dest_path = dst.as_ref().join(relative);

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            fs::copy(path, dest_path)?;
        }
    }
    
    Ok(())
}
