#!/bin/bash
set -e

# Xác định thư mục gốc của dự án
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Kiểm tra công cụ biên dịch Rust
if ! command -v cargo &> /dev/null; then
    echo "Lỗi: Không tìm thấy công cụ Cargo. Vui lòng cài đặt Rust trước khi tiếp tục." >&2
    exit 1
fi

# Biên dịch bộ cài đặt TUI nếu chưa tồn tại bản phát hành
if [ ! -f "target/release/babydra-installer" ]; then
    echo "Đang biên dịch công cụ cài đặt BabyDra TUI Installer (cargo build --release)..."
    cargo build --release -p babydra-installer
fi

# Khởi chạy giao diện cài đặt TUI
exec ./target/release/babydra-installer "$@"
