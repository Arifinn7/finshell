use gtk::prelude::*;
use gtk::{Button, Label}; // Ubah return type jadi Button
use chrono::Local;
use glib::ControlFlow;

// Return type berubah dari Label ke Button
pub fn create_clock_widget() -> Button {
    let button = Button::new();
    button.set_widget_name("clock-button"); // ID CSS baru

    let label = Label::new(None);
    label.set_widget_name("clock-label");
    
    // Masukkan label ke dalam button
    button.set_child(Some(&label));

    // Update waktu pertama kali
    update_time(&label);

    // Timer Loop
    let label_clone = label.clone();
    glib::timeout_add_seconds_local(1, move || {
        update_time(&label_clone);
        ControlFlow::Continue
    });

    button
}

fn update_time(label: &Label) {
    let now = Local::now();
    // Format: Jam:Menit  Hari Tanggal Bulan
    let format = now.format("%H:%M  %a %d %b").to_string();
    label.set_label(&format);
}