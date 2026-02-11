# GoVM 功能说明

## 系统 Go 兼容功能

GoVM 现在可以智能地处理系统中已安装的 Go：

### 1. 自动检测系统 Go

- 检测 PATH 中的 Go 安装
- 识别安装来源（官方安装包/包管理器）
- 显示 Go 版本和安装路径

### 2. 导入系统 Go

```bash
govm import
```

功能：
- 将系统 Go 复制到 GoVM 版本目录
- 之后可以用 `govm use` 切换
- 避免重复下载

### 3. PATH 冲突检测

自动检测并警告：
- 系统 Go 是否在 PATH 中优先级更高
- 提示用户如何修复

### 4. 增强的列表显示

```bash
govm list
```

输出包含：
- 系统 Go 信息（版本、位置、来源）
- 已安装的 Go 版本
- 标识当前版本 (`*`) 和系统版本 (`=`)

### 5. 详细的系统信息

```bash
govm system
```

显示：
- Go 版本
- 二进制文件位置
- GOROOT
- 安装来源
- PATH 优先级状态
- 是否已导入到 GoVM

## 新增的命令

| 命令 | 说明 |
|------|------|
| `govm import` | 导入系统已安装的 Go |
| `govm system` | 显示系统 Go 详细信息 |

## 改进的命令

| 命令 | 改进 |
|------|------|
| `govm list` | 显示系统 Go 和导入状态 |
| `govm current` | 显示 PATH 冲突警告 |
| `govm use` | 显示 PATH 冲突警告 |
| `govm install` | 提示可以导入系统 Go |

## 典型使用场景

### 场景 1：已有 Go，想使用 GoVM

```bash
# 检测系统 Go
govm system

# 导入系统 Go
govm import

# 开始使用
govm list
govm use 1.25.6
```

### 场景 2：PATH 冲突解决

```bash
# 发现警告
govm list
# ⚠️ WARNING: System Go appears before GoVM in PATH

# Windows 修复：调整 PATH 顺序，确保 %LOCALAPPDATA%\govm\current\bin 在最前面
# Unix 修复：export PATH="$HOME/.govm/current/bin:$PATH"
```

### 场景 3：多版本管理

```bash
# 导入现有版本
govm import

# 安装新版本
govm install 1.21.5

# 在版本间切换
govm use 1.25.6  # 系统版本
govm use 1.21.5  # GoVM 安装的版本
```
