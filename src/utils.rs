use gtk::prelude::*;
use gtk::{ApplicationWindow, EventControllerKey, GestureClick, Widget};
use gdk::Key;
use glib::{clone, Propagation};

/// Fungsi ini mengatur behavior standar untuk window overlay.
/// Menggunakan GEOMETRY CHECK agar klik pada tombol child tidak menutup window.
pub fn setup_overlay_window(
    window: &ApplicationWindow, 
    content_area: &impl IsA<Widget>
) {
    // --- 1. LOGIC ESCAPE KEY ---
    let key_controller = EventControllerKey::new();
    let win_clone_key = window.clone();
    
    key_controller.connect_key_pressed(move |_, key, _, _| {
        if key == Key::Escape {
            win_clone_key.set_visible(false);
            return Propagation::Stop;
        }
        Propagation::Proceed
    });
    window.add_controller(key_controller);

    // --- 2. LOGIC CLICK OUTSIDE (Geometry Check) ---
    let gesture_click = GestureClick::new();
    
    // Kita butuh referensi ke content_area untuk menghitung posisinya
    // Kita cast ke Widget agar bisa dikelola oleh glib::clone
    let content_widget = content_area.as_ref().clone(); 

    gesture_click.connect_pressed(clone!(@weak window, @weak content_widget => move |_, _, x, y| {
        // x dan y adalah posisi klik relatif terhadap Window (kiri-atas window = 0,0)
        
        // Hitung batas (bounds) kotak konten relatif terhadap window
        if let Some(bounds) = content_widget.compute_bounds(&window) {
            let bx = bounds.x() as f64;
            let by = bounds.y() as f64;
            let bw = bounds.width() as f64;
            let bh = bounds.height() as f64;

            // Cek apakah klik terjadi DI DALAM kotak konten
            let inside_x = x >= bx && x <= (bx + bw);
            let inside_y = y >= by && y <= (by + bh);

            if inside_x && inside_y {
                // Klik terjadi DI DALAM kotak (kena tombol/slider/profil)
                // Biarkan event lanjut agar tombol bisa diklik.
                // JANGAN tutup window.
                return;
            }
        }

        // Jika sampai sini, berarti klik terjadi DI LUAR kotak konten (di area gelap)
        window.set_visible(false);
    }));

    window.add_controller(gesture_click);
}