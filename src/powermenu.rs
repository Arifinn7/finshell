use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Label, Orientation
};
use gtk4_layer_shell::{Edge, Layer, LayerShell, KeyboardMode};
use std::process::Command;
use crate::utils; // Import utility

pub fn create_powermenu_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Power Menu")
        .default_width(1920)
        .default_height(1080)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay); 
    
    // Fullscreen Anchor
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    window.set_keyboard_mode(KeyboardMode::Exclusive);
    window.set_widget_name("powermenu-window");

    // --- UI LAYOUT ---
    let main_box = Box::new(Orientation::Vertical, 0);
    main_box.set_valign(gtk::Align::Center);
    main_box.set_halign(gtk::Align::Center);
    main_box.set_widget_name("powermenu-box");

    // --- GUNAKAN UTILS ---
    // Setup ESC dan Click Outside Close
    utils::setup_overlay_window(&window, &main_box);

    // Content
    let label = Label::new(Some("Goodbye?"));
    label.set_widget_name("powermenu-title");
    main_box.append(&label);

    let buttons_box = Box::new(Orientation::Horizontal, 20);
    buttons_box.set_halign(gtk::Align::Center);
    buttons_box.set_margin_top(20);

    let actions = vec![
        ("", "Lock", "loginctl lock-session"),
        ("", "Suspend", "systemctl suspend"),
        ("", "Logout", "hyprctl dispatch exit"),
        ("", "Reboot", "systemctl reboot"),
        ("", "Shutdown", "systemctl poweroff"),
    ];

    for (icon, name, cmd) in actions {
        let btn = create_button(icon, name);
        let cmd_string = cmd.to_string();
        
        btn.connect_clicked(move |_| {
            let _ = Command::new("sh").arg("-c").arg(&cmd_string).spawn();
        });
        
        buttons_box.append(&btn);
    }

    main_box.append(&buttons_box);
    window.set_child(Some(&main_box));
    window.set_visible(false);
    window
}

fn create_button(icon: &str, label: &str) -> Button {
    let btn = Button::new();
    btn.add_css_class("powermenu-btn");

    let container = Box::new(Orientation::Vertical, 10);
    let lbl_icon = Label::new(Some(icon));
    lbl_icon.add_css_class("powermenu-icon");
    let lbl_text = Label::new(Some(label));
    lbl_text.add_css_class("powermenu-text");

    container.append(&lbl_icon);
    container.append(&lbl_text);
    btn.set_child(Some(&container));
    btn
}