use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation};
use super::WidgetModule;
use std::process::Command;
use std::thread;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::env;
use std::path::PathBuf;
use serde::Deserialize;
use std::sync::mpsc; // Gunakan channel standar Rust

#[derive(Deserialize, Debug, Clone)]
struct WorkspaceData {
    id: i32,
    #[serde(skip)] // Ignore field name
    _name: String, 
}

#[derive(Deserialize, Debug, Clone)]
struct ActiveWorkspaceData {
    id: i32,
}

enum IpcEvent {
    WorkspaceChanged(i32),
    WorkspacesListChanged,
    DataRefreshed(Vec<WorkspaceData>, i32),
    Error(String),
}

pub struct WorkspacesModule;

impl WorkspacesModule {
    fn get_event_socket_path() -> Result<PathBuf, String> {
        let xdg_runtime = env::var("XDG_RUNTIME_DIR")
            .map_err(|_| "XDG_RUNTIME_DIR not set")?;
        
        let signature = env::var("HYPRLAND_INSTANCE_SIGNATURE")
            .map_err(|_| "HYPRLAND_INSTANCE_SIGNATURE not set")?;
        
        let path = PathBuf::from(format!("{}/hypr/{}/.socket2.sock", xdg_runtime, signature));
        if !path.exists() {
            return Err(format!("Socket not found at: {:?}", path));
        }
        Ok(path)
    }

    fn fetch_full_state() -> Result<(Vec<WorkspaceData>, i32), String> {
        let output_ws = Command::new("hyprctl")
            .args(["workspaces", "-j"])
            .output()
            .map_err(|e| e.to_string())?;

        let output_active = Command::new("hyprctl")
            .args(["activeworkspace", "-j"])
            .output()
            .map_err(|e| e.to_string())?;

        let workspaces: Vec<WorkspaceData> = serde_json::from_slice(&output_ws.stdout)
            .map_err(|_| "Failed to parse workspaces JSON")?;
            
        let active: ActiveWorkspaceData = serde_json::from_slice(&output_active.stdout)
            .map_err(|_| "Failed to parse activeworkspace JSON")?;

        Ok((workspaces, active.id))
    }

    fn rebuild_ui(container: &Box, workspaces: Vec<WorkspaceData>, active_id: i32) {
        while let Some(child) = container.first_child() {
            container.remove(&child);
        }

        let mut sorted_ws = workspaces;
        sorted_ws.sort_by_key(|w| w.id);

        for ws in sorted_ws {
            let id = ws.id;
            if id < 0 { continue; }

            let button = Button::builder()
                .label(id.to_string())
                .css_classes(vec!["workspace-button".to_string()])
                .build();

            if id == active_id {
                button.add_css_class("active");
            }

            let button_clone = button.clone();
            button.connect_clicked(move |_| {
                button_clone.add_css_class("active"); 
                let _ = Command::new("hyprctl")
                    .args(["dispatch", "workspace", &id.to_string()])
                    .spawn();
            });

            container.append(&button);
        }
    }

    fn update_active_state(container: &Box, active_id: i32) {
        let mut child = container.first_child();
        while let Some(widget) = child {
            if let Some(button) = widget.downcast_ref::<Button>() {
                if let Some(label) = button.label() {
                    if let Ok(id) = label.parse::<i32>() {
                        if id == active_id {
                            button.add_css_class("active");
                        } else {
                            button.remove_css_class("active");
                        }
                    }
                }
            }
            child = widget.next_sibling();
        }
    }
}

impl WidgetModule for WorkspacesModule {
    fn build_widget(&self) -> gtk4::Widget {
        let container = Box::new(Orientation::Horizontal, 5);
        container.add_css_class("workspaces-widget");
        let container_weak = container.downgrade();

        // REPLACEMENT: Gunakan std::sync::mpsc channel
        // Ini adalah cara standar Rust, tidak terpengaruh versi library
        let (sender, receiver) = mpsc::channel();
        let sender_clone = sender.clone();

        // Thread 1: Fetch Awal
        thread::spawn(move || {
            match Self::fetch_full_state() {
                Ok((ws, active)) => {
                    let _ = sender_clone.send(IpcEvent::DataRefreshed(ws, active));
                }
                Err(e) => {
                    let _ = sender_clone.send(IpcEvent::Error(e));
                }
            }
        });

        // Thread 2: Socket Listener
        thread::spawn(move || {
            let socket_path = match Self::get_event_socket_path() {
                Ok(p) => p,
                Err(e) => {
                    let _ = sender.send(IpcEvent::Error(e));
                    return;
                }
            };

            loop {
                if let Ok(stream) = UnixStream::connect(&socket_path) {
                    let reader = BufReader::new(stream);
                    for line in reader.lines().flatten() {
                        if line.starts_with("workspace>>") {
                            if let Some(data) = line.split(">>").nth(1) {
                                if let Ok(id) = data.parse::<i32>() {
                                        let _ = sender.send(IpcEvent::WorkspaceChanged(id));
                                } else {
                                        let _ = sender.send(IpcEvent::WorkspacesListChanged);
                                }
                            }
                        } else if line.starts_with("createworkspace>>") 
                                || line.starts_with("destroyworkspace>>") {
                            let _ = sender.send(IpcEvent::WorkspacesListChanged);
                        }
                    }
                } else {
                    thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        });

        // UI Thread: Cek inbox setiap 100ms
        // try_recv() itu non-blocking (instan), jadi tidak bikin berat UI
        glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
            if let Some(container) = container_weak.upgrade() {
                // Proses SEMUA pesan yang menumpuk di inbox (drain channel)
                while let Ok(msg) = receiver.try_recv() {
                    match msg {
                        IpcEvent::DataRefreshed(ws, active) => {
                            Self::rebuild_ui(&container, ws, active);
                        }
                        IpcEvent::WorkspaceChanged(active_id) => {
                            Self::update_active_state(&container, active_id);
                        }
                        IpcEvent::WorkspacesListChanged => {
                            // Quick refresh
                            thread::spawn(move || {
                                let _ = Command::new("hyprctl").arg("reload").output();
                            });
                             // Trigger fetch ulang manual di UI thread
                             if let Ok((ws, active)) = Self::fetch_full_state() {
                                Self::rebuild_ui(&container, ws, active);
                             }
                        }
                        IpcEvent::Error(e) => eprintln!("[Workspaces] {}", e),
                    }
                }
            }
            glib::ControlFlow::Continue
        });

        container.upcast()
    }
}