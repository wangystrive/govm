# GoVM - Go Version Manager

一个用 Rust 编写�?Go 多版本管理工具，类似�?`nvm` �?`pyenv`�?

## 功能特�?

- 📦 **安装/卸载** - 轻松安装或卸载任�?Go 版本
- 🔄 **版本切换** - 快速在不同 Go 版本之间切换
- 📋 **版本列表** - 查看已安装和远程可用�?Go 版本
- 🎯 **当前版本** - 显示当前激活的 Go 版本
- 🔍 **系统兼容** - 检测并导入系统已安装的 Go
- ⚠️ **冲突检�?* - 自动检�?PATH 冲突并给出提�?
- 🌍 **全平台支�?* - 支持 Go 官方支持的所有平�?

## 安装

### 快速安装（推荐�?

#### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/wangystrive/govm/master/scripts/install.sh | bash
```

#### Windows (PowerShell)

```powershell
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/wangystrive/govm/master/scripts/install.ps1" -OutFile "install.ps1"; .\install.ps1
```

### 从源码编�?

```bash
# 克隆仓库
git clone https://github.com/wangystrive/govm.git
cd govm

# 编译发布版本
cargo build --release

# 安装（可选）
make install  # Linux/macOS
```

### 手动下载

�?[Releases](https://github.com/wangystrive/govm/releases) 页面下载对应平台的预编译二进制文件�?

#### Linux

```bash
# 下载（根据你的架构选择�?
wget https://github.com/wangystrive/govm/releases/latest/download/govm-x86_64-unknown-linux-gnu.tar.gz

# 解压
tar xzf govm-x86_64-unknown-linux-gnu.tar.gz

# 移动�?PATH 目录
sudo mv govm /usr/local/bin/
```

#### macOS

```bash
# Intel Mac
wget https://github.com/wangystrive/govm/releases/latest/download/govm-x86_64-apple-darwin.tar.gz
tar xzf govm-x86_64-apple-darwin.tar.gz
sudo mv govm /usr/local/bin/

# Apple Silicon Mac
wget https://github.com/wangystrive/govm/releases/latest/download/govm-aarch64-apple-darwin.tar.gz
tar xzf govm-aarch64-apple-darwin.tar.gz
sudo mv govm /usr/local/bin/
```

#### Windows

```powershell
# 下载
Invoke-WebRequest -Uri "https://github.com/wangystrive/govm/releases/latest/download/govm-x86_64-pc-windows-msvc.zip" -OutFile "govm.zip"

# 解压
Expand-Archive -Path "govm.zip" -DestinationPath "$env:LOCALAPPDATA\govm\bin"

# 添加�?PATH
[Environment]::SetEnvironmentVariable("Path", "$env:LOCALAPPDATA\govm\bin;$env:Path", "User")
```

## 使用方法

### 查看帮助

```bash
govm --help
```

### 列出已安装的版本

```bash
govm list
```

### 列出远程可用的版�?

```bash
govm list-remote
```

### 安装指定版本�?Go

```bash
# 安装 Go 1.21.5
govm install 1.21.5

# 或带前缀
govm install go1.21.5
```

### 切换到指定版�?

```bash
govm use 1.21.5
```

**注意�?* 切换版本后，需要确�?GoVM �?bin 目录�?PATH 中才能使用。程序会提示你添加以下路径：
- Windows: `%LOCALAPPDATA%\govm\current\bin`
- Linux/macOS: `$HOME/.govm/current/bin`

### 显示当前版本

```bash
govm current
```

### 卸载指定版本

```bash
govm uninstall 1.21.5
```

### 清理下载缓存

```bash
govm clean
```

## 环境变量配置

### Windows

将以下路径添加到系统 PATH 环境变量�?

```
%LOCALAPPDATA%\govm\current\bin
```

**手动设置步骤�?*
1. 右键"此电�? �?属�?�?高级系统设置
2. 环境变量 �?用户变量 �?Path �?编辑
3. 添加新条目：`%LOCALAPPDATA%\govm\current\bin`
4. 确保这个条目在系�?Go 路径之前（如果有系统 Go�?
5. 确定保存，重启终�?

### Linux/macOS

添加到你�?shell 配置文件（`.bashrc`, `.zshrc` 等）�?

```bash
export PATH="$HOME/.govm/current/bin:$PATH"
```

然后重新加载配置�?

```bash
source ~/.bashrc  # �?source ~/.zshrc
```

## 系统 Go 兼容

### 查看系统 Go 信息

```bash
govm system
```

输出示例�?
```
System Go Information

  Version:     1.25.6
  Binary:      C:\Program Files\Go\bin\go.exe
  GOROOT:      C:\Program Files\Go
  Source:      package manager

  ⚠️  System Go has priority over GoVM in PATH

  Run 'govm import' to manage this version with GoVM
```

### 导入系统已安装的 Go

如果你已经在系统中安装了 Go，可以将其导入到 GoVM 中管理：

```bash
govm import
```

这将�?
- 检测系统中�?Go 安装
- 将其复制或链接到 GoVM 的版本目�?
- 之后可以�?`govm use` 切换到这个版�?

### PATH 冲突检�?

GoVM 会自动检测系�?Go 是否�?PATH 中优先级高于 GoVM。如果是，会显示警告�?

```
⚠️  WARNING: System Go appears before GoVM in your PATH.
   GoVM's version switching will not take effect.

   To fix this, ensure GoVM's bin directory is BEFORE system Go in PATH:
   - Windows: %LOCALAPPDATA%\govm\current\bin
   - Unix: $HOME/.govm/current/bin
```

## 快速开�?

### 全新安装

```bash
# 1. 安装 govm（使用上面的安装命令�?

# 2. 安装 Go 版本
govm install 1.21.5

# 3. 切换到该版本
govm use 1.21.5

# 4. 配置环境变量（根据提示手动添加到 PATH�?
# Windows: 添加 %LOCALAPPDATA%\govm\current\bin �?PATH
# Linux/macOS: export PATH="$HOME/.govm/current/bin:$PATH"

# 5. 验证
go version
```

### 使用系统已有 Go

```bash
# 1. 导入系统 Go
govm import

# 2. 查看已导入的版本
govm list

# 3. 配置环境变量（确�?GoVM 路径在系�?Go 之前�?

# 4. 在版本间切换
govm use 1.25.6  # 系统版本
govm use 1.21.5  # 其他版本
```

## 开�?

### 构建

```bash
# 开发构�?
cargo build

# 发布构建
cargo build --release
```

### 运行测试

```bash
cargo test
```

### 交叉编译

使用 [cross](https://github.com/cross-rs/cross) 进行交叉编译�?

```bash
# 安装 cross
cargo install cross

# 构建所有目�?
make release-all

# 或单独构�?
cross build --release --target x86_64-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target x86_64-apple-darwin
```

### Makefile 命令

```bash
make build        # 构建当前平台
make test         # 运行测试
make clean        # 清理构建产物
make install      # 安装�?/usr/local/bin
make release-all  # 构建所有平�?
make setup        # 安装交叉编译目标
```

## 目录结构

```
$HOME/.govm/              # Unix 系统
%LOCALAPPDATA%/govm/      # Windows 系统
├── versions/             # 存放所有安装的 Go 版本
�?  ├── 1.20.0/
�?  ├── 1.21.0/
�?  └── 1.21.5/
├── current/              # 当前激活的 Go 版本（符号链�?复制�?
└── cache/                # 下载缓存
```

## 工作流示�?

### 日常开发工作流

```bash
# 查看可用�?Go 版本
govm list-remote

# 安装新版�?
govm install 1.21.5

# 切换版本
govm use 1.21.5

# 验证安装（确保环境变量已配置�?
go version

# 安装另一个版本用于测�?
govm install 1.20.0

# 切换回旧版本
govm use 1.20.0
```

## 与其他工具共�?

### 与官方安装包共存

如果你使用官方的 MSI (Windows) �?PKG (macOS) 安装�?Go�?
- 检测到这个安装
- 通过 `govm import` 将其纳入管理
- 或者在 `govm list` 中显示系统版�?

### 与包管理器共�?

对于通过 Homebrew、apt、yum 等安装的 Go�?
- GoVM 会识别安装来�?
- 你可以选择继续使用包管理器，或使用 GoVM 管理
- 建议卸载包管理器�?Go，完全使�?GoVM 管理

## 常见问题

### Q: 为什么切换版本后 `go` 命令没有变化�?

A: 你需要确�?GoVM �?bin 目录�?PATH 中优先级最高。检查：

```bash
# 查看 PATH 顺序
echo $PATH  # Unix
$env:PATH   # PowerShell
```

确保以下路径在系�?Go 路径之前�?
- Windows: `%LOCALAPPDATA%\govm\current\bin`
- Unix: `$HOME/.govm/current/bin`

### Q: 如何完全替换系统 Go�?

A: 
1. 导入现有 Go: `govm import`
2. 卸载系统 Go (通过包管理器或删除安装目�?
3. 添加 GoVM �?PATH
4. 使用 `govm use <version>` 切换版本

### Q: 支持哪些操作系统�?

A: GoVM 支持 Go 官方支持的所有平台：

| OS | 架构 |
|----|------|
| Linux | amd64, 386, arm64, armv6l |
| macOS | amd64 (Intel), arm64 (Apple Silicon) |
| Windows | amd64, 386, arm64 |
| FreeBSD | amd64 |

详见 [PLATFORMS.md](PLATFORMS.md) 获取完整列表和安装说明�?

### Q: 下载很慢怎么办？

A: GoVM 会从 Go 官方服务器下载，如果速度慢可以考虑使用代理。下载文件会缓存，多次安装同一版本不需要重新下载�?

## 发布

### 手动发布流程

1. 更新版本号：`Cargo.toml`
2. 创建标签：`git tag v0.1.0`
3. 推送标签：`git push origin v0.1.0`
4. GitHub Actions 会自动构建并创建 Release

### 触发手动构建

�?GitHub Actions 页面手动触发 `release` 工作流�?

## 贡献

欢迎提交 Issue �?Pull Request�?

## 许可�?

MIT License
