use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

// Struktur data untuk menyimpan info aplikasi
#[derive(Clone, Debug)]
pub struct AppInfo {
    pub name: String,
    pub icon: String,
    pub exec: String,
}

pub fn get_installed_apps() -> Vec<AppInfo> {
    let mut apps = Vec::new();
    
    // Folder standar file .desktop di Linux
    let paths = vec![
        "/usr/share/applications",
        // Kita bisa tambah path user lokal nanti: format!("{}/.local/share/applications", std::env::var("HOME").unwrap())
    ];

    for path in paths {
        if !Path::new(path).exists() { continue; }

        // Scan direktori
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("desktop") {
                if let Some(app) = parse_desktop_file(entry.path()) {
                    apps.push(app);
                }
            }
        }
    }

    // Sortir berdasarkan nama (A-Z)
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn parse_desktop_file(path: &Path) -> Option<AppInfo> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut name = String::new();
    let mut icon = String::new();
    let mut exec = String::new();
    let mut no_display = false;
    let mut is_desktop_entry = false;

    for line in reader.lines() {
        if let Ok(l) = line {
            let trim = l.trim();
            if trim == "[Desktop Entry]" {
                is_desktop_entry = true;
                continue;
            }
            
            // Kita hanya peduli bagian [Desktop Entry], bukan [Action ...]
            if trim.starts_with('[') && trim != "[Desktop Entry]" {
                is_desktop_entry = false;
            }

            if !is_desktop_entry { continue; }

            if trim.starts_with("Name=") && name.is_empty() {
                name = trim.replace("Name=", "");
            } else if trim.starts_with("Icon=") {
                icon = trim.replace("Icon=", "");
            } else if trim.starts_with("Exec=") && exec.is_empty() {
                exec = trim.replace("Exec=", "");
            } else if trim == "NoDisplay=true" {
                no_display = true;
            }
        }
    }

    if no_display || name.is_empty() || exec.is_empty() {
        return None;
    }

    // Bersihkan Exec command (hapus placeholder argumen seperti %u, %F)
    // Contoh: "firefox %u" menjadi "firefox"
    let clean_exec = exec
        .split_whitespace()
        .filter(|s| !s.starts_with('%'))
        .collect::<Vec<&str>>()
        .join(" ");

    Some(AppInfo {
        name,
        icon,
        exec: clean_exec,
    })
}