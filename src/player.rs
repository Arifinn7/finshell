use gtk::prelude::*;
use gtk::{Box, Button, Label, Orientation};
use mpris::{PlayerFinder, PlaybackStatus};
use glib::ControlFlow;

pub fn create_player_widget() -> Box {
    let container = Box::new(Orientation::Horizontal, 5);
    container.set_widget_name("player-container");

    // 1. Widget Icon (Nerd Font: Music Note)
    let icon = Label::new(Some("\u{f001}")); // 
    icon.add_css_class("player-icon");

    // 2. Widget Label (Judul Lagu)
    let label = Label::new(Some("No Media"));
    label.add_css_class("player-label");
    // Limit panjang teks agar bar tidak meledak
    label.set_max_width_chars(30); 
    label.set_ellipsize(gtk::pango::EllipsizeMode::End);

    // 3. Tombol Play/Pause
    let play_btn = Button::with_label("\u{f04b}"); //  (Play icon default)
    play_btn.set_widget_name("player-btn");
    
    // Logic Klik Tombol Play/Pause
    play_btn.connect_clicked(move |_| {
        // Cari player aktif dan toggle playback
        if let Ok(player) = PlayerFinder::new().expect("Error DBus").find_active() {
            let _ = player.play_pause();
        }
    });

    // Susun Widget
    container.append(&icon);
    container.append(&label);
    container.append(&play_btn);

    // 4. Update Loop (Setiap 2 detik)
    // Kita clone widget agar bisa diakses di dalam closure timer
    let l_clone = label.clone();
    let b_clone = play_btn.clone();
    let c_clone = container.clone();

    glib::timeout_add_seconds_local(2, move || {
        update_player(&l_clone, &b_clone, &c_clone);
        ControlFlow::Continue
    });

    // Update pertama kali
    update_player(&label, &play_btn, &container);

    container
}

fn update_player(label: &Label, btn: &Button, container: &Box) {
    let finder = PlayerFinder::new();
    
    if let Ok(player_finder) = finder {
        // Cari player yang sedang "Playing" atau "Paused" (Active)
        if let Ok(player) = player_finder.find_active() {
            // Player ditemukan! Tampilkan container
            container.set_visible(true);

            // 1. Update Metadata (Judul - Artis)
            if let Ok(metadata) = player.get_metadata() {
                let title = metadata.title().unwrap_or("Unknown Title");
                let artists = metadata.artists().unwrap_or(vec![]);
                let artist_str = if !artists.is_empty() {
                    artists.join(", ")
                } else {
                    "Unknown Artist".to_string()
                };

                let text = format!("{} - {}", title, artist_str);
                label.set_label(&text);
            }

            // 2. Update Icon Play/Pause sesuai status
            if let Ok(status) = player.get_playback_status() {
                match status {
                    PlaybackStatus::Playing => btn.set_label("\u{f04c}"), //  (Pause)
                    PlaybackStatus::Paused => btn.set_label("\u{f04b}"),  //  (Play)
                    _ => btn.set_label("\u{f04b}"),
                }
            }
        } else {
            // Tidak ada player aktif? Sembunyikan widget agar bar bersih
            // Atau ganti text jadi "No Media"
            label.set_label("No Media");
            btn.set_label("\u{f04b}");
            // Opsional: container.set_visible(false); jika ingin auto-hide
        }
    }
}