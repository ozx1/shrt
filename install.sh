#!/bin/bash
set -e

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$ARCH" = "x86_64" ]; then
    ARCH="x86_64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH="arm64"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi

# Map OS names
case "$OS" in
    linux)
        PLATFORM="linux"
        BINARY="shrt-linux-x86_64"
        ;;
    darwin)
        PLATFORM="macos"
        if [ "$ARCH" = "arm64" ]; then
            BINARY="shrt-macos-arm64"
        else
            BINARY="shrt-macos-x86_64"
        fi
        ;;
    *)
        echo "Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "Installing shrt for $PLATFORM ($ARCH)..."

# Download the latest release
LATEST_RELEASE=$(curl -s https://api.github.com/repos/ozx1/shrt/releases/latest | grep "tag_name" | cut -d '"' -f 4)
DOWNLOAD_URL="https://github.com/ozx1/shrt/releases/download/$LATEST_RELEASE/$BINARY"

echo "Downloading from $DOWNLOAD_URL..."
curl -L -o /tmp/shrt "$DOWNLOAD_URL"

# Make executable
chmod +x /tmp/shrt

# Install to /usr/local/bin
echo "Installing to /usr/local/bin (may require sudo)..."
sudo mv /tmp/shrt /usr/local/bin/shrt

echo "✓ shrt installed successfully!"
echo "Run 'shrt help' to get started"