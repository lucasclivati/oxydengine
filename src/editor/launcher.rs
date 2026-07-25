use egui::{RichText, Color32, Vec2, Frame, Margin, Rounding, TextureHandle};
use crate::project::{ProjectConfig, ProjectsHistory};
use crate::editor::I18nManager;
use std::collections::HashMap;
use std::path::Path;

#[derive(PartialEq, Clone, Copy)]
pub enum LevelTemplate {
    Blank,
    ThirdPerson,
    FirstPerson,
}

pub struct LauncherState {
    pub history: ProjectsHistory,
    pub layout_settings: crate::editor::LayoutSettings,
    pub new_project_name: String,
    pub new_project_path: String,
    pub open_project_path: String,
    pub selected_project_index: Option<usize>,
    pub selected_template: LevelTemplate,
    pub open_error_msg: Option<String>,
    pub texture_cache: HashMap<String, TextureHandle>,
    pub show_theme_window: bool,
}

impl LauncherState {
    pub fn new() -> Self {
        let history = ProjectsHistory::load();
        let layout_settings = crate::editor::LayoutSettings::load();
        let default_name = "My_Oxyd_Game".to_string();
        let default_path = format!("{}/{}", history.default_projects_dir, default_name);
        let has_recent = !history.recent_projects.is_empty();

        Self {
            history,
            layout_settings,
            new_project_name: default_name,
            new_project_path: default_path,
            open_project_path: String::new(),
            selected_project_index: if has_recent { Some(0) } else { None },
            selected_template: LevelTemplate::ThirdPerson,
            open_error_msg: None,
            texture_cache: HashMap::new(),
            show_theme_window: false,
        }
    }
}

fn load_color_image(path_str: &str) -> egui::ColorImage {
    let p = Path::new(path_str);
    let target_path = if p.exists() { p } else { Path::new("logo.jpg") };

    if let Ok(img) = image::open(target_path) {
        let size = [img.width() as usize, img.height() as usize];
        let rgba = img.to_rgba8();
        return egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    }

    egui::ColorImage::new([170, 105], Color32::from_rgb(40, 45, 55))
}

pub fn show_launcher_gui(
    ctx: &egui::Context,
    state: &mut LauncherState,
    i18n: &mut I18nManager,
) -> Option<ProjectConfig> {
    let mut selected_project_to_launch: Option<ProjectConfig> = None;
    let mut switch_lang: Option<&'static str> = None;

    let accent = Color32::from_rgba_unmultiplied(
        state.layout_settings.current_theme.accent_color[0],
        state.layout_settings.current_theme.accent_color[1],
        state.layout_settings.current_theme.accent_color[2],
        state.layout_settings.current_theme.accent_color[3],
    );

    // Renderiza a janela interativa do Gerenciador de Temas quando ativada
    crate::ui::theme::show_theme_manager_window(
        ctx,
        &mut state.show_theme_window,
        &mut state.layout_settings.current_theme,
        &mut state.layout_settings.custom_themes,
        &mut state.layout_settings.theme_backup,
        &mut state.layout_settings.picker_state,
        i18n,
    );

    // Renderiza a janela interativa do Gerenciador de Contas & Tripo3D AI
    crate::editor::show_accounts_manager_window(
        ctx,
        &mut state.layout_settings.show_accounts_window,
        &mut state.layout_settings.account_settings,
        i18n,
    );

    // Top Bar estilo Oxyd Hub
    egui::TopBottomPanel::top("launcher_top_bar").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.heading(RichText::new("⚙️ Oxyd Engine").color(accent).strong());
            ui.label(RichText::new("v0.1.0").color(Color32::from_rgb(140, 145, 160)));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);

                // BOTAO ACCOUNTS (NA DIREITA DO BOTAO LANGUAGE)
                if ui.button(RichText::new("Accounts").color(accent).strong()).clicked() {
                    state.layout_settings.show_accounts_window = !state.layout_settings.show_accounts_window;
                }

                ui.add_space(8.0);
                // MENU LANGUAGE
                ui.menu_button(RichText::new("Language").color(accent).strong(), |ui| {
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        let langs = [
                            ("en", "English (US)"),
                            ("pt", "Português (BR)"),
                            ("zh", "中文 (Chinese)"),
                            ("es", "Español (Spanish)"),
                            ("hi", "हिन्दी (Hindi)"),
                            ("ar", "العربية (Arabic)"),
                            ("bn", "বাংলা (Bengali)"),
                            ("fr", "Français (French)"),
                            ("ru", "Русский (Russian)"),
                            ("ja", "日本語 (Japanese)"),
                            ("de", "Deutsch (German)"),
                            ("ko", "한국어 (Korean)"),
                            ("it", "Italiano (Italian)"),
                            ("tr", "Türkçe (Turkish)"),
                            ("vi", "Tiếng Việt (Vietnamese)"),
                            ("pl", "Polski (Polish)"),
                            ("nl", "Nederlands (Dutch)"),
                            ("uk", "Українська (Ukrainian)"),
                            ("id", "Bahasa Indonesia"),
                            ("th", "ไทย (Thai)"),
                            ("sv", "Svenska (Swedish)"),
                            ("cs", "Čeština (Czech)"),
                            ("el", "Ελληνικά (Greek)"),
                            ("ro", "Română (Romanian)"),
                            ("hu", "Magyar (Hungarian)"),
                            ("fi", "Suomi (Finnish)"),
                            ("da", "Dansk (Danish)"),
                            ("no", "Norsk (Norwegian)"),
                            ("he", "עברית (Hebrew)"),
                            ("ms", "Bahasa Melayu (Malay)"),
                        ];

                        for (code, label) in langs {
                            if ui.selectable_label(i18n.current_lang == code, label).clicked() {
                                switch_lang = Some(code);
                                ui.close_menu();
                            }
                        }
                    });
                });

                ui.add_space(8.0);
                // BOTAO THEMES (ABRE A JANELA COMPLETA DE CUSTOMIZAÇÃO DE CORES)
                if ui.button(RichText::new("Themes").color(accent).strong()).clicked() {
                    state.show_theme_window = !state.show_theme_window;
                }
            });
        });
        ui.add_space(4.0);
    });

    if let Some(lang) = switch_lang {
        i18n.load_language(lang);
    }

    let tr = &i18n.strings;

    // Central Panel: Hub Principal de Projetos
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(8.0);

        // Barra de Configuração da Pasta Padrão de Projetos
        Frame::none()
            .fill(Color32::from_rgb(30, 33, 42))
            .rounding(Rounding::same(6.0))
            .inner_margin(Margin::symmetric(14.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("{}:", tr.default_projects_dir)).color(Color32::from_rgb(230, 235, 245)).strong());
                    ui.label(RichText::new(&state.history.default_projects_dir).color(accent).strong());

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add_sized(Vec2::new(180.0, 28.0), egui::Button::new(RichText::new(format!("📁 {}", tr.change_default_dir)).color(Color32::from_rgb(230, 235, 245)).strong())).clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                let path_str = folder.to_string_lossy().to_string();
                                state.history.set_default_dir(&path_str);
                                state.new_project_path = format!("{}/{}", state.history.default_projects_dir, state.new_project_name);
                            }
                        }
                    });
                });
            });

        ui.add_space(12.0);

        ui.columns(2, |cols| {
            // Coluna Esquerda: Cartões de Previa Visual de Projetos Recentes
            cols[0].group(|ui| {
                ui.add_space(4.0);
                ui.heading(RichText::new(format!("📁 {}", tr.recent_projects)).color(Color32::from_rgb(230, 235, 245)).strong());
                ui.separator();
                ui.add_space(6.0);

                egui::ScrollArea::vertical().max_height(450.0).show(ui, |ui| {
                    if state.history.recent_projects.is_empty() {
                        ui.label(RichText::new("Nenhum projeto encontrado.").italics().color(Color32::GRAY));
                    } else {
                        ui.horizontal_wrapped(|ui| {
                            for (idx, proj) in state.history.recent_projects.iter().enumerate() {
                                let is_selected = state.selected_project_index == Some(idx);
                                
                                let bg_color = if is_selected {
                                    accent
                                } else {
                                    Color32::from_rgb(32, 35, 45)
                                };

                                let card_frame = Frame::none()
                                    .fill(bg_color)
                                    .rounding(Rounding::same(6.0))
                                    .inner_margin(Margin::same(8.0));

                                let card_res = card_frame.show(ui, |ui| {
                                    ui.set_width(170.0);
                                    ui.vertical(|ui| {
                                        let thumb_path = proj.thumbnail_path.as_deref().unwrap_or("logo.jpg").to_string();
                                        let texture = state.texture_cache.entry(proj.name.clone()).or_insert_with(|| {
                                            ctx.load_texture(
                                                format!("proj_thumb_{}", proj.name),
                                                load_color_image(&thumb_path),
                                                Default::default()
                                            )
                                        });

                                        let img_res = ui.add(
                                            egui::Image::new((texture.id(), Vec2::new(170.0, 105.0)))
                                                .sense(egui::Sense::click())
                                        );

                                        if img_res.double_clicked() {
                                            selected_project_to_launch = Some(proj.clone());
                                        }

                                        ui.add_space(6.0);
                                        let text_color = if is_selected { Color32::from_rgb(20, 22, 28) } else { Color32::WHITE };
                                        ui.label(RichText::new(&proj.name).color(text_color).strong());
                                        ui.label(RichText::new(&proj.engine_version).color(text_color));
                                        ui.add_space(8.0);

                                        if ui.add_sized(Vec2::new(154.0, 32.0), egui::Button::new(RichText::new(format!("▶ {}", tr.launch_project_btn)).strong())).clicked() {
                                            selected_project_to_launch = Some(proj.clone());
                                        }
                                    });
                                }).response;

                                if card_res.double_clicked() {
                                    selected_project_to_launch = Some(proj.clone());
                                } else if card_res.clicked() {
                                    state.selected_project_index = Some(idx);
                                }

                                ui.add_space(10.0);
                            }
                        });
                    }
                });
            });

            // Coluna Direita: Criar Novo Projeto / Abrir
            cols[1].vertical(|ui| {
                // Seção 1: Criar Novo Projeto
                ui.group(|ui| {
                    ui.add_space(4.0);
                    ui.heading(RichText::new(format!("➕ {}", tr.new_project)).color(Color32::from_rgb(230, 235, 245)).strong());
                    ui.separator();
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{}:", tr.project_name)).color(Color32::from_rgb(230, 235, 245)).strong());
                        ui.add_space(4.0);
                        if ui.text_edit_singleline(&mut state.new_project_name).changed() {
                            state.new_project_path = format!("{}/{}", state.history.default_projects_dir, state.new_project_name);
                        }
                    });

                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{}:", tr.project_path)).color(Color32::from_rgb(230, 235, 245)).strong());
                        ui.add_space(4.0);
                        ui.add_sized(Vec2::new(240.0, 24.0), egui::TextEdit::singleline(&mut state.new_project_path));
                        ui.add_space(6.0);

                        if ui.add_sized(Vec2::new(110.0, 26.0), egui::Button::new(RichText::new(format!("📁 {}", tr.browse_folder)).color(Color32::from_rgb(230, 235, 245)).strong())).clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                state.new_project_path = folder.to_string_lossy().to_string();
                            }
                        }
                    });

                    ui.add_space(8.0);

                    ui.label(RichText::new("Choose Starting Level Template:").color(Color32::from_rgb(230, 235, 245)).strong());
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut state.selected_template, LevelTemplate::Blank, "✨ Blank Level");
                        ui.selectable_value(&mut state.selected_template, LevelTemplate::ThirdPerson, "🏃 Third Person");
                        ui.selectable_value(&mut state.selected_template, LevelTemplate::FirstPerson, "🎯 First Person");
                    });

                    ui.add_space(10.0);

                    if ui.add_sized(Vec2::new(180.0, 32.0), egui::Button::new(RichText::new(format!("🚀 {}", tr.create_project_btn)).color(Color32::from_rgb(230, 235, 245)).strong())).clicked() {
                        let new_proj = ProjectConfig::new(&state.new_project_name, &state.new_project_path);
                        if let Ok(_) = new_proj.save() {
                            state.history.add_project(new_proj.clone());
                            selected_project_to_launch = Some(new_proj);
                        }
                    }
                    ui.add_space(4.0);
                });

                ui.add_space(16.0);

                // Seção 2: Abrir Projeto Existente
                ui.group(|ui| {
                    ui.add_space(4.0);
                    ui.heading(RichText::new(format!("📂 {}", tr.open_project)).color(Color32::from_rgb(230, 235, 245)).strong());
                    ui.separator();
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{}:", tr.project_path)).color(Color32::from_rgb(230, 235, 245)).strong());
                        ui.add_space(4.0);
                        ui.add_sized(Vec2::new(240.0, 24.0), egui::TextEdit::singleline(&mut state.open_project_path).hint_text("Select a folder with project.oxyd..."));
                        ui.add_space(6.0);

                        if ui.add_sized(Vec2::new(110.0, 26.0), egui::Button::new(RichText::new(format!("📁 {}", tr.browse_folder)).color(Color32::from_rgb(230, 235, 245)).strong())).clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                state.open_project_path = folder.to_string_lossy().to_string();
                                state.open_error_msg = None;
                            }
                        }
                    });

                    ui.add_space(12.0);

                    if ui.add_sized(Vec2::new(180.0, 32.0), egui::Button::new(RichText::new(format!("📂 {}", tr.launch_project_btn)).color(Color32::from_rgb(230, 235, 245)).strong())).clicked() {
                        if state.open_project_path.trim().is_empty() {
                            state.open_error_msg = Some("⚠️ Nenhuma pasta selecionada. Escolha uma pasta para abrir.".to_string());
                        } else if let Ok(loaded_proj) = ProjectConfig::load_from_dir(&state.open_project_path) {
                            state.history.add_project(loaded_proj.clone());
                            selected_project_to_launch = Some(loaded_proj);
                            state.open_error_msg = None;
                        } else {
                            state.open_error_msg = Some("⚠️ Nenhum projeto compatível do Oxyd Engine foi encontrado nesta pasta.".to_string());
                        }
                    }

                    let show_err = state.open_error_msg.is_some();
                    let opacity = ctx.animate_bool_with_time(egui::Id::new("open_proj_err_fade"), show_err, 0.3);
                    
                    if opacity > 0.0 {
                        if let Some(err_txt) = &state.open_error_msg {
                            ui.add_space(6.0);
                            let text_color = Color32::from_rgba_unmultiplied(255, 90, 80, (opacity * 255.0) as u8);
                            ui.label(RichText::new(err_txt).color(text_color).strong());
                        }
                    }

                    ui.add_space(4.0);
                });
            });
        });
    });

    selected_project_to_launch
}
