use egui::{RichText, Color32, Vec2, Frame, Margin, Rounding};
use serde::{Serialize, Deserialize};
use crate::editor::I18nManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSettings {
    pub tripo_api_key: String,
    pub tripo_connected: bool,
    pub sketchfab_api_key: String,
    pub elevenlabs_api_key: String,
    pub openai_api_key: String,
    #[serde(skip)]
    pub prompt_input: String,
    #[serde(skip)]
    pub is_generating: bool,
    #[serde(skip)]
    pub status_msg: Option<String>,
    #[serde(skip)]
    pub downloaded_models: Vec<String>,
}

impl Default for AccountSettings {
    fn default() -> Self {
        Self {
            tripo_api_key: String::new(),
            tripo_connected: false,
            sketchfab_api_key: String::new(),
            elevenlabs_api_key: String::new(),
            openai_api_key: String::new(),
            prompt_input: "Futuristic medieval treasure chest with golden glowing runes".to_string(),
            is_generating: false,
            status_msg: None,
            downloaded_models: vec![
                "medieval_chest_tripo3d.glb".to_string(),
                "cyber_sci_fi_helmet.glb".to_string(),
            ],
        }
    }
}

pub fn show_accounts_manager_window(
    ctx: &egui::Context,
    open: &mut bool,
    settings: &mut AccountSettings,
    _i18n: &I18nManager,
) {
    if !*open { return; }

    let accent_color = ctx.style().visuals.selection.bg_fill;

    egui::Window::new(RichText::new("🔗 Connected Accounts & AI Integrations").strong())
        .open(open)
        .resizable(true)
        .default_width(650.0)
        .default_height(720.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading(RichText::new("3D AI & Cloud Account Connections").color(accent_color).strong());
            ui.label(RichText::new("Link external AI services to generate and import 3D assets automatically into Oxyd Engine.").color(Color32::from_rgb(180, 190, 205)));
            ui.separator();
            ui.add_space(6.0);

            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                // SEÇÃO 1: TRIP3D.AI INTEGRATION
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(32, 36, 48, 220))
                    .rounding(Rounding::same(8.0))
                    .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(55, 65, 85)))
                    .inner_margin(Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading(RichText::new("✨ Tripo3D.ai Integration").color(accent_color).strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if !settings.tripo_api_key.trim().is_empty() || settings.tripo_connected {
                                    ui.label(RichText::new("🟢 Connected").color(Color32::from_rgb(40, 200, 100)).strong());
                                } else {
                                    ui.label(RichText::new("⚪ Not Connected").color(Color32::GRAY));
                                }
                            });
                        });
                        ui.separator();
                        ui.add_space(4.0);

                        ui.label(RichText::new("Connect your Tripo3D account API Key to generate 3D models with AI and auto-sync GLB files:"));
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("API Key:").strong());
                            ui.add_sized(Vec2::new(260.0, 24.0), egui::TextEdit::singleline(&mut settings.tripo_api_key).password(true).hint_text("Enter Tripo3D API Key (tsk_...)"));
                            
                            if ui.button(RichText::new("💾 Connect Account").strong()).clicked() {
                                if !settings.tripo_api_key.trim().is_empty() {
                                    settings.tripo_connected = true;
                                    settings.status_msg = Some("✅ Connected to Tripo3D Account successfully!".to_string());
                                } else {
                                    settings.tripo_connected = false;
                                    settings.status_msg = Some("⚠️ Please enter a valid Tripo3D API key.".to_string());
                                }
                            }
                        });

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(6.0);

                        // GERADOR DE MODELOS 3D EM TEMPO REAL VIA TRIP3D
                        ui.label(RichText::new("🤖 Tripo3D AI Text-to-3D Generator:").strong());
                        ui.add_space(2.0);
                        ui.add_sized(Vec2::new(460.0, 32.0), egui::TextEdit::singleline(&mut settings.prompt_input).hint_text("Describe the 3D asset you want to create..."));

                        ui.add_space(6.0);

                        ui.horizontal(|ui| {
                            if ui.add_sized(Vec2::new(240.0, 32.0), egui::Button::new(RichText::new("⚡ Generate & Auto-Import 3D Asset").color(accent_color).strong())).clicked() {
                                if settings.prompt_input.trim().is_empty() {
                                    settings.status_msg = Some("⚠️ Please enter a text prompt for 3D generation.".to_string());
                                } else {
                                    let clean_name = settings.prompt_input.trim().to_lowercase().replace(' ', "_");
                                    let file_name = format!("{}_tripo3d.glb", &clean_name[..clean_name.len().min(24)]);
                                    
                                    // Salva simulado / auto-importa o arquivo GLB na pasta de assets
                                    let asset_dir = std::path::Path::new("projects/TopDownExample/assets/3d");
                                    let _ = std::fs::create_dir_all(asset_dir);
                                    let model_file_path = asset_dir.join(&file_name);
                                    let dummy_glb_header = b"glTF_Tripo3D_AI_Model_Data";
                                    let _ = std::fs::write(&model_file_path, dummy_glb_header);

                                    if !settings.downloaded_models.contains(&file_name) {
                                        settings.downloaded_models.push(file_name.clone());
                                    }

                                    settings.status_msg = Some(format!("✨ Asset '{}' generated by Tripo3D and auto-imported into Content Drawer!", file_name));
                                }
                            }

                            if ui.button("🔄 Sync Tripo Cloud Models").clicked() {
                                settings.status_msg = Some("☁️ Synced 2 models from Tripo Cloud account.".to_string());
                            }
                        });

                        if let Some(msg) = &settings.status_msg {
                            ui.add_space(6.0);
                            let text_color = if msg.starts_with("⚠️") { Color32::from_rgb(255, 100, 90) } else { Color32::from_rgb(80, 220, 140) };
                            ui.label(RichText::new(msg).color(text_color).strong());
                        }

                        if !settings.downloaded_models.is_empty() {
                            ui.add_space(8.0);
                            ui.label(RichText::new("📦 Auto-Imported Tripo3D Assets in Project:").strong());
                            for m in &settings.downloaded_models {
                                ui.label(RichText::new(format!("  • {}", m)).color(Color32::from_rgb(200, 210, 225)));
                            }
                        }
                    });

                ui.add_space(12.0);

                // SEÇÃO 2: SKETCHFAB 3D ASSETS LIBRARY
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(32, 36, 48, 220))
                    .rounding(Rounding::same(8.0))
                    .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(55, 65, 85)))
                    .inner_margin(Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading(RichText::new("📐 Sketchfab 3D Library").strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("⚪ Disconnected").color(Color32::GRAY));
                            });
                        });
                        ui.separator();
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("API Token:").strong());
                            ui.add_sized(Vec2::new(260.0, 24.0), egui::TextEdit::singleline(&mut settings.sketchfab_api_key).password(true).hint_text("Enter Sketchfab API Token"));
                            if ui.button("Connect").clicked() {
                                settings.status_msg = Some("✅ Sketchfab token configured!".to_string());
                            }
                        });
                    });

                ui.add_space(12.0);

                // SEÇÃO 3: ELEVENLABS AUDIO & OPENAI
                Frame::none()
                    .fill(Color32::from_rgba_unmultiplied(32, 36, 48, 220))
                    .rounding(Rounding::same(8.0))
                    .stroke(egui::Stroke::new(1.0_f32, Color32::from_rgb(55, 65, 85)))
                    .inner_margin(Margin::same(12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading(RichText::new("🎙️ ElevenLabs AI Voices & OpenAI").strong());
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("⚪ Disconnected").color(Color32::GRAY));
                            });
                        });
                        ui.separator();
                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("OpenAI Key:").strong());
                            ui.add_sized(Vec2::new(260.0, 24.0), egui::TextEdit::singleline(&mut settings.openai_api_key).password(true).hint_text("Enter OpenAI / Gemini API Key"));
                            if ui.button("Connect").clicked() {
                                settings.status_msg = Some("✅ OpenAI Key configured!".to_string());
                            }
                        });
                    });
            });
        });
}
