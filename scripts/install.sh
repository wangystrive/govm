#!/bin/bash
# GoVM Installation Script
# Usage: curl -fsSL https://raw.githubusercontent.com/wangystrive/govm/master/scripts/install.sh | bash

set -e

# Configuration
REPO="wangystrive/govm"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
GOVM_BIN="${INSTALL_DIR}/govm"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect OS and architecture, return Go-style platform name
detect_platform() {
    local os
    local arch
    
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)
    
    case "$os" in
        linux)
            case "$arch" in
                x86_64)
                    PLATFORM="linux-amd64"
                    ;;
                i386|i686)
                    PLATFORM="linux-386"
                    ;;
                aarch64|arm64)
                    PLATFORM="linux-arm64"
                    ;;
                armv7l|armv6l)
                    PLATFORM="linux-armv6l"
                    ;;
                *)
                    echo -e "${RED}Unsupported architecture: $arch${NC}"
                    exit 1
                    ;;
            esac
            ;;
        darwin)
            case "$arch" in
                x86_64)
                    PLATFORM="darwin-amd64"
                    ;;
                arm64)
                    PLATFORM="darwin-arm64"
                    ;;
                *)
                    echo -e "${RED}Unsupported architecture: $arch${NC}"
                    exit 1
                    ;;
            esac
            ;;
        freebsd)
            case "$arch" in
                x86_64)
                    PLATFORM="freebsd-amd64"
                    ;;
                *)
                    echo -e "${RED}Unsupported architecture: $arch${NC}"
                    exit 1
                    ;;
            esac
            ;;
        *)
            echo -e "${RED}Unsupported OS: $os${NC}"
            exit 1
            ;;
    esac
    
    echo -e "${BLUE}Detected platform: $PLATFORM${NC}"
}

# Get latest version
get_latest_version() {
    echo -e "${BLUE}Checking latest version...${NC}"
    
    if command -v curl &> /dev/null; then
        LATEST=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | sed -n 's/.*"tag_name": "\([^"]*\)".*/\1/p')
    elif command -v wget &> /dev/null; then
        LATEST=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | sed -n 's/.*"tag_name": "\([^"]*\)".*/\1/p')
    else
        echo -e "${RED}curl or wget is required${NC}"
        exit 1
    fi
    
    if [ -z "$LATEST" ]; then
        echo -e "${RED}Failed to get latest version${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}Latest version: $LATEST${NC}"
}

# Download and install
download_and_install() {
    local version="$1"
    local platform="$2"
    local temp_dir
    local download_url
    
    temp_dir=$(mktemp -d)
    download_url="https://github.com/${REPO}/releases/download/${version}/govm-${platform}.tar.gz"
    
    echo -e "${BLUE}Downloading from: $download_url${NC}"
    
    if command -v curl &> /dev/null; then
        curl -fsSL "$download_url" -o "${temp_dir}/govm.tar.gz"
    else
        wget -q "$download_url" -O "${temp_dir}/govm.tar.gz"
    fi
    
    if [ ! -f "${temp_dir}/govm.tar.gz" ]; then
        echo -e "${RED}Download failed${NC}"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    echo -e "${BLUE}Extracting...${NC}"
    tar xzf "${temp_dir}/govm.tar.gz" -C "$temp_dir"
    
    echo -e "${BLUE}Installing to $INSTALL_DIR...${NC}"
    if [ -w "$INSTALL_DIR" ]; then
        mv "${temp_dir}/govm" "$GOVM_BIN"
    else
        echo -e "${YELLOW}sudo access required to install to $INSTALL_DIR${NC}"
        sudo mv "${temp_dir}/govm" "$GOVM_BIN"
    fi
    
    chmod +x "$GOVM_BIN"
    rm -rf "$temp_dir"
    
    echo -e "${GREEN}govm installed successfully!${NC}"
}

# Verify installation
verify_installation() {
    if command -v govm &> /dev/null; then
        echo -e "${GREEN}govm is now available:${NC}"
        govm --version
    elif [ -f "$GOVM_BIN" ]; then
        echo -e "${YELLOW}govm is installed at $GOVM_BIN${NC}"
        echo -e "${YELLOW}Please ensure $INSTALL_DIR is in your PATH${NC}"
        "$GOVM_BIN" --version
    else
        echo -e "${RED}Installation verification failed${NC}"
        exit 1
    fi
}

# Print post-installation message
print_post_install() {
    echo ""
    echo -e "${GREEN}GoVM installation complete!${NC}"
    echo ""
    echo -e "${BLUE}Quick start:${NC}"
    echo "  govm list-remote     # List available Go versions"
    echo "  govm install 1.21.5  # Install Go 1.21.5"
    echo "  govm use 1.21.5      # Switch to Go 1.21.5"
    echo ""
    echo -e "${BLUE}Add to your shell profile:${NC}"
    
    case "$SHELL" in
        */zsh)
            echo "  echo 'export PATH=\"\$HOME/.govm/current/bin:\$PATH\"' >> ~/.zshrc"
            ;;
        */bash)
            echo "  echo 'export PATH=\"\$HOME/.govm/current/bin:\$PATH\"' >> ~/.bashrc"
            ;;
        *)
            echo "  export PATH=\"\$HOME/.govm/current/bin:\$PATH\""
            ;;
    esac
    
    echo ""
    echo -e "${BLUE}For more information:${NC}"
    echo "  govm --help"
    echo "  https://github.com/${REPO}"
}

# Main
main() {
    echo -e "${BLUE}GoVM Installer${NC}"
    echo ""
    
    detect_platform
    get_latest_version
    download_and_install "$LATEST" "$PLATFORM"
    verify_installation
    print_post_install
}

# Run main function
main "$@"
