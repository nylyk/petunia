use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::data::Thread;

pub fn store_path() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("petunia")
        .join("petunia.db3")
}

fn session_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("petunia")
        .join("session.json")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub window: WindowSize,
    pub layout: Option<Layout>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            window: WindowSize {
                width: 1024.0,
                height: 720.0,
            },
            layout: None,
        }
    }
}

impl Session {
    pub fn load() -> Self {
        fs::read_to_string(session_path())
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let path = session_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let contents = serde_json::to_string_pretty(self).expect("session is serializable");
        if let Err(error) = fs::write(&path, contents) {
            warn!(%error, path = %path.display(), "failed to save session");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Layout {
    Split {
        axis: Axis,
        ratio: f32,
        a: Box<Layout>,
        b: Box<Layout>,
    },
    Pane(Option<Thread>),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Axis {
    Horizontal,
    Vertical,
}
