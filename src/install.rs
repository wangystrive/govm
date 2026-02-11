use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use tar::Archive;
use flate2::read::GzDecoder;
use walkdir::WalkDir;

use crate::version::GoVersion;
use crate::utils::get_versions_dir;

/// 解压 .tar.gz 文件
fn extract_tar_gz(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = fs::File::open(archive_path)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);

    // 解压到临时目录
    let temp_dir = tempfile::tempdir()?;
    archive.unpack(temp_dir.path())?;

    // 移动 go 目录内容到目标目录
    let go_dir = temp_dir.path().join("go");
    if go_dir.exists() {
        copy_dir_all(&go_dir, dest_dir)?;
    }

    Ok(())
}

/// 解压 .zip 文件 (Windows)
#[cfg(target_os = "windows")]
fn extract_zip(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    use zip::read::ZipArchive;

    let file = fs::File::open(archive_path)?;
    let mut archive = ZipArchive::new(file)?;

    let temp_dir = tempfile::tempdir()?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = temp_dir.path().join(file.name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    // 移动 go 目录内容到目标目录
    let go_dir = temp_dir.path().join("go");
    if go_dir.exists() {
        copy_dir_all(&go_dir, dest_dir)?;
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn extract_zip(_archive_path: &Path, _dest_dir: &Path) -> Result<()> {
    Err(anyhow!("ZIP extraction is only supported on Windows"))
}

/// 递归复制目录
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    
    for entry in WalkDir::new(&src) {
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

/// 安装 Go 版本
pub fn install_version(archive_path: &Path, version: &GoVersion) -> Result<PathBuf> {
    let versions_dir = get_versions_dir()?;
    let version_dir = versions_dir.join(version.to_dir_name());

    println!("Installing Go {} to: {}", version, version_dir.display());

    // 如果目录已存在，先删除
    if version_dir.exists() {
        fs::remove_dir_all(&version_dir)?;
    }

    fs::create_dir_all(&version_dir)?;

    // 根据文件扩展名选择解压方式
    let extension = archive_path.extension().and_then(|s| s.to_str());
    
    match extension {
        Some("gz") => extract_tar_gz(archive_path, &version_dir)?,
        Some("zip") => extract_zip(archive_path, &version_dir)?,
        _ => {
            // 尝试根据文件名判断
            let file_name = archive_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if file_name.ends_with(".tar.gz") {
                extract_tar_gz(archive_path, &version_dir)?;
            } else if file_name.ends_with(".zip") {
                extract_zip(archive_path, &version_dir)?;
            } else {
                return Err(anyhow!("Unknown archive format: {}", archive_path.display()));
            }
        }
    }

    println!("✅ Go {} installed successfully", version);
    Ok(version_dir)
}

/// 卸载 Go 版本
pub fn uninstall_version(version: &GoVersion) -> Result<()> {
    let versions_dir = get_versions_dir()?;
    let version_dir = versions_dir.join(version.to_dir_name());

    if !version_dir.exists() {
        return Err(anyhow!("Go {} is not installed", version));
    }

    fs::remove_dir_all(&version_dir)?;
    println!("✅ Go {} uninstalled successfully", version);

    Ok(())
}

/// 检查指定版本是否已安装
pub fn is_version_installed(version: &GoVersion) -> Result<bool> {
    let versions_dir = get_versions_dir()?;
    let version_dir = versions_dir.join(version.to_dir_name());
    Ok(version_dir.exists())
}

/// 获取已安装的版本列表
pub fn get_installed_versions() -> Result<Vec<GoVersion>> {
    let versions_dir = get_versions_dir()?;
    
    if !versions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut versions = Vec::new();

    for entry in fs::read_dir(&versions_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            let dir_name = path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            
            if let Ok(version) = GoVersion::parse(dir_name) {
                versions.push(version);
            }
        }
    }

    // 按版本号排序（从大到小）
    versions.sort_by(|a, b| b.cmp(a));

    Ok(versions)
}
