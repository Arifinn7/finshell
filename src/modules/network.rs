use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation, GestureClick};
use super::WidgetModule;
use std::process::Command;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::fs;

struct NetworkInfo {
    connected: bool,
    is_wifi: bool,
    ssid: String,
    signal_strength: u8,
}

pub struct NetworkModule;

impl NetworkModule {
    // Fungsi khusus membaca sinyal langsung dari kernel Linux
    // Ini jauh lebih cepat & akurat daripada spawn nmcli
    fn get_wifi_signal() -> u8 {
        if let Ok(content) = fs::read_to_string("/proc/net/wireless") {
            for line in content.lines() {
                // Format: wlan0: 0000  70.  -40. ...
                // Kolom ke-3 biasanya adalah Link Quality (biasanya max 70 atau 100)
                if line.contains(":") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 3 {
                        // Ambil angka, buang titik di belakangnya (misal "70.")
                        let raw_signal = parts[2].trim_matches('.');
                        let signal_val = raw_signal.parse::<f32>().unwrap_or(0.0);
                        
                        // Konversi ke persen (asumsi max 70 di file wireless standar)
                        let percent = if signal_val > 70.0 {
                            signal_val // Kalau formatnya sudah persen (0-100)
                        } else {
                            (signal_val / 70.0) * 100.0 // Konversi 0-70 ke 0-100
                        };
                        return percent as u8;
                    }
                }
            }
        }
        0
    }

    fn fetch_network_info() -> NetworkInfo {
        // STRATEGI BARU: Tanya "Connection Active", bukan "Device"
        // Output contoh: 
        // 802-11-wireless:Infinix 1:wlan0
        // 802-3-ethernet:Wired connection 1:enp3s0
        let output = Command::new("nmcli")
            .env("LC_ALL", "C")
            .args(["-t", "-f", "TYPE,NAME", "connection", "show", "--active"])
            .output();

        if let Ok(out) = output {
            let string_out = String::from_utf8_lossy(&out.stdout);

            for line in string_out.lines() {
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() < 2 { continue; }

                let conn_type = parts[0];
                let conn_name = parts[1]; // INI PASTI NAMA SSID (Infinix 1)

                // Cek WiFi
                if conn_type == "802-11-wireless" || conn_type == "wifi" {
                    return NetworkInfo {
                        connected: true,
                        is_wifi: true,
                        ssid: conn_name.to_string(),
                        signal_strength: Self::get_wifi_signal(), // Ambil dari kernel
                    };
                }
                
                // Cek Ethernet
                if conn_type == "802-3-ethernet" || conn_type == "ethernet" {
                    return NetworkInfo {
                        connected: true,
                        is_wifi: false,
                        ssid: conn_name.to_string(),
                        signal_strength: 0,
                    };
                }
            }
        }

        // --- FALLBACK (Jika nmcli gagal total) ---
        // Kembali ke cara cek folder /sys/class/net
        if let Ok(entries) = fs::read_dir("/sys/class/net") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name == "lo" || name.starts_with("docker") || name.starts_with("veth") { continue; }

                let operstate_path = entry.path().join("operstate");
                if let Ok(state) = fs::read_to_string(operstate_path) {
                    if state.trim() == "up" {
                        let is_wifi = name.starts_with("w") || name.starts_with("wl");
                        return NetworkInfo {
                            connected: true,
                            is_wifi,
                            ssid: name, // Terpaksa pakai nama interface (wlan0)
                            signal_strength: if is_wifi { Self::get_wifi_signal() } else { 0 },
                        };
                    }
                }
            }
        }

        NetworkInfo {
            connected: false,
            is_wifi: false,
            ssid: "Offline".to_string(),
            signal_strength: 0,
        }
    }

    fn update_view(label: &Label, container: &Box, info: NetworkInfo) {
        container.remove_css_class("disconnected");
        container.remove_css_class("wifi");
        container.remove_css_class("ethernet");

        if !info.connected {
            label.set_text("󰤮 Offline");
            container.add_css_class("disconnected");
            return;
        }

        if info.is_wifi {
            container.add_css_class("wifi");
            
            let icon = match info.signal_strength {
                80..=100 => "󰤨",
                60..=79  => "󰤥",
                40..=59  => "󰤢",
                20..=39  => "󰤟",
                _        => "󰤯",
            };
            
            // Truncate SSID
            let mut ssid = info.ssid.clone();
            if ssid.len() > 15 {
                ssid = format!("{}...", &ssid[0..12]);
            }

            label.set_text(&format!("{} {}", icon, ssid));
        } else {
            container.add_css_class("ethernet");
            label.set_text("󰈀 Wired");
        }
    }

    fn open_manager() {
        let _ = Command::new("nm-connection-editor").spawn().or_else(|_| {
             Command::new("kitty").arg("-e").arg("nmtui").spawn()
        });
    }
}

impl WidgetModule for NetworkModule {
    fn build_widget(&self) -> gtk4::Widget {
        let container = Box::new(Orientation::Horizontal, 5);
        container.add_css_class("network-widget");

        let label = Label::new(None);
        container.append(&label);

        let click = GestureClick::new();
        click.connect_pressed(|_, _, _, _| { Self::open_manager(); });
        container.add_controller(click);

        let (sender, receiver) = mpsc::channel();
        let container_weak = container.downgrade();

        thread::spawn(move || {
            loop {
                let info = Self::fetch_network_info();
                let _ = sender.send(info);
                thread::sleep(Duration::from_secs(5));
            }
        });

        glib::timeout_add_local(Duration::from_millis(100), move || {
            if let Some(container) = container_weak.upgrade() {
                if let Ok(info) = receiver.try_recv() {
                    Self::update_view(&label, &container, info);
                }
            }
            glib::ControlFlow::Continue
        });

        container.upcast()
    }
}