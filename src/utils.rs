use anyhow::{Result, anyhow};
use dirs;
use std::env;
use std::path::PathBuf;

/// 获取 GoVM 的根目录
/// Windows: %LOCALAPPDATA%/govm
/// Unix: ~/.govm
pub fn get_govm_dir() -> Result<PathBuf> {
    let dir = if cfg!(windows) {
        dirs::data_local_dir()
            .ok_or_else(|| anyhow!("Failed to get local data directory"))?
            .join("govm")
    } else {
        dirs::home_dir()
            .ok_or_else(|| anyhow!("Failed to get home directory"))?
            .join(".govm")
    };

    Ok(dir)
}

/// 获取存放所有版本的目录
pub fn get_versions_dir() -> Result<PathBuf> {
    let dir = get_govm_dir()?.join("versions");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// 获取缓存目录
pub fn get_cache_dir() -> Result<PathBuf> {
    let dir = get_govm_dir()?.join("cache");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// 获取当前激活版本的目录
pub fn get_current_dir() -> Result<PathBuf> {
    Ok(get_govm_dir()?.join("current"))
}

/// 清理缓存目录
pub fn clean_cache() -> Result<()> {
    let cache_dir = get_cache_dir()?;
    
    if cache_dir.exists() {
        std::fs::remove_dir_all(&cache_dir)?;
        std::fs::create_dir_all(&cache_dir)?;
        println!("✅ Cache cleaned successfully");
    } else {
        println!("Cache directory does not exist, nothing to clean");
    }

    Ok(())
}

/// 获取系统的临时目录
pub fn get_temp_dir() -> Result<PathBuf> {
    let dir = env::temp_dir().join("govm");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// 格式化文件大小
pub fn format_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}
