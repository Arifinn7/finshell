# Contributing to Finshell

Terima kasih atas minat Anda untuk berkontribusi! Dokumen ini akan memandu Anda menyiapkan lingkungan pengembangan.

## 🛠️ Development Setup

1. **Fork & Clone** repository ini.
2. **Install Dev Dependencies:**
   - Rust Toolchain (`rustup`)
   - GTK4 Development Libraries
     - Arch: `sudo pacman -S gtk4 gtk4-layer-shell base-devel`
     - Ubuntu/Debian: `sudo apt install libgtk-4-dev libgtk-4-layer-shell-dev`

3. **Run in Debug Mode:**
   ```bash
   RUST_LOG=debug cargo run

## 🧪 Coding Standards
Formatting: Kami menggunakan rustfmt. Jalankan cargo fmt sebelum commit.
Linting: Pastikan kode bersih dari warning. Jalankan cargo clippy.
Async/Threading: Jangan pernah memblokir main thread GTK. Gunakan std::thread::spawn untuk operasi I/O dan kirim data kembali via channel.

## 📝 Pull Request Process
Buat branch fitur baru (git checkout -b feature/AmazingFeature).
Commit perubahan Anda dengan pesan yang jelas (Conventional Commits direkomendasikan).
Contoh: feat(audio): add support for microphone mute
Push ke branch Anda.
Buka Pull Request dan deskripsikan perubahan yang Anda buat.

## 🐛 Reporting Bugs
Jika menemukan bug, silakan buka Issue dengan menyertakan:
Output terminal (cargo run).
Versi Hyprland dan OS yang digunakan.
Langkah-langkah untuk mereproduksi error.