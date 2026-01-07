use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};
use super::WidgetModule;
use std::fs;
use std::path::Path;

pub struct BatteryModule;

impl BatteryModule {
    // Fungsi untuk membaca persentase (0-100)
    fn get_percentage() -> i32 {
        // Coba baca BAT0 (umumnya ini), kalau gagal coba BAT1
        let paths = ["/sys/class/power_supply/BAT0/capacity", "/sys/class/power_supply/BAT1/capacity"];
        
        for path in paths {
            if Path::new(path).exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    // Hapus spasi/enter, lalu ubah ke angka
                    return content.trim().parse().unwrap_or(0);
                }
            }
        }
        -1 // Kode bahwa tidak ada baterai (Desktop PC)
    }

    // Fungsi untuk cek status charging
    fn get_status() -> String {
        let paths = ["/sys/class/power_supply/BAT0/status", "/sys/class/power_supply/BAT1/status"];
        
        for path in paths {
            if Path::new(path).exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    return content.trim().to_string();
                }
            }
        }
        "Unknown".to_string()
    }

    // Menentukan Ikon berdasarkan persen & status
    fn get_icon(percentage: i32, status: &str) -> &'static str {
        if status == "Charging" {
            return "⚡";
        }
        
        match percentage {
            90..=100 => "", // Ikon baterai penuh (Nerd Font)
            60..=89  => "",
            40..=59  => "",
            20..=39  => "",
            0..=19   => "", // Baterai kritis
            _        => "", // Desktop / AC Power
        }
    }

    fn update_view(label: &Label, container: &Box) {
        let percentage = Self::get_percentage();
        
        // Hapus semua class CSS dulu (reset state)
        container.remove_css_class("charging");
        container.remove_css_class("critical");
        container.remove_css_class("low");

        if percentage < 0 {
            // Mode Desktop (Tanpa Baterai)
            label.set_text(" AC");
            return;
        }

        let status = Self::get_status();
        let icon = Self::get_icon(percentage, &status);

        // Update Teks: " 98%"
        label.set_text(&format!("{} {}%", icon, percentage));

        // Logic pewarnaan CSS
        if status == "Charging" {
            container.add_css_class("charging");
        } else if percentage <= 15 {
            container.add_css_class("critical");
        } else if percentage <= 30 {
            container.add_css_class("low");
        }
    }
}

impl WidgetModule for BatteryModule {
    fn build_widget(&self) -> gtk4::Widget {
        let container = Box::new(Orientation::Horizontal, 5);
        container.add_css_class("battery-widget");

        let label = Label::new(None);
        container.append(&label);

        let label_clone = label.clone();
        let container_clone = container.clone();

        // Render awal
        Self::update_view(&label, &container);

        // Update setiap 5 detik (Baterai tidak perlu dicek tiap milidetik)
        glib::timeout_add_seconds_local(5, move || {
            Self::update_view(&label_clone, &container_clone);
            glib::ControlFlow::Continue
        });

        container.upcast()
    }
}