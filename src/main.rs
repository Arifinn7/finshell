use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, CenterBox, Orientation, CssProvider};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use gdk::Display;
use gtk::style_context_add_provider_for_display;
use std::path::PathBuf;

mod workspaces;
mod clock;
mod player;
mod system;
mod apps;
mod launcher;
mod osd;
mod powermenu;
mod utils;
mod dashboard;

fn main() {
    let app = Application::builder()
        .application_id("com.my.rust_shell")
        .build();

    app.connect_activate(build_ui);
    app.connect_startup(|_| load_css());
    
    app.run();
}

fn build_ui(app: &Application) {
    // 1. SETUP WINDOWS
    osd::create_osd_window(app);
    let launcher_win = launcher::create_launcher_window(app);
    let power_win = powermenu::create_powermenu_window(app);
    let dashboard_win = dashboard::create_dashboard_window(app);

    // 2. SETUP BAR
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Rust Shell Bar")
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);
    window.auto_exclusive_zone_enable();

    let center_box = CenterBox::new();
    center_box.set_widget_name("main-bar");

    // --- KIRI ---
    let left_box = Box::new(Orientation::Horizontal, 10);
    let menu_btn = Button::with_label("");
    menu_btn.set_widget_name("menu-button");
    let l_win_clone = launcher_win.clone();
    menu_btn.connect_clicked(move |_| {
        if l_win_clone.is_visible() { l_win_clone.set_visible(false); }
        else { l_win_clone.set_visible(true); l_win_clone.present(); }
    });
    left_box.append(&menu_btn);
    left_box.append(&workspaces::create_workspace_widget());
    center_box.set_start_widget(Some(&left_box));

    // --- TENGAH ---
    let clock_btn = clock::create_clock_widget();
    let d_win_clone = dashboard_win.clone();
    clock_btn.connect_clicked(move |_| {
        if d_win_clone.is_visible() { d_win_clone.set_visible(false); }
        else { d_win_clone.set_visible(true); d_win_clone.present(); }
    });
    center_box.set_center_widget(Some(&clock_btn));

    // --- KANAN ---
    let right_box = Box::new(Orientation::Horizontal, 10);
    right_box.set_halign(gtk::Align::End);
    right_box.append(&system::create_system_widget());
    right_box.append(&player::create_player_widget());

    let power_btn = Button::with_label("");
    power_btn.set_widget_name("power-button");
    let p_win_clone = power_win.clone();
    power_btn.connect_clicked(move |_| { p_win_clone.set_visible(true); p_win_clone.present(); });
    right_box.append(&power_btn);

    center_box.set_end_widget(Some(&right_box));
    window.set_child(Some(&center_box));
    window.present();
}

fn load_css() {
    let display = Display::default().expect("Error: Tidak bisa connect ke Wayland display.");
    let provider = CssProvider::new();
    
    // PERBAIKAN FINAL: Memuat CSS dari ~/.config/rust_shell/style.css
    // Ini agar aplikasi tetap cantik dimanapun ia dijalankan.
    let mut path = PathBuf::from(std::env::var("HOME").unwrap_or_default());
    path.push(".config/rust_shell/style.css");

    // Fallback: Jika di folder config tidak ada, coba cari di folder saat ini (untuk dev)
    if !path.exists() {
        println!("Config tidak ditemukan di {:?}, mencoba lokal ./style.css", path);
        path = PathBuf::from("style.css");
    }

    provider.load_from_path(path.to_str().unwrap());

    style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}