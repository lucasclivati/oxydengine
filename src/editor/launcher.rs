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
    pub new_project_name: String,
    pub new_project_path: String,
    pub open_project_path: String,
    pub selected_project_index: Option<usize>,
    pub selected_template: LevelTemplate,
    pub open_error_msg: Option<String>,
    pub texture_cache: HashMap<String, TextureHandle>,
}

impl LauncherState {
    pub fn new() -> Self {
        let history = ProjectsHistory::load();
        let default_name = "My_Oxyd_Game".to_string();
        let default_path = format!("{}/{}", history.default_projects_dir, default_name);
        let has_recent = !history.recent_projects.is_empty();

        Self {
            history,
            new_project_name: default_name,
            new_project_path: default_path,
            open_project_path: String::new(), // Inicia 100% vazio como solicitado
            selected_project_index: if has_recent { Some(0) } else { None },
            selected_template: LevelTemplate::ThirdPerson,
            open_error_msg: None,
            texture_cache: HashMap::new(),
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

    // Top Bar estilo Unreal Hub (Sem o quadrado bugado ao lado da logo!)
    egui::TopBottomPanel::top("launcher_top_bar").show(ctx, |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.heading(RichText::new("⚙️ Oxyd Engine").color(Color32::from_rgb(255, 107, 53)).strong());
            ui.label(RichText::new("v0.1.0").color(Color32::from_rgb(140, 145, 160)));

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                // REGRA CRÍTICA UNTRANSLATED MENU LANGUAGE
                ui.menu_button(RichText::new("LANGUAGE").color(Color32::from_rgb(255, 200, 50)).strong(), |ui| {
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

        // Barra de Configuração da Pasta Padrão de Projetos (Caminho Completo do Computador)
        Frame::none()
            .fill(Color32::from_rgb(30, 33, 42))
            .rounding(Rounding::same(6.0))
            .inner_margin(Margin::symmetric(14.0, 10.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("{}:", tr.default_projects_dir)).strong());
                    ui.label(RichText::new(&state.history.default_projects_dir).color(Color32::from_rgb(255, 107, 53)).strong());

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add_sized(Vec2::new(180.0, 28.0), egui::Button::new(RichText::new(format!("📁 {}", tr.change_default_dir)).strong())).clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                let path_str = folder.to_string_lossy().to_string();
                                state.history.set_default_dir(path_str);
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
                ui.heading(RichText::new(format!("📁 {}", tr.recent_projects)).color(Color32::from_rgb(220, 220, 220)).strong());
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
                                    Color32::from_rgb(0, 110, 210) // Selecionado (Azul Unreal)
                                } else {
                                    Color32::from_rgb(32, 35, 45)
                                };

                                // Cartão do Projeto (Visual Card)
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
                                        ui.label(RichText::new(&proj.name).color(Color32::WHITE).strong());
                                        ui.label(RichText::new(&proj.engine_version).color(Color32::from_rgb(180, 195, 210)));
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

            // Coluna Direita: Criar Novo Projeto (com seleção de Template de Level) / Abrir
            cols[1].vertical(|ui| {
                // Seção 1: Criar Novo Projeto
                ui.group(|ui| {
                    ui.add_space(4.0);
                    ui.heading(RichText::new(format!("➕ {}", tr.new_project)).color(Color32::from_rgb(220, 220, 220)).strong());
                    ui.separator();
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{}:", tr.project_name)).strong());
                        ui.add_space(4.0);
                        if ui.text_edit_singleline(&mut state.new_project_name).changed() {
                            state.new_project_path = format!("{}/{}", state.history.default_projects_dir, state.new_project_name);
                        }
                    });

                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{}:", tr.project_path)).strong());
                        ui.add_space(4.0);
                        ui.add_sized(Vec2::new(240.0, 24.0), egui::TextEdit::singleline(&mut state.new_project_path));
                        ui.add_space(6.0);

                        if ui.add_sized(Vec2::new(110.0, 26.0), egui::Button::new(RichText::new(format!("📁 {}", tr.browse_folder)).strong())).clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                state.new_project_path = folder.to_string_lossy().to_string();
                            }
                        }
                    });

                    ui.add_space(8.0);

                    // Seleção de Template de Level (Estilo Unreal Engine Starter Content)
                    ui.label(RichText::new("Choose Starting Level Template:").strong());
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut state.selected_template, LevelTemplate::Blank, "🏞️ Blank Level");
                        ui.selectable_value(&mut state.selected_template, LevelTemplate::ThirdPerson, "🏃 Third Person");
                        ui.selectable_value(&mut state.selected_template, LevelTemplate::FirstPerson, "🎯 First Person");
                    });

                    ui.add_space(10.0);

                    // Botão Criar Projeto Padronizado (180x32)
                    if ui.add_sized(Vec2::new(180.0, 32.0), egui::Button::new(RichText::new(format!("🚀 {}", tr.create_project_btn)).strong())).clicked() {
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
                    ui.heading(RichText::new(format!("📂 {}", tr.open_project)).color(Color32::from_rgb(220, 220, 220)).strong());
                    ui.separator();
                    ui.add_space(6.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{}:", tr.project_path)).strong());
                        ui.add_space(4.0);
                        ui.add_sized(Vec2::new(240.0, 24.0), egui::TextEdit::singleline(&mut state.open_project_path).hint_text("Select a folder with project.oxyd..."));
                        ui.add_space(6.0);

                        if ui.add_sized(Vec2::new(110.0, 26.0), egui::Button::new(RichText::new(format!("📁 {}", tr.browse_folder)).strong())).clicked() {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                state.open_project_path = folder.to_string_lossy().to_string();
                                state.open_error_msg = None;
                            }
                        }
                    });

                    ui.add_space(12.0);

                    // Botão Abrir Projeto Padronizado (180x32)
                    if ui.add_sized(Vec2::new(180.0, 32.0), egui::Button::new(RichText::new(format!("📂 {}", tr.launch_project_btn)).strong())).clicked() {
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

                    // AVISO SUAVE COM TRANSIÇÃO FADE-IN DE 0.3s SE PROJETO INCOMPATÍVEL
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
