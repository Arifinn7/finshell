use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub bar: BarConfig,
    pub modules: ModulesConfig,
}

#[derive(Deserialize, Clone)]
pub struct BarConfig {
    pub position: Option<String>,
    pub height: Option<i32>,
}

#[derive(Deserialize, Clone)]
pub struct ModulesConfig {
    pub left: Option<Vec<String>>,
    pub center: Option<Vec<String>>,
    pub right: Option<Vec<String>>,
}

impl Config {
    // Fungsi untuk memuat config
    pub fn load() -> Self {
        // Coba cari di folder saat ini dulu (untuk development)
        let local_path = "config.toml";
        if Path::new(local_path).exists() {
            return Self::from_file(local_path);
        }

        // Kalau tidak ada, cari di ~/.config/finshell/config.toml
        if let Ok(home) = std::env::var("HOME") {
            let config_path = format!("{}/.config/finshell/config.toml", home);
            if Path::new(&config_path).exists() {
                return Self::from_file(&config_path);
            }
        }

        // Kalau tidak ada sama sekali, pakai default
        println!("Config file not found, using defaults.");
        Self::default()
    }

    fn from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Error parsing config: {}", e);
            Self::default()
        })
    }

    // Default configuration (kalau file config rusak/hilang)
    fn default() -> Self {
        Self {
            bar: BarConfig {
                position: Some("top".to_string()),
                height: Some(40),
            },
            modules: ModulesConfig {
                left: Some(vec!["workspaces".to_string()]),
                center: Some(vec!["clock".to_string()]),
                right: Some(vec!["battery".to_string()]),
            },
        }
    }
}