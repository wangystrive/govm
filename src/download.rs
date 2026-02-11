use anyhow::{Result, anyhow, Context};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::version::GoVersion;

const GO_DOWNLOAD_URL: &str = "https://go.dev/dl";

/// 获取当前系统的 Go 下载文件名
pub fn get_go_archive_name(version: &GoVersion) -> Result<String> {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    // 映射 arch 名称
    let go_arch = match arch {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        "x86" => "386",
        _ => arch,
    };

    let ext = match os {
        "windows" => "zip",
        _ => "tar.gz",
    };

    // 映射 os 名称
    let go_os = match os {
        "macos" => "darwin",
        _ => os,
    };

    Ok(format!(
        "go{}.{}.{}.{}-{}.{}",
        version.major,
        version.minor,
        version.patch,
        go_os,
        go_arch,
        ext
    ))
}

/// 下载指定版本的 Go
pub async fn download_go(
    client: &Client,
    version: &GoVersion,
    dest_dir: &Path,
) -> Result<std::path::PathBuf> {
    let archive_name = get_go_archive_name(version)?;
    let download_url = format!("{}/{}", GO_DOWNLOAD_URL, archive_name);
    let dest_path = dest_dir.join(&archive_name);

    // 如果文件已存在，先删除
    if dest_path.exists() {
        tokio::fs::remove_file(&dest_path).await?;
    }

    println!("Downloading from: {}", download_url);

    // 创建进度条
    let pb = ProgressBar::new(0);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );

    // 发送请求
    let response = client
        .get(&download_url)
        .timeout(Duration::from_secs(300))
        .send()
        .await
        .context("Failed to download Go archive")?;

    // 检查状态码
    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to download: HTTP {} - {}",
            response.status(),
            download_url
        ));
    }

    // 设置进度条总大小
    if let Some(content_length) = response.content_length() {
        pb.set_length(content_length);
    }

    // 创建目标文件
    let mut file = File::create(&dest_path).await
        .context("Failed to create destination file")?;

    // 分块下载
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.context("Failed to download chunk")?;
        file.write_all(&chunk).await
            .context("Failed to write to file")?;
        pb.inc(chunk.len() as u64);
    }

    file.flush().await?;
    pb.finish_with_message("Download complete");

    println!("Downloaded to: {}", dest_path.display());
    Ok(dest_path)
}

/// 获取可用的 Go 版本列表
pub async fn fetch_remote_versions(client: &Client) -> Result<Vec<GoVersion>> {
    // Go 的下载页面 JSON 数据
    let url = "https://go.dev/dl/?mode=json&include=all";
    
    #[derive(Debug, serde::Deserialize)]
    struct GoRelease {
        version: String,
    }

    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to fetch remote versions")?;

    if !response.status().is_success() {
        return Err(anyhow!("Failed to fetch versions: HTTP {}", response.status()));
    }

    let releases: Vec<GoRelease> = response.json().await
        .context("Failed to parse versions response")?;

    let mut versions: Vec<GoVersion> = releases
        .into_iter()
        .filter_map(|r| GoVersion::parse(&r.version).ok())
        .collect();

    // 按版本号排序（从大到小）
    versions.sort_by(|a, b| b.cmp(a));

    Ok(versions)
}
