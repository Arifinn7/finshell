use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Label, Orientation, 
    Scale, Adjustment, Image
};
use gtk4_layer_shell::{Edge, Layer, LayerShell, KeyboardMode};
use std::process::Command;
use crate::utils; // Reuse logic click-outside yang sudah kita buat

pub fn create_dashboard_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Dashboard")
        .default_width(400)
        .default_height(300)
        .build();

    // --- SETUP LAYER SHELL ---
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    
    // PERBAIKAN 1: Keyboard Mode ON (Agar ESC bisa menutup dashboard)
    window.set_keyboard_mode(KeyboardMode::OnDemand);

    // PERBAIKAN 2: Fullscreen Anchor (Agar bisa menangkap klik di luar)
    // Kita buat window ini menutupi seluruh layar (transparan)
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    window.set_widget_name("dashboard-window");

    // --- UI LAYOUT ---
    let main_box = Box::new(Orientation::Vertical, 15);
    main_box.set_widget_name("dashboard-box");
    main_box.set_width_request(380);

    // PERBAIKAN 3: Positioning
    // Taruh konten di Tengah Atas, beri jarak dari Top Bar
    main_box.set_valign(gtk::Align::Start); // Rata atas
    main_box.set_halign(gtk::Align::Center); // Rata tengah horizontal
    main_box.set_margin_top(55); // Beri jarak ~55px dari atas (sesuaikan tinggi bar kamu)

    // Gunakan utils untuk Close on ESC / Click Outside
    // Logic ini sekarang akan bekerja benar karena window fullscreen:
    // - Klik di Box -> Ditangkap gesture konten (Stop Propagation) -> Window tetap buka.
    // - Klik di luar Box -> Tembus ke Window Background -> Window tutup.
    utils::setup_overlay_window(&window, &main_box);

    // 1. HEADER PROFIL
    let profile_box = Box::new(Orientation::Horizontal, 15);
    profile_box.set_widget_name("dash-profile");
    
    let avatar = Image::from_icon_name("avatar-default");
    avatar.set_pixel_size(48);
    
    let user = std::env::var("USER").unwrap_or("User".to_string());
    // let host = std::env::var("HOSTNAME").unwrap_or("Arch".to_string()); // Kadang error di beberapa env
    let lbl_user = Label::new(Some(&format!("Hello, {}", user)));
    lbl_user.add_css_class("dash-username");

    profile_box.append(&avatar);
    profile_box.append(&lbl_user);
    main_box.append(&profile_box);

    // 2. SLIDERS (Volume & Brightness)
    // --- Volume Slider ---
    let vol_box = create_slider_row("", "Volume", |val| {
        let pct = (val * 100.0) as i32;
        let _ = Command::new("wpctl")
            .arg("set-volume")
            .arg("@DEFAULT_AUDIO_SINK@")
            .arg(format!("{}%", pct))
            .spawn();
    });
    main_box.append(&vol_box);

    // --- Brightness Slider ---
    // Note: Pastikan user punya permission (biasanya butuh add user ke group video/input atau rule udev)
    let bri_box = create_slider_row("", "Brightness", |val| {
        // Brightnessctl butuh range misal 0-255 atau persentase.
        // Kita coba pakai persentase absolut
        let pct = (val * 100.0) as i32;
        
        // Debug print untuk cek apakah slider jalan
        // println!("Set Brightness: {}%", pct); 
        
        let _ = Command::new("brightnessctl")
            .arg("set")
            .arg(format!("{}%", pct))
            .spawn();
    });
    main_box.append(&bri_box);

    // 3. QUICK TOGGLES (Mockup)
    let toggles_box = Box::new(Orientation::Horizontal, 10);
    toggles_box.set_halign(gtk::Align::Center);
    
    toggles_box.append(&create_toggle_btn(""));
    toggles_box.append(&create_toggle_btn(""));
    toggles_box.append(&create_toggle_btn(""));
    toggles_box.append(&create_toggle_btn(""));

    main_box.append(&toggles_box);

    window.set_child(Some(&main_box));
    window.set_visible(false);
    window
}

// Helper Slider
fn create_slider_row<F>(icon: &str, label: &str, on_change: F) -> Box 
where F: Fn(f64) + 'static {
    let container = Box::new(Orientation::Vertical, 5);
    
    let header = Box::new(Orientation::Horizontal, 10);
    let lbl_icon = Label::new(Some(icon));
    let lbl_text = Label::new(Some(label));
    header.append(&lbl_icon);
    header.append(&lbl_text);

    let adjustment = Adjustment::new(0.5, 0.0, 1.0, 0.05, 0.1, 0.0);
    let scale = Scale::new(Orientation::Horizontal, Some(&adjustment));
    scale.set_hexpand(true);
    scale.add_css_class("dash-slider");

    scale.connect_value_changed(move |s| {
        let val = s.value();
        on_change(val);
    });

    container.append(&header);
    container.append(&scale);
    container.add_css_class("dash-slider-box");
    
    container
}

// Helper Toggle
fn create_toggle_btn(icon: &str) -> Button {
    let btn = Button::with_label(icon);
    btn.add_css_class("dash-toggle");
    btn
}