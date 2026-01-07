mod modules;
mod config; // <-- Panggil file config baru

use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, CenterBox, CssProvider, gdk, Box, Orientation
};
use gtk4_layer_shell::{Layer, LayerShell, Edge};

// Import semua modul
use modules::clock::ClockModule;
use modules::workspaces::WorkspacesModule;
use modules::battery::BatteryModule;
use modules::WidgetModule;
use config::Config;
use modules::audio::AudioModule;
use modules::mpris::MprisModule;
use modules::network::NetworkModule;
use modules::sys_info::SysInfoModule;
use modules::power::PowerModule;

// --- PABRIK MODUL (Module Factory) ---
// Fungsi ini menerjemahkan string "clock" menjadi Widget Clock
fn create_module(name: &str) -> Option<gtk4::Widget> {
    match name {
        "workspaces" => Some(WorkspacesModule.build_widget()),
        "clock" => Some(ClockModule.build_widget()),
        "battery" => Some(BatteryModule.build_widget()),
        "audio" => Some(AudioModule.build_widget()),
        "mpris" => Some(MprisModule.build_widget()),
        "network" => Some(NetworkModule.build_widget()),
        "sys_info" => Some(SysInfoModule.build_widget()),
        "power" => Some(PowerModule.build_widget()),
        "spacer" => {
            // Widget kosong untuk peregang jarak (opsional)
            let spacer = Box::new(Orientation::Horizontal, 0);
            spacer.set_hexpand(true);
            Some(spacer.upcast())
        },
        _ => {
            eprintln!("Warning: Modul '{}' tidak dikenal", name);
            None
        }
    }
}

// Fungsi helper untuk mengisi kotak (kiri/tengah/kanan) berdasarkan config
fn fill_box(container: &Box, module_names: &Option<Vec<String>>) {
    if let Some(names) = module_names {
        for name in names {
            if let Some(widget) = create_module(name) {
                container.append(&widget);
            }
        }
    }
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_path("style.css");

    gtk4::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Gagal mendapatkan display GDK"),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let app = Application::builder()
        .application_id("com.arifinn7.finshell")
        .build();

    app.connect_activate(|app| {
        // 1. Load Config
        let config = Config::load();
        
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Finshell")
            .build();

        // 2. Setup Layer Shell berdasarkan Config
        window.init_layer_shell();
        window.set_namespace("finshell");
        window.set_layer(Layer::Top);
        
        // Atur posisi (Top/Bottom)
        let position = config.bar.position.as_deref().unwrap_or("top");
        let is_bottom = position == "bottom";
        
        window.set_anchor(if is_bottom { Edge::Bottom } else { Edge::Top }, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        
        // Atur Tinggi
        if let Some(h) = config.bar.height {
            window.set_height_request(h);
        }

        window.auto_exclusive_zone_enable();

        // 3. Layout Utama
        let center_box = CenterBox::new();
        center_box.add_css_class("main-bar");

        // --- KONSTRUKSI DINAMIS ---
        
        // Kiri
        let left_box = Box::new(Orientation::Horizontal, 5);
        fill_box(&left_box, &config.modules.left);
        center_box.set_start_widget(Some(&left_box));

        // Tengah
        let mid_box = Box::new(Orientation::Horizontal, 5);
        fill_box(&mid_box, &config.modules.center);
        center_box.set_center_widget(Some(&mid_box));

        // Kanan
        let right_box = Box::new(Orientation::Horizontal, 5);
        fill_box(&right_box, &config.modules.right);
        center_box.set_end_widget(Some(&right_box));

        window.set_child(Some(&center_box));
        window.present();
    });

    app.run();
}