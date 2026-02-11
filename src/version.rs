use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Go 版本信息
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GoVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl GoVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// 从版本字符串解析，支持格式: "1.21.5", "go1.21.5", "v1.21.5"
    pub fn parse(version_str: &str) -> Result<Self> {
        let cleaned = version_str
            .trim()
            .trim_start_matches('v')
            .trim_start_matches("go");

        let parts: Vec<&str> = cleaned.split('.').collect();
        if parts.len() != 3 {
            return Err(anyhow!(
                "Invalid version format: {}. Expected format: 1.21.5",
                version_str
            ));
        }

        let major = parts[0].parse::<u32>()
            .map_err(|_| anyhow!("Invalid major version: {}", parts[0]))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| anyhow!("Invalid minor version: {}", parts[1]))?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| anyhow!("Invalid patch version: {}", parts[2]))?;

        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    /// 获取下载用的版本字符串 (例如: go1.21.5)
    pub fn to_download_string(&self) -> String {
        format!("go{}.{}.{}", self.major, self.minor, self.patch)
    }

    /// 获取目录名用的版本字符串 (例如: 1.21.5)
    pub fn to_dir_name(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Display for GoVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for GoVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        let v = GoVersion::parse("1.21.5").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 21);
        assert_eq!(v.patch, 5);
    }

    #[test]
    fn test_parse_with_prefix() {
        let v1 = GoVersion::parse("go1.21.5").unwrap();
        let v2 = GoVersion::parse("v1.21.5").unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_version_ordering() {
        let v1 = GoVersion::new(1, 20, 0);
        let v2 = GoVersion::new(1, 21, 0);
        let v3 = GoVersion::new(1, 21, 5);
        
        assert!(v1 < v2);
        assert!(v2 < v3);
    }
}
