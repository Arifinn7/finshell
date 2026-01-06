use gtk::prelude::*;
use gtk::{Box, Label, Orientation};
// PERBAIKAN: Import MemoryRefreshKind
use sysinfo::{MemoryRefreshKind, RefreshKind, System};
use std::process::Command;
use glib::ControlFlow;
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_system_widget() -> Box {
    let container = Box::new(Orientation::Horizontal, 10);
    container.set_widget_name("system-container");

    // 1. Widget Volume
    let vol_label = Label::new(Some("Vol: --%"));
    vol_label.add_css_class("sys-label");

    // 2. Widget RAM
    let ram_label = Label::new(Some("RAM: --%"));
    ram_label.add_css_class("sys-label");

    // 3. Widget Baterai
    let bat_label = Label::new(Some("BAT: --%"));
    bat_label.add_css_class("sys-label");
    bat_label.set_widget_name("battery-label"); 

    container.append(&vol_label);
    container.append(&ram_label);
    container.append(&bat_label);

    // PERBAIKAN INISIALISASI SYSINFO:
    // sysinfo 0.30+ butuh argumen MemoryRefreshKind::everything()
    // Kita HAPUS .with_components() karena bikin error dan berat.
    let sys = System::new_with_specifics(
        RefreshKind::new()
            .with_memory(MemoryRefreshKind::everything()),
    );
    
    let sys_state = Rc::new(RefCell::new(sys));

    // Loop Timer (2 Detik)
    let v_clone = vol_label.clone();
    let r_clone = ram_label.clone();
    let b_clone = bat_label.clone();
    
    glib::timeout_add_seconds_local(2, move || {
        // A. Update Volume
        let vol = get_volume();
        v_clone.set_label(&format!(" {}", vol));

        // B. Update RAM
        let mut sys = sys_state.borrow_mut();
        sys.refresh_memory(); // Refresh hanya memori
        
        let used = sys.used_memory();
        let total = sys.total_memory();
        // Cegah pembagian dengan nol (safety)
        let ram_pct = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        r_clone.set_label(&format!(" {:.0}%", ram_pct));

        // C. Update Battery (Via Linux Filesystem)
        // Kita tidak lagi pakai sysinfo untuk baterai agar lebih stabil
        let (bat_icon, bat_text, css_class) = get_battery_info_linux();
        b_clone.set_label(&format!("{} {}", bat_icon, bat_text));
        
        // Update styling
        b_clone.remove_css_class("charging");
        b_clone.remove_css_class("low");
        if !css_class.is_empty() {
            b_clone.add_css_class(&css_class);
        }

        ControlFlow::Continue
    });

    container
}

// Helper Volume (wpctl)
fn get_volume() -> String {
    let output = Command::new("wpctl")
        .arg("get-volume")
        .arg("@DEFAULT_AUDIO_SINK@")
        .output();

    if let Ok(o) = output {
        let stdout = String::from_utf8_lossy(&o.stdout);
        if stdout.contains("MUTED") { return "Muted".to_string(); }
        
        for token in stdout.split_whitespace() {
            if let Ok(val) = token.parse::<f64>() {
                return format!("{:.0}%", val * 100.0);
            }
        }
    }
    "--%".to_string()
}

// Helper Battery via Linux File System
// Ini metode standar di Arch Linux (upower/acpi baca dari sini juga)
fn get_battery_info_linux() -> (String, String, String) {
    // Cari interface baterai (BAT0 atau BAT1)
    let paths = ["/sys/class/power_supply/BAT0", "/sys/class/power_supply/BAT1"];
    
    for path in paths {
        let path_cap = format!("{}/capacity", path);
        let path_stat = format!("{}/status", path);

        // Jika file capacity ada, berarti baterai terdeteksi
        if std::path::Path::new(&path_cap).exists() {
            // Baca Kapasitas
            let cap_str = std::fs::read_to_string(&path_cap).unwrap_or("0".to_string());
            let capacity: i32 = cap_str.trim().parse().unwrap_or(0);
            
            // Baca Status (Charging, Discharging, Full)
            let status = std::fs::read_to_string(&path_stat).unwrap_or("Unknown".to_string());
            let is_charging = status.trim() == "Charging";

            // Tentukan Ikon
            let icon = if is_charging {
                ""
            } else {
                match capacity {
                    90..=100 => "",
                    60..=89  => "",
                    40..=59  => "",
                    20..=39  => "",
                    _        => "",
                }
            };

            // Tentukan CSS Class
            let css = if is_charging {
                "charging".to_string()
            } else if capacity < 20 {
                "low".to_string()
            } else {
                "".to_string()
            };

            return (icon.to_string(), format!("{}%", capacity), css);
        }
    }

    // Jika tidak ada folder BAT0/BAT1 (PC Desktop / VM)
    // Tampilkan ikon Plug
    ("".to_string(), "AC".to_string(), "".to_string())
}