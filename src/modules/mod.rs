// Daftar modul yang tersedia
pub mod clock;
pub mod workspaces; // Modul baru untuk workspace
pub mod battery;
pub mod audio;
pub mod mpris;
pub mod network;
pub mod sys_info;
pub mod power;

// Trait (Kontrak) yang harus dipatuhi semua widget
pub trait WidgetModule {
    // Setiap widget wajib punya fungsi ini untuk merender tampilannya
    fn build_widget(&self) -> gtk4::Widget;
}