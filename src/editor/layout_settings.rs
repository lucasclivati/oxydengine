use serde::{Serialize, Deserialize};
use std::fs;
use crate::ui::theme::CustomTheme;

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
    pub location_snap: f32,
    pub rotation_snap: f32,
    pub scale_snap: f32,
    pub show_map_assets: bool,
    pub show_details: bool,
    #[serde(default)]
    pub show_theme_window: bool,
    #[serde(default)]
    pub show_accounts_window: bool,
    #[serde(default)]
    pub account_settings: crate::editor::AccountSettings,
    #[serde(default)]
    pub current_theme: CustomTheme,
    #[serde(default)]
    pub custom_themes: Vec<CustomTheme>,
    #[serde(skip)]
    pub theme_backup: Option<CustomTheme>,
    #[serde(skip)]
    pub picker_state: crate::ui::theme::ColorPickerPopupState,
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
            location_snap: 10.0,
            rotation_snap: 10.0,
            scale_snap: 0.25,
            show_map_assets: true,
            show_details: true,
            show_theme_window: false,
            show_accounts_window: false,
            account_settings: crate::editor::AccountSettings::default(),
            current_theme: CustomTheme::oxyd_gold(),
            custom_themes: Vec::new(),
            theme_backup: None,
            picker_state: crate::ui::theme::ColorPickerPopupState::default(),
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
                if settings.location_snap <= 0.0 { settings.location_snap = 10.0; }
                if settings.rotation_snap <= 0.0 { settings.rotation_snap = 10.0; }
                if settings.scale_snap <= 0.0 { settings.scale_snap = 0.25; }
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
