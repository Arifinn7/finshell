#!/bin/bash

# Warna untuk output terminal
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}=========================================${NC}"
echo -e "${BLUE}    RUST SHELL FOR HYPRLAND INSTALLER    ${NC}"
echo -e "${BLUE}=========================================${NC}"

# 1. Cek apakah Cargo terinstall
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}[ERROR] Rust/Cargo tidak ditemukan!${NC}"
    echo "Silakan install rustup terlebih dahulu: https://rustup.rs/"
    exit 1
fi

# 2. Build Project
echo -e "${GREEN}[1/4] Compiling project (Release Mode)...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}[ERROR] Build gagal! Cek error log di atas.${NC}"
    exit 1
fi

# 3. Setup Direktori Config
CONFIG_DIR="$HOME/.config/rust_shell"
echo -e "${GREEN}[2/4] Menyiapkan konfigurasi di $CONFIG_DIR...${NC}"

if [ ! -d "$CONFIG_DIR" ]; then
    mkdir -p "$CONFIG_DIR"
    echo "Folder config dibuat."
fi

# Backup config lama jika ada
if [ -f "$CONFIG_DIR/style.css" ]; then
    mv "$CONFIG_DIR/style.css" "$CONFIG_DIR/style.css.bak"
    echo "Config lama di-backup ke style.css.bak"
fi

cp config/style.css "$CONFIG_DIR/style.css"

# 4. Install Binary
BIN_DIR="$HOME/.local/bin"
echo -e "${GREEN}[3/4] Menginstall binary ke $BIN_DIR...${NC}"

if [ ! -d "$BIN_DIR" ]; then
    mkdir -p "$BIN_DIR"
fi

cp target/release/rust_shell "$BIN_DIR/"

echo -e "${GREEN}[4/4] Selesai!${NC}"
echo -e "${BLUE}=========================================${NC}"
echo "Tambahkan baris ini di ~/.config/hypr/hyprland.conf:"
echo -e "${GREEN}exec-once = $BIN_DIR/rust_shell${NC}"
echo ""
echo "Jalankan sekarang dengan mengetik: rust_shell"
