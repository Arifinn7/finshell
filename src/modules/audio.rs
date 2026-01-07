use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation, GestureClick};
use super::WidgetModule;
use std::process::Command;

pub struct AudioModule;

impl AudioModule {
    fn get_volume_info() -> (i32, bool) {
        let output = Command::new("wpctl")
            .args(["get-volume", "@DEFAULT_AUDIO_SINK@"])
            .output()
            .ok();

        if let Some(out) = output {
            let str_out = String::from_utf8_lossy(&out.stdout);
            let is_muted = str_out.contains("MUTED");
            if let Some(vol_str) = str_out.split("Volume: ").nth(1) {
                let clean_vol = vol_str.split_whitespace().next().unwrap_or("0");
                let vol_float: f64 = clean_vol.parse().unwrap_or(0.0);
                return ((vol_float * 100.0) as i32, is_muted);
            }
        }
        (0, false)
    }

    fn update_view(label: &Label, container: &Box) {
        let (volume, is_muted) = Self::get_volume_info();
        let icon = if is_muted { "󰝟" } else if volume >= 50 { "" } else if volume >= 20 { "" } else { "" };
        
        label.set_text(&format!("{} {}%", icon, volume));
        
        container.remove_css_class("muted");
        if is_muted { container.add_css_class("muted"); }
    }

    fn run_wpctl(args: &[&str]) {
        let _ = Command::new("wpctl")
            .arg("set-volume")
            .arg("@DEFAULT_AUDIO_SINK@")
            .args(args)
            .spawn();
    }
    
    fn toggle_mute() {
        let _ = Command::new("wpctl").args(["set-mute", "@DEFAULT_AUDIO_SINK@", "toggle"]).spawn();
    }
}

impl WidgetModule for AudioModule {
    fn build_widget(&self) -> gtk4::Widget {
        let container = Box::new(Orientation::Horizontal, 5);
        container.add_css_class("audio-widget");

        let label = Label::new(None);
        container.append(&label);

        // --- SCROLL ---
        let scroll = gtk4::EventControllerScroll::new(gtk4::EventControllerScrollFlags::VERTICAL);
        let label_clone_scroll = label.clone();
        let container_clone_scroll = container.clone();
        
        scroll.connect_scroll(move |_, _, dy| {
            if dy > 0.0 { Self::run_wpctl(&["5%-"]); } 
            else { Self::run_wpctl(&["5%+", "--limit", "1.0"]); }
            Self::update_view(&label_clone_scroll, &container_clone_scroll);
            gtk4::glib::Propagation::Stop
        });
        container.add_controller(scroll);

        // --- CLICK ---
        let click = GestureClick::new();
        // Variabel ini "dimiliki" oleh closure 'connect_pressed'
        let label_for_click = label.clone();
        let container_for_click = container.clone();
        
        click.connect_pressed(move |_, _, _, _| {
            Self::toggle_mute();
            
            // PERBAIKAN DI SINI:
            // Kita clone lagi variabelnya UNTUK dipindahkan ke timeout
            let l = label_for_click.clone();
            let c = container_for_click.clone();
            
            glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
                Self::update_view(&l, &c);
                glib::ControlFlow::Break
            });
        });
        container.add_controller(click);

        // --- TIMER ---
        let label_clone = label.clone();
        let container_clone = container.clone();
        
        Self::update_view(&label, &container);

        glib::timeout_add_seconds_local(2, move || {
            Self::update_view(&label_clone, &container_clone);
            glib::ControlFlow::Continue
        });

        container.upcast()
    }
}