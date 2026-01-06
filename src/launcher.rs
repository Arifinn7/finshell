use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Entry, Image, Label, 
    ListBox, ListBoxRow, Orientation, ScrolledWindow, EventControllerKey
};
use gtk4_layer_shell::{Edge, Layer, LayerShell, KeyboardMode};
use gdk::Key;
use glib::Propagation;
use std::process::{Command, Stdio};
use std::time::Duration; // Import Duration untuk Timer
use crate::apps;

pub fn create_launcher_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("App Launcher")
        .default_width(500)
        .default_height(600)
        .build();

    // --- LAYER SHELL SETUP ---
    window.init_layer_shell();
    window.set_layer(Layer::Overlay); 
    window.set_keyboard_mode(KeyboardMode::OnDemand);

    // Posisi Tengah (Stabil)
    window.set_anchor(Edge::Top, false);
    window.set_anchor(Edge::Bottom, false);
    window.set_anchor(Edge::Left, false);
    window.set_anchor(Edge::Right, false);

    window.set_widget_name("launcher-window");

    // --- LOGIC: ESC TO CLOSE ---
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

    // --- LAYOUT ---
    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_widget_name("launcher-box");
    
    // --- CONTENT ---
    let search_entry = Entry::new();
    search_entry.set_placeholder_text(Some("Cari Aplikasi..."));
    search_entry.set_widget_name("launcher-search");
    search_entry.set_icon_from_icon_name(gtk::EntryIconPosition::Primary, Some("system-search-symbolic"));
    
    main_box.append(&search_entry);

    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    
    let list_box = ListBox::new();
    list_box.set_widget_name("launcher-list");
    list_box.set_selection_mode(gtk::SelectionMode::None);

    // --- LOAD APPS ---
    let apps_data = apps::get_installed_apps();
    
    for app_info in apps_data {
        let row_box = Box::new(Orientation::Horizontal, 10);
        
        let icon_name = if app_info.icon.is_empty() { "application-x-executable" } else { &app_info.icon };
        let image = Image::from_icon_name(icon_name);
        image.set_pixel_size(32);
        
        let label = Label::new(Some(&app_info.name));
        
        row_box.append(&image);
        row_box.append(&label);

        let button = Button::builder().child(&row_box).build();
        button.add_css_class("app-button");
        button.set_tooltip_text(Some(&app_info.name.to_lowercase()));

        let exec_cmd = app_info.exec.clone();
        
        // Clone window untuk akses di dalam closure
        let win_clone = window.clone();
        
        // --- LOGIC KLIK DENGAN DELAY ---
        button.connect_clicked(move |_| {
            // 1. Eksekusi App
            let _ = Command::new("sh")
                .arg("-c")
                .arg(&exec_cmd)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn();
            
            // 2. TUTUP WINDOW DENGAN JEDA 300ms
            // Jeda ini memberi waktu agar state tombol "Pressed" selesai diproses oleh GTK
            // sebelum window dihilangkan. Ini mencegah crash di VM/HDD.
            let win = win_clone.clone();
            glib::timeout_add_local(Duration::from_millis(150), move || {
                win.set_visible(false);
                glib::ControlFlow::Break // Stop timer agar tidak berulang
            });
        });

        list_box.append(&button);
    }

    scrolled.set_child(Some(&list_box));
    
    // --- SEARCH LOGIC ---
    let list_box_clone = list_box.clone();
    search_entry.connect_changed(move |_| { list_box_clone.invalidate_filter(); });

    list_box.set_filter_func(move |row: &ListBoxRow| -> bool {
        let query = search_entry.text().to_lowercase();
        if query.is_empty() { return true; }
        if let Some(child) = row.child() {
            if let Some(button) = child.downcast_ref::<Button>() {
                if let Some(app_name) = button.tooltip_text() {
                    if app_name.contains(&query) { return true; }
                }
            }
        }
        false
    });

    main_box.append(&scrolled);
    window.set_child(Some(&main_box));
    window.set_visible(false);
    
    window
}