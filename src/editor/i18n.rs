use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LocaleStrings {
    pub file_menu: String,
    pub new_level: String,
    pub save_level: String,
    pub exit: String,
    pub edit_menu: String,
    pub delete_selected: String,
    pub window_menu: String,
    pub help_menu: String,
    pub quick_add: String,
    pub actors_primitives: String,
    pub cube_actor: String,
    pub sphere_actor: String,
    pub point_light: String,
    pub play: String,
    pub stop: String,
    pub world_outliner: String,
    pub clear_selection: String,
    pub details: String,
    pub actor_name: String,
    pub transform: String,
    pub location: String,
    pub rotation: String,
    pub scale: String,
    pub material_shading: String,
    pub base_color: String,
    pub light_component: String,
    pub intensity: String,
    pub no_actor_selected: String,
    pub select_actor_hint: String,
    pub content_drawer: String,
    pub output_log: String,
    pub engine_ready: String,
    pub vulkan_init: String,
    pub actors_loaded: String,
    pub project_launcher: String,
    pub recent_projects: String,
    pub new_project: String,
    pub open_project: String,
    pub project_name: String,
    pub project_path: String,
    pub create_project_btn: String,
    pub launch_project_btn: String,
    pub switch_project: String,
    pub browse_folder: String,
    pub default_projects_dir: String,
    pub change_default_dir: String,
    pub choose_level_template: String,
    pub blank_level_template: String,
    pub third_person_template: String,
    pub first_person_template: String,
    pub theme_customizer_title: String,
    pub theme_presets_heading: String,
    pub theme_subtitle: String,
    pub standard_presets: String,
    pub custom_saved_themes: String,
    pub color_matrix_heading: String,
    pub accent_highlight_color: String,
    pub main_window_bg: String,
    pub panel_dock_bg: String,
    pub inactive_button_fill: String,
    pub panel_border: String,
    pub normal_text_label_color: String,
    pub selected_active_button_text: String,
    pub highlight_text_color: String,
    pub revert_to_default_btn: String,
    pub cancel_changes_btn: String,
    pub save_as_new_theme_btn: String,
    pub save_color_btn: String,
    pub revert_color_btn: String,
    pub cancel_color_btn: String,
}

impl Default for LocaleStrings {
    fn default() -> Self {
        Self {
            file_menu: "File".to_string(),
            new_level: "New Level".to_string(),
            save_level: "Save Level".to_string(),
            exit: "Exit Engine".to_string(),
            edit_menu: "Edit".to_string(),
            delete_selected: "Delete Selected Actor".to_string(),
            window_menu: "Window".to_string(),
            help_menu: "Help".to_string(),
            quick_add: "Add".to_string(),
            actors_primitives: "Actors & Primitives".to_string(),
            cube_actor: "Cube Actor".to_string(),
            sphere_actor: "Sphere Actor".to_string(),
            point_light: "Point Light".to_string(),
            play: "Play".to_string(),
            stop: "Stop".to_string(),
            world_outliner: "World Outliner".to_string(),
            clear_selection: "Clear Selection".to_string(),
            details: "Details".to_string(),
            actor_name: "Actor Name".to_string(),
            transform: "Transform".to_string(),
            location: "Location".to_string(),
            rotation: "Rotation".to_string(),
            scale: "Scale".to_string(),
            material_shading: "Material / Shading".to_string(),
            base_color: "Base Color".to_string(),
            light_component: "Light Component".to_string(),
            intensity: "Intensity".to_string(),
            no_actor_selected: "No Actor Selected".to_string(),
            select_actor_hint: "Select an object in the World Outliner to inspect its properties.".to_string(),
            content_drawer: "Content Drawer".to_string(),
            output_log: "Output Log".to_string(),
            engine_ready: "Oxyd Engine Editor v0.0.1 Ready.".to_string(),
            vulkan_init: "WGPU Vulkan Device Initialized.".to_string(),
            actors_loaded: "4 Actors loaded in World.".to_string(),
            project_launcher: "Oxyd Engine Project Launcher".to_string(),
            recent_projects: "Recent Projects".to_string(),
            new_project: "New Project".to_string(),
            open_project: "Open Project".to_string(),
            project_name: "Project Name".to_string(),
            project_path: "Project Path".to_string(),
            create_project_btn: "Create New Project".to_string(),
            launch_project_btn: "Open Project".to_string(),
            switch_project: "Switch Project".to_string(),
            browse_folder: "Browse...".to_string(),
            default_projects_dir: "Default Projects Folder".to_string(),
            change_default_dir: "Change Default Folder".to_string(),
            choose_level_template: "Choose Starting Level Template:".to_string(),
            blank_level_template: "Blank Level".to_string(),
            third_person_template: "Third Person".to_string(),
            first_person_template: "First Person".to_string(),
            theme_customizer_title: "Theme & Color Customizer".to_string(),
            theme_presets_heading: "Theme Presets & Custom Colors".to_string(),
            theme_subtitle: "Fully customize the engine color palette in real time.".to_string(),
            standard_presets: "Standard Theme Presets:".to_string(),
            custom_saved_themes: "My Custom Saved Themes:".to_string(),
            color_matrix_heading: "Color Customization Matrix".to_string(),
            accent_highlight_color: "Accent / Highlight Color:".to_string(),
            main_window_bg: "Main Window / Viewport Background:".to_string(),
            panel_dock_bg: "Panel & Dock Background:".to_string(),
            inactive_button_fill: "Inactive Button / Box Fill:".to_string(),
            panel_border: "Panel Border (1px):".to_string(),
            normal_text_label_color: "Normal Text & Label Color:".to_string(),
            selected_active_button_text: "Selected / Active Button Text:".to_string(),
            highlight_text_color: "Highlight Text Color (on Accent):".to_string(),
            revert_to_default_btn: "Revert to Default".to_string(),
            cancel_changes_btn: "Cancel Changes".to_string(),
            save_as_new_theme_btn: "Save as New Custom Theme".to_string(),
            save_color_btn: "Save Color".to_string(),
            revert_color_btn: "Revert".to_string(),
            cancel_color_btn: "Cancel".to_string(),
        }
    }
}

pub struct I18nManager {
    pub current_lang: String,
    pub strings: LocaleStrings,
}

impl I18nManager {
    pub fn new() -> Self {
        let mut manager = Self {
            current_lang: "en".to_string(),
            strings: LocaleStrings::default(),
        };
        manager.load_language("en");
        manager
    }

    pub fn load_language(&mut self, lang_code: &str) {
        let path = format!("assets/locales/{}.json", lang_code);
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(strings) = serde_json::from_str::<LocaleStrings>(&content) {
                self.current_lang = lang_code.to_string();
                self.strings = strings;
                log::info!("Switched engine language to: {}", lang_code);
                return;
            }
        }
        log::warn!("Could not load locale file at: {}. Using default English.", path);
    }
}
