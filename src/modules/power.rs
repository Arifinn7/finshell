use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation, Revealer, RevealerTransitionType};
use super::WidgetModule;
use std::process::Command;
use std::cell::RefCell;
use std::rc::Rc;

pub struct PowerModule;

impl PowerModule {
    // Fungsi eksekusi perintah sistem
    fn run_cmd(cmd: &str) {
        if cmd == "logout" {
            let _ = Command::new("hyprctl").arg("dispatch").arg("exit").spawn();
        } else {
            // systemctl poweroff / reboot
            let _ = Command::new("systemctl").arg(cmd).spawn();
        }
    }
}

impl WidgetModule for PowerModule {
    fn build_widget(&self) -> gtk4::Widget {
        let container = Box::new(Orientation::Horizontal, 0);
        container.add_css_class("power-widget-container");

        // 1. Tombol Utama (Ikon Power)
        let main_btn = Button::builder()
            .label("⏻")
            .css_classes(vec!["power-main-btn".to_string()])
            .build();

        // 2. Wadah Tombol Aksi (Disembunyikan dalam Revealer)
        let actions_box = Box::new(Orientation::Horizontal, 5);
        actions_box.add_css_class("power-actions");

        // Tombol Logout
        let btn_logout = Button::builder().label("󰗽").build();
        btn_logout.add_css_class("power-sub-btn");
        btn_logout.add_css_class("logout");
        btn_logout.connect_clicked(|_| Self::run_cmd("logout"));

        // Tombol Reboot
        let btn_reboot = Button::builder().label("").build();
        btn_reboot.add_css_class("power-sub-btn");
        btn_reboot.add_css_class("reboot");
        btn_reboot.connect_clicked(|_| Self::run_cmd("reboot"));

        // Tombol Shutdown
        let btn_shutdown = Button::builder().label("⏾").build();
        btn_shutdown.add_css_class("power-sub-btn");
        btn_shutdown.add_css_class("shutdown");
        btn_shutdown.connect_clicked(|_| Self::run_cmd("poweroff"));

        actions_box.append(&btn_logout);
        actions_box.append(&btn_reboot);
        actions_box.append(&btn_shutdown);

        // 3. Revealer (Tirai Animasi)
        let revealer = Revealer::builder()
            .transition_type(RevealerTransitionType::SlideLeft)
            .transition_duration(300) // 300ms animasi
            .child(&actions_box)
            .reveal_child(false) // Default tertutup
            .build();

        // 4. Logika Buka/Tutup
        // Kita butuh Rc<RefCell> untuk menyimpan ID Timer (supaya bisa di-reset)
        let timer_handle = Rc::new(RefCell::new(None::<glib::SourceId>));
        
        let revealer_clone = revealer.clone();
        let timer_clone = timer_handle.clone();

        main_btn.connect_clicked(move |_| {
            let is_open = revealer_clone.reveals_child();
            
            if is_open {
                // Kalau sedang terbuka -> Tutup
                revealer_clone.set_reveal_child(false);
            } else {
                // Kalau tertutup -> Buka
                revealer_clone.set_reveal_child(true);

                // Batalkan timer lama jika ada (biar gak nutup mendadak)
                if let Some(source_id) = timer_clone.borrow_mut().take() {
                    source_id.remove();
                }

                // Pasang Timer Baru: Tutup otomatis setelah 5 detik
                let r_clone = revealer_clone.clone();
                let t_clone = timer_clone.clone();
                
                let source_id = glib::timeout_add_seconds_local(5, move || {
                    r_clone.set_reveal_child(false);
                    *t_clone.borrow_mut() = None; // Hapus handle timer
                    glib::ControlFlow::Break
                });

                *timer_clone.borrow_mut() = Some(source_id);
            }
        });

        // Susunan: [Actions (Hidden)] [Power Button]
        // Kita taruh actions di kiri tombol power, atau kanan tergantung selera.
        // Di sini kita taruh di sebelah kiri (karena posisi widget di ujung kanan layar)
        container.append(&revealer); 
        container.append(&main_btn);

        container.upcast()
    }
}