use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation, GestureClick};
use super::WidgetModule;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use mpris::{PlayerFinder, PlaybackStatus};

// Data yang dikirim dari Thread ke UI
struct MediaInfo {
    title: String,
    artist: String,
    status: String, // "Playing", "Paused", "Stopped"
    has_player: bool,
}

pub struct MprisModule;

impl MprisModule {
    // Fungsi Berat: Mencari Player dan Metadata
    fn fetch_media_info() -> MediaInfo {
        // PERBAIKAN DI SINI:
        // Kita coba buat finder. Kalau gagal konek DBus, langsung return kosong.
        let finder = match PlayerFinder::new() {
            Ok(f) => f,
            Err(_) => return MediaInfo {
                title: String::new(),
                artist: String::new(),
                status: "Stopped".to_string(),
                has_player: false,
            },
        };
        
        // Cari player yang aktif (Spotify, Firefox, dll)
        // finder sekarang sudah berupa "PlayerFinder" asli, bukan "Result" lagi.
        if let Ok(player) = finder.find_active() {
            // Ambil Metadata
            let title = player.get_metadata()
                .ok()
                .and_then(|m| m.title().map(|t| t.to_string()))
                .unwrap_or("Unknown Title".to_string());

            let artist = player.get_metadata()
                .ok()
                .and_then(|m| m.artists().map(|a| a.join(", ")))
                .unwrap_or("Unknown Artist".to_string());

            let status = match player.get_playback_status() {
                Ok(PlaybackStatus::Playing) => "Playing",
                Ok(PlaybackStatus::Paused) => "Paused",
                _ => "Stopped",
            };

            return MediaInfo {
                title,
                artist,
                status: status.to_string(),
                has_player: true,
            };
        }

        // Kalau tidak ada player aktif
        MediaInfo {
            title: String::new(),
            artist: String::new(),
            status: "Stopped".to_string(),
            has_player: false,
        }
    }

    fn update_view(label: &Label, container: &Box, info: MediaInfo) {
        if !info.has_player || info.status == "Stopped" {
            container.set_visible(false);
            return;
        }

        container.set_visible(true);
        container.remove_css_class("paused");
        
        let icon = if info.status == "Playing" { 
            "" 
        } else { 
            container.add_css_class("paused");
            "" 
        };

        // Truncate text (batasi panjang judul)
        let mut display_text = format!("{} - {}", info.artist, info.title);
        if display_text.len() > 40 {
            display_text = format!("{}...", &display_text[0..37]);
        }

        label.set_text(&format!("{}  {}", icon, display_text));
    }

    // Fungsi Kontrol: Play/Pause
    fn toggle_play_pause() {
        // PERBAIKAN DI SINI JUGA:
        // Cek dulu apakah Finder berhasil dibuat
        if let Ok(finder) = PlayerFinder::new() {
            if let Ok(player) = finder.find_active() {
                let _ = player.play_pause();
            }
        }
    }
}

impl WidgetModule for MprisModule {
    fn build_widget(&self) -> gtk4::Widget {
        let container = Box::new(Orientation::Horizontal, 5);
        container.add_css_class("mpris-widget");
        container.set_visible(false);

        let label = Label::new(None);
        container.append(&label);

        // --- INTERAKSI KLIK ---
        let click = GestureClick::new();
        click.connect_pressed(move |_, _, _, _| {
            thread::spawn(|| {
                Self::toggle_play_pause();
            });
        });
        container.add_controller(click);

        // --- THREADING LOGIC ---
        let (sender, receiver) = mpsc::channel();
        let container_weak = container.downgrade();

        // Worker Thread
        thread::spawn(move || {
            loop {
                let info = Self::fetch_media_info();
                let _ = sender.send(info);
                thread::sleep(Duration::from_secs(1));
            }
        });

        // UI Update
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