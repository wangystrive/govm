# GoVM Installation Script for Windows
# Usage: Invoke-WebRequest -Uri "https://raw.githubusercontent.com/wangystrive/govm/master/scripts/install.ps1" -OutFile "install.ps1"; .\install.ps1

param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:LOCALAPPDATA\govm\bin"
)

$ErrorActionPreference = "Stop"

# Configuration
$Repo = "wangystrive/govm"
$GovmExe = Join-Path $InstallDir "govm.exe"

# Colors
$Green = "`e[32m"
$Red = "`e[31m"
$Yellow = "`e[33m"
$Blue = "`e[34m"
$Reset = "`e[0m"

function Write-Info($Message) {
    Write-Host "$Blue$Message$Reset"
}

function Write-Success($Message) {
    Write-Host "$Green$Message$Reset"
}

function Write-Error($Message) {
    Write-Host "$Red$Message$Reset"
}

function Write-Warning($Message) {
    Write-Host "$Yellow$Message$Reset"
}

# Detect platform
function Get-Platform {
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture
    
    switch ($arch) {
        "X64" { return "x86_64-pc-windows-msvc" }
        "X86" { return "i686-pc-windows-msvc" }
        "Arm64" { return "aarch64-pc-windows-msvc" }
        default {
            Write-Error "Unsupported architecture: $arch"
            exit 1
        }
    }
}

# Get latest version
function Get-LatestVersion {
    Write-Info "Checking latest version..."
    
    try {
        $release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
        $version = $release.tag_name
        Write-Success "Latest version: $version"
        return $version
    }
    catch {
        Write-Error "Failed to get latest version: $_"
        exit 1
    }
}

# Download and install
function Install-Govm {
    param(
        [string]$Version,
        [string]$Platform
    )
    
    $downloadUrl = "https://github.com/$Repo/releases/download/$Version/govm-$Platform.zip"
    $tempFile = [System.IO.Path]::GetTempFileName() + ".zip"
    $tempDir = [System.IO.Path]::GetTempPath() + [System.Guid]::NewGuid().ToString()
    
    Write-Info "Downloading from: $downloadUrl"
    
    try {
        Invoke-WebRequest -Uri $downloadUrl -OutFile $tempFile -UseBasicParsing
    }
    catch {
        Write-Error "Download failed: $_"
        exit 1
    }
    
    Write-Info "Extracting..."
    Expand-Archive -Path $tempFile -DestinationPath $tempDir -Force
    
    Write-Info "Installing to $InstallDir..."
    if (!(Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }
    
    $sourceExe = Join-Path $tempDir "govm.exe"
    Copy-Item -Path $sourceExe -Destination $GovmExe -Force
    
    # Cleanup
    Remove-Item -Path $tempFile -Force -ErrorAction SilentlyContinue
    Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
    
    Write-Success "govm installed successfully!"
}

# Add to PATH
function Add-ToPath {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    if ($currentPath -notlike "*$InstallDir*") {
        Write-Info "Adding $InstallDir to PATH..."
        [Environment]::SetEnvironmentVariable(
            "Path",
            "$InstallDir;$currentPath",
            "User"
        )
        Write-Success "Added to PATH"
    }
    else {
        Write-Warning "Already in PATH"
    }
}

# Verify installation
function Test-Installation {
    if (Test-Path $GovmExe) {
        Write-Success "govm is installed at $GovmExe"
        & $GovmExe --version
    }
    else {
        Write-Error "Installation verification failed"
        exit 1
    }
}

# Print post-installation message
function Show-PostInstall {
    Write-Host ""
    Write-Success "GoVM installation complete!"
    Write-Host ""
    Write-Info "Quick start:"
    Write-Host "  govm list-remote     # List available Go versions"
    Write-Host "  govm install 1.21.5  # Install Go 1.21.5"
    Write-Host "  govm use 1.21.5      # Switch to Go 1.21.5"
    Write-Host ""
    Write-Info "PATH configuration:"
    Write-Host "  $InstallDir has been added to your user PATH"
    Write-Host "  Please restart your terminal to use govm"
    Write-Host ""
    Write-Info "For more information:"
    Write-Host "  govm --help"
    Write-Host "  https://github.com/$Repo"
}

# Main
function Main {
    Write-Info "GoVM Installer for Windows"
    Write-Host ""
    
    $platform = Get-Platform
    Write-Info "Detected platform: $platform"
    
    if ($Version -eq "latest") {
        $Version = Get-LatestVersion
    }
    
    Install-Govm -Version $Version -Platform $platform
    Add-ToPath
    Test-Installation
    Show-PostInstall
}

# Run main
Main
