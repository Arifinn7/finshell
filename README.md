# 🐚 Finshell

> A blazing fast, fully customizable, and aesthetic Wayland bar/panel for Hyprland, built with **Rust** and **GTK4**.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/built_with-Rust-red.svg)
![GTK4](https://img.shields.io/badge/toolkit-GTK4-green.svg)
![Status](https://img.shields.io/badge/status-Active_Development-orange.svg)

**Finshell** bukanlah bar biasa. Ini adalah panel modern yang dirancang dengan filosofi:
1.  **Performance First:** Menggunakan native IPC dan threading untuk latensi nol.
2.  **Event Driven:** Tidak ada polling berat. Bar bereaksi instan terhadap perubahan sistem.
3.  **Configurable:** Konfigurasi via TOML dan styling via CSS standar.

---

# ✨ Features

- **🚀 Workspaces:** Socket-based IPC (Hyprland v0.53+ support). Instant switching tanpa lag.
- **🎵 MPRIS Player:** Integrasi media player (Spotify/Firefox) dengan play/pause control.
- **🔊 Audio Control:** Scroll untuk volume, klik untuk mute/unmute (via PipeWire/WirePlumber).
- **🔋 Battery Smart:** Deteksi otomatis Laptop/Desktop. Indikator warna dinamis.
- **📶 Network Manager:** Deteksi WiFi/Ethernet real-time dengan sinyal meter akurat.
- **📊 System Monitor:** CPU & RAM usage monitor (direct kernel reading).
- **⏱️ Clock:** Waktu dan Tanggal presisi.
- **⏻ Power Menu:** Akses cepat untuk shutdown/reboot.
- **🎨 Theming:** Full CSS styling support (Rounded corners, blur, transparency).

---

# 📦 Installation

### Prerequisites
Pastikan sistem Anda memiliki dependensi berikut:
- **Rust** (latest stable)
- **GTK4** (`gtk4`, `gtk4-layer-shell`)
- **Hyprland** (Running session)
- **Font:** Nerd Fonts (rekomendasi: JetBrainsMono Nerd Font)

### Build from Source

## 1. Clone repository
```bash
git clone https://github.com/Arifinn7/finshell
cd finshell
```

## 2. Build Release Binary
```bash
cargo build --release
```

## 3. Install (Copy to user bin)
```bash
cp target/release/finshell ~/.local/bin/
```

# ⚙️ Configuration
## 1. Finshell mencari konfigurasi di ~/.config/finshell/config.toml.
Basic Configuration (config.toml)

```bash
[bar]
position = "top"      # Options: top, bottom
height = 45           # Pixel height

[modules]
# Available: "workspaces", "clock", "battery", "audio", 
#            "mpris", "network", "sys_info", "power"

left = ["workspaces", "mpris"]
center = ["clock"]
right = ["sys_info", "network", "audio", "battery", "power"]
```

# 2. Styling (style.css)
Anda bisa mengubah tampilan sepenuhnya di style.css. Contoh:

```bash
.main-bar {
    background-color: rgba(30, 30, 46, 0.8);
    border-radius: 12px;
    margin: 5px;
}

.workspace-button.active {
    background-color: #cba6f7;
    color: #1e1e2e;
}
```

# 🛠️ Troubleshooting

Q: Bar tidak muncul / Error Socket not found?
A: Pastikan Anda menjalankan finshell di dalam sesi Hyprland. Finshell membutuhkan variabel environment HYPRLAND_INSTANCE_SIGNATURE yang disediakan otomatis oleh Hyprland.

Q: Network status "Offline" padahal connect?
A: Finshell menggunakan nmcli. Pastikan networkmanager terinstall dan berjalan.

Q: Audio widget tidak merespon?
A: Pastikan wireplumber atau pipewire-pulse berjalan. Coba jalankan wpctl status di terminal.


# 🤝 Contributing

Kontribusi sangat diterima! Silakan baca CONTRIBUTING.md untuk panduan pengembangan.


# 📄 License
Project ini dilisensikan di bawah MIT License.