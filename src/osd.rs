use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Label, LevelBar, Orientation
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use async_channel::unbounded;

enum OsdEvent {
    VolumeChanged(f64, bool),
    BrightnessChanged(f64),
}

pub fn create_osd_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OSD")
        .default_width(200)
        .default_height(60)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Bottom, true);
    window.set_margin(Edge::Bottom, 150);
    window.set_widget_name("osd-window");

    let container = Box::new(Orientation::Horizontal, 15);
    container.set_widget_name("osd-box");

    let icon_label = Label::new(Some(""));
    icon_label.set_widget_name("osd-icon");
    
    let level_bar = LevelBar::new();
    level_bar.set_min_value(0.0);
    level_bar.set_max_value(1.0);
    level_bar.set_hexpand(true);
    level_bar.set_widget_name("osd-bar");

    container.append(&icon_label);
    container.append(&level_bar);

    window.set_child(Some(&container));
    window.set_visible(false);

    let (sender, receiver) = unbounded();

    // 1. MONITOR THREAD
    thread::spawn(move || {
        let mut last_vol = match get_volume_data() {
            Some((v, _)) => v,
            None => -1.0, // Set -1 agar saat start terdeteksi perubahannya (untuk init UI)
        };
        let mut last_bri = get_brightness_data().unwrap_or(0.0);

        loop {
            if let Some((vol, muted)) = get_volume_data() {
                if (vol - last_vol).abs() > 0.001 {
                    let _ = sender.send_blocking(OsdEvent::VolumeChanged(vol, muted));
                    last_vol = vol;
                }
            }

            if let Some(bri) = get_brightness_data() {
                if (bri - last_bri).abs() > 0.001 {
                    let _ = sender.send_blocking(OsdEvent::BrightnessChanged(bri));
                    last_bri = bri;
                }
            }
            thread::sleep(Duration::from_millis(150));
        }
    });

    // 2. UI UPDATER (FIXED LOGIC)
    let hide_timer = Arc::new(Mutex::new(None::<glib::SourceId>));
    
    glib::MainContext::default().spawn_local(async move {
        while let Ok(event) = receiver.recv().await {
            window.set_visible(true);
            
            match event {
                OsdEvent::VolumeChanged(val, muted) => {
                    level_bar.set_value(val);
                    if muted {
                        icon_label.set_text("");
                        level_bar.add_css_class("muted");
                    } else {
                        icon_label.set_text("");
                        level_bar.remove_css_class("muted");
                    }
                }
                OsdEvent::BrightnessChanged(val) => {
                    level_bar.set_value(val);
                    icon_label.set_text("");
                    level_bar.remove_css_class("muted");
                }
            }

            // --- PERBAIKAN LOGIKA TIMER DI SINI ---
            let mut timer_guard = hide_timer.lock().unwrap();
            
            // 1. Hapus timer lama jika ada (Debounce)
            if let Some(source_id) = timer_guard.take() {
                // Jangan gunakan unwrap(), abaikan error jika timer sudah mati
                let _ = source_id.remove(); 
            }

            let win_clone = window.clone();
            // Kita butuh akses ke guard di dalam closure timer juga
            let guard_clone = hide_timer.clone();

            // 2. Buat timer baru
            let new_timer = glib::timeout_add_local(Duration::from_secs(1), move || {
                win_clone.set_visible(false);
                
                // PENTING: Timer selesai secara alami. 
                // Kita harus set guard jadi None agar event berikutnya tidak mencoba remove ID mati ini.
                let mut g = guard_clone.lock().unwrap();
                *g = None;

                glib::ControlFlow::Break
            });
            
            *timer_guard = Some(new_timer);
        }
    });
}

fn get_volume_data() -> Option<(f64, bool)> {
    let output = Command::new("wpctl")
        .arg("get-volume")
        .arg("@DEFAULT_AUDIO_SINK@")
        .output()
        .ok()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let muted = stdout.contains("MUTED");
    let clean_text = stdout.replace("Volume:", "").replace("[MUTED]", "");
    
    if let Ok(val) = clean_text.trim().parse::<f64>() {
        return Some((val, muted));
    }
    
    // Fallback parsing
    for token in stdout.split_whitespace() {
        if let Ok(val) = token.parse::<f64>() {
            return Some((val, muted));
        }
    }
    None
}

fn get_brightness_data() -> Option<f64> {
    let current_out = Command::new("brightnessctl").arg("get").output().ok()?;
    let max_out = Command::new("brightnessctl").arg("max").output().ok()?;
    
    let current = String::from_utf8_lossy(&current_out.stdout).trim().parse::<f64>().ok()?;
    let max = String::from_utf8_lossy(&max_out.stdout).trim().parse::<f64>().ok()?;
    
    if max == 0.0 { return None; }
    Some(current / max)
}