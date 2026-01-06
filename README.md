# 🦀 Rust Shell for Hyprland

A custom, high-performance desktop shell built from scratch using **Rust**, **GTK4**, and **Layer Shell**. Designed for Hyprland on Arch Linux.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Built%20with-Rust-orange)

## ✨ Features

- **Workspaces**: Real-time Hyprland workspace indicator.
- **Clock & Dashboard**: Clickable clock revealing a control center (Volume, Brightness, Profile).
- **System Info**: RAM, Volume, and Battery status (compatible with VM/Desktop).
- **Media Player**: Spotify/MPRIS integration with Play/Pause controls.
- **App Launcher**: Built-in fast app launcher with search.
- **Power Menu**: Fullscreen overlay for Shutdown, Reboot, Lock, etc.
- **OSD**: On-Screen Display for volume/brightness changes.
- **Theming**: Fully customizable via CSS (`~/.config/rust_shell/style.css`).

## 📦 Dependencies

Before installing, ensure you have the required system libraries.

**Arch Linux:**
```bash
sudo pacman -S rustup gtk4 gtk4-layer-shell libdbus brightnessctl wireplumber ttf-jetbrains-mono-nerd
```

## 🚀 Installation

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/USERNAME/rust_shell.git
    cd rust_shell
    ```

2.  **Run the installer:**
    ```bash
    ./install.sh
    ```

3.  **Add to Hyprland Config:**
    Add this line to your `~/.config/hypr/hyprland.conf`:
    ```ini
    exec-once = ~/.local/bin/rust_shell
    ```

## 🎨 Customization

Edit the CSS file to change colors, fonts, and borders:
```bash
nano ~/.config/rust_shell/style.css
```
Restart the shell to apply changes.

## 🤝 Contributing

Feel free to open issues or pull requests!
