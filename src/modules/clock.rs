// Pastikan baris ini gtk4, BUKAN gtk
use gtk4::prelude::*; 
use gtk4::Label;
use chrono::Local;
use super::WidgetModule;

pub struct ClockModule;

impl WidgetModule for ClockModule {
    fn build_widget(&self) -> gtk4::Widget { // Pastikan return type-nya gtk4
        let label = Label::new(None);
        label.add_css_class("clock-widget");

        let label_clone = label.clone();
        glib::timeout_add_seconds_local(1, move || {
            let time = Local::now().format("%H:%M:%S").to_string();
            label_clone.set_text(&time);
            glib::ControlFlow::Continue
        });

        label.upcast()
    }
}