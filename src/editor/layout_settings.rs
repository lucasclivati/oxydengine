use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BottomTab {
    None,
    ContentDrawer,
    OutputLog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSettings {
    pub outliner_width: f32,
    pub details_height: f32,
    pub active_bottom_tab: BottomTab,
    pub content_drawer_height: f32,
    pub active_tab_index: usize,
    pub open_tabs: Vec<String>,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            outliner_width: 300.0,
            details_height: 350.0,
            active_bottom_tab: BottomTab::None,
            content_drawer_height: 260.0,
            active_tab_index: 0,
            open_tabs: vec!["Map_MainMenu".to_string()],
        }
    }
}

impl LayoutSettings {
    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string("layout_settings.json") {
            if let Ok(mut settings) = serde_json::from_str::<LayoutSettings>(&content) {
                if settings.open_tabs.is_empty() {
                    settings.open_tabs = vec!["Map_MainMenu".to_string()];
                }
                return settings;
            }
        }
        let default_settings = Self::default();
        default_settings.save();
        default_settings
    }

    pub fn save(&self) {
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write("layout_settings.json", content);
        }
    }
}
