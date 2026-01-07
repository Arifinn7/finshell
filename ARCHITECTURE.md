# 🏗️ Finshell Architecture

Dokumen ini menjelaskan desain teknis dan aliran data di dalam Finshell.

## High-Level Overview

Finshell dibangun dengan arsitektur **Modular Event-Driven**. Berbeda dengan bar tradisional yang melakukan refresh UI secara keseluruhan, Finshell memecah setiap komponen menjadi modul independen yang berjalan di thread terpisah.

```mermaid
graph TD
    Main[Main Thread / GTK Loop]
    Config[Config Loader]
    
    subgraph Modules [Worker Threads]
        WS[Workspaces Worker]
        Net[Network Worker]
        Sys[System Info Worker]
    end
    
    Hyprland((Hyprland Socket))
    Kernel((Linux Kernel))
    
    Config --> Main
    Main -- Spawns --> Modules
    
    WS -- Listen --> Hyprland
    Net -- Polls --> Kernel
    Sys -- Polls --> Kernel
    
    Modules -- mpsc channel --> Main
    Main -- Updates --> UI[GTK Widgets]
Core Concepts
1. The WidgetModule Trait
Setiap fitur (Jam, Baterai, dll) harus mengimplementasikan trait WidgetModule. Ini adalah kontrak standar yang memastikan setiap modul bisa dipasang (plug-and-play).
code
Rust
pub trait WidgetModule {
    fn build_widget(&self) -> gtk4::Widget;
}
2. Threading Model & Communication
Untuk mencegah UI freeze (macet), Finshell melarang keras melakukan I/O (baca file/socket) di Main Thread.
Worker Thread: Melakukan pekerjaan berat (baca /proc, connect socket, exec command).
Communication: Menggunakan std::sync::mpsc::channel.
UI Update: Menggunakan glib::timeout_add_local untuk mengonsumsi pesan dari channel secara non-blocking.
3. Module Factory
Finshell menggunakan pola Factory di main.rs. String dari config.toml (misal: "clock") diterjemahkan secara dinamis menjadi inisialisasi struct modul terkait.
Directory Structure
src/main.rs: Entry point, inisialisasi window, dan layer shell setup.
src/config.rs: Parser untuk TOML configuration.
src/modules/: Logika bisnis per fitur.
mod.rs: Registry modul.
workspaces.rs: Socket client untuk Hyprland IPC.
network.rs: Wrapper untuk NetworkManager & Kernel wireless info.
... (modul lainnya)