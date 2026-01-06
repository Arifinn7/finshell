use gtk::prelude::*;
use gtk::{Box, Button, Orientation};
use hyprland::shared::{HyprData, HyprDataActive};
use hyprland::data::{Workspaces, Workspace};
use hyprland::event_listener::AsyncEventListener;
use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use std::thread;
use tokio::runtime::Runtime;
use glib::clone;

// Enum sinyal untuk update UI
pub enum WorkspaceEvent {
    Update,
}

pub fn create_workspace_widget() -> Box {
    let container = Box::new(Orientation::Horizontal, 5);
    container.set_widget_name("workspaces-container");

    let (sender, receiver) = async_channel::unbounded();

    // 1. BACKGROUND THREAD: Listener Event Hyprland (Tetap Async)
    thread::spawn(move || {
        let rt = Runtime::new().expect("Gagal inisialisasi Tokio Runtime");
        
        rt.block_on(async {
            let mut event_listener = AsyncEventListener::new();
            
            let s_change = sender.clone();
            let s_add = sender.clone();
            let s_destroy = sender.clone();

            // Handler Workspace Changed
            event_listener.add_workspace_changed_handler(move |_| {
                let sender = s_change.clone();
                std::boxed::Box::pin(async move {
                    let _ = sender.send(WorkspaceEvent::Update).await;
                })
            });
            
            // Handler Workspace Added
            event_listener.add_workspace_added_handler(move |_| {
                let sender = s_add.clone();
                std::boxed::Box::pin(async move {
                    let _ = sender.send(WorkspaceEvent::Update).await;
                })
            });

            // Handler Workspace Deleted
            event_listener.add_workspace_deleted_handler(move |_| {
                let sender = s_destroy.clone();
                std::boxed::Box::pin(async move {
                    let _ = sender.send(WorkspaceEvent::Update).await;
                })
            });

            if let Err(e) = event_listener.start_listener_async().await {
                eprintln!("Error Listener Hyprland: {:?}", e);
            }
        });
    });

    // 2. MAIN THREAD: UI Updater
    // Kita tetap menggunakan channel receiver secara async
    let main_context = glib::MainContext::default();
    
    // Initial Render
    let container_start = container.clone();
    main_context.spawn_local(async move {
        // Panggil fungsi synchronous
        refresh_ui(&container_start);
    });

    // Loop Update dari Channel
    main_context.spawn_local(clone!(@weak container => async move {
        while let Ok(_) = receiver.recv().await {
            // Panggil fungsi synchronous saat ada sinyal
            refresh_ui(&container);
        }
    }));

    container
}

// PERBAIKAN: Fungsi ini tidak lagi 'async'
// Kita menggunakan Blocking API (.get()) yang aman dipanggil di Main Thread
fn refresh_ui(container: &Box) {
    // Hapus widget lama
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }

    // Ambil data (Synchronous / Blocking)
    // Ini tidak butuh Tokio Runtime
    let workspaces_res = Workspaces::get();
    let active_res = Workspace::get_active();

    if let (Ok(workspaces_data), Ok(active_ws)) = (workspaces_res, active_res) {
        let mut workspaces_vec: Vec<Workspace> = workspaces_data.into_iter().collect();
        workspaces_vec.sort_by(|a, b| a.id.cmp(&b.id));

        for ws in workspaces_vec {
            let id = ws.id;
            let button = Button::with_label(&id.to_string());

            if id == active_ws.id {
                button.add_css_class("workspace-active");
            } else {
                button.add_css_class("workspace-inactive");
            }

            button.connect_clicked(move |_| {
                // Dispatch juga blocking call, aman di sini
                let _ = Dispatch::call(DispatchType::Workspace(
                    WorkspaceIdentifierWithSpecial::Id(id)
                ));
            });

            container.append(&button);
        }
    } else {
        eprintln!("Gagal mengambil data workspace (Sync).");
    }
}