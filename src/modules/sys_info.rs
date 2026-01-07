use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};
use super::WidgetModule;
use std::fs;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

struct SysInfo {
    cpu_usage: u8, // 0-100%
    ram_usage: u8, // 0-100%
    ram_used_gb: f32, // Misal 4.5 GB
}

pub struct SysInfoModule;

impl SysInfoModule {
    // --- LOGIKA CPU ---
    // Membaca /proc/stat untuk mendapatkan total waktu CPU
    fn read_cpu_stats() -> (u64, u64) {
        if let Ok(content) = fs::read_to_string("/proc/stat") {
            if let Some(first_line) = content.lines().next() {
                // Format: cpu  user nice system idle iowait ...
                let parts: Vec<&str> = first_line.split_whitespace().collect();
                if parts.len() > 4 {
                    let user: u64 = parts[1].parse().unwrap_or(0);
                    let nice: u64 = parts[2].parse().unwrap_or(0);
                    let system: u64 = parts[3].parse().unwrap_or(0);
                    let idle: u64 = parts[4].parse().unwrap_or(0);
                    let iowait: u64 = parts[5].parse().unwrap_or(0);
                    
                    // Total waktu aktif = user + nice + system + iowait
                    // Total waktu = aktif + idle
                    let active = user + nice + system + iowait;
                    let total = active + idle;
                    return (active, total);
                }
            }
        }
        (0, 0)
    }

    // --- LOGIKA RAM ---
    // Membaca /proc/meminfo
    fn read_ram_stats() -> (u8, f32) {
        let mut total = 0.0;
        let mut available = 0.0;

        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 2 { continue; }
                
                let key = parts[0];
                let value: f32 = parts[1].parse().unwrap_or(0.0);

                if key == "MemTotal:" {
                    total = value;
                } else if key == "MemAvailable:" {
                    available = value;
                }

                // Kalau sudah ketemu dua-duanya, stop loop
                if total > 0.0 && available > 0.0 {
                    break;
                }
            }
        }

        if total > 0.0 {
            let used = total - available;
            let percent = (used / total) * 100.0;
            let used_gb = used / 1024.0 / 1024.0; // kB -> GB
            return (percent as u8, used_gb);
        }
        (0, 0.0)
    }

    fn update_view(label_cpu: &Label, label_ram: &Label, container: &Box, info: SysInfo) {
        // CPU
        label_cpu.set_text(&format!(" {}%", info.cpu_usage));
        
        // RAM
        // Tampilkan persentase dan GB
        label_ram.set_text(&format!("  {:.1}GB", info.ram_used_gb));

        // Styling Warning (Jika CPU tinggi > 80%)
        container.remove_css_class("high-load");
        if info.cpu_usage > 80 || info.ram_usage > 90 {
            container.add_css_class("high-load");
        }
    }
}

impl WidgetModule for SysInfoModule {
    fn build_widget(&self) -> gtk4::Widget {
        let container = Box::new(Orientation::Horizontal, 10); // Jarak antar elemen 10px
        container.add_css_class("sys-info-widget");

        // Label CPU
        let label_cpu = Label::new(None);
        label_cpu.add_css_class("cpu-label");
        
        // Label RAM
        let label_ram = Label::new(None);
        label_ram.add_css_class("ram-label");

        container.append(&label_cpu);
        container.append(&label_ram);

        let (sender, receiver) = mpsc::channel();
        let container_weak = container.downgrade();

        // Worker Thread
        thread::spawn(move || {
            let mut prev_cpu = Self::read_cpu_stats();
            
            loop {
                thread::sleep(Duration::from_secs(2)); // Update tiap 2 detik

                // Hitung Delta CPU
                let curr_cpu = Self::read_cpu_stats();
                let delta_active = curr_cpu.0.saturating_sub(prev_cpu.0);
                let delta_total = curr_cpu.1.saturating_sub(prev_cpu.1);
                
                let cpu_percent = if delta_total > 0 {
                    (delta_active as f32 / delta_total as f32 * 100.0) as u8
                } else {
                    0
                };
                prev_cpu = curr_cpu;

                // Hitung RAM
                let (ram_percent, ram_gb) = Self::read_ram_stats();

                let info = SysInfo {
                    cpu_usage: cpu_percent,
                    ram_usage: ram_percent,
                    ram_used_gb: ram_gb,
                };

                let _ = sender.send(info);
            }
        });

        // UI Update
        glib::timeout_add_local(Duration::from_millis(100), move || {
            if let Some(container) = container_weak.upgrade() {
                if let Ok(info) = receiver.try_recv() {
                    Self::update_view(&label_cpu, &label_ram, &container, info);
                }
            }
            glib::ControlFlow::Continue
        });

        container.upcast()
    }
}