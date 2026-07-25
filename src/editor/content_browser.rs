use egui::{RichText, Color32, Vec2, Frame, Rounding, Margin, Stroke};
use crate::editor::{I18nManager, LayoutSettings, BottomTab};
use crate::scene::{World, PrimitiveType};
use glam::Vec3;
use std::fs;

pub struct ConsoleState {
    pub input_buffer: String,
    pub logs: Vec<(String, Color32)>,
    pub selected_folder: String,
}

impl ConsoleState {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            logs: vec![
                ("[INFO] Oxyd Engine v0.0.1 Console Ready.".to_string(), Color32::from_rgb(80, 220, 100)),
                ("[INFO] Type 'help' for a list of console commands.".to_string(), Color32::from_rgb(245, 158, 11)),
            ],
            selected_folder: "Maps".to_string(),
        }
    }

    pub fn execute(&mut self, cmd: &str, world: &mut World, i18n: &mut I18nManager) {
        let trimmed = cmd.trim();
        if trimmed.is_empty() { return; }

        self.logs.push((format!("> {}", trimmed), Color32::WHITE));

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        match parts[0].to_lowercase().as_str() {
            "help" => {
                self.logs.push(("Commands: spawn <cube|sphere|light> [name], delete <name>, play, stop, setlang <en|pt|zh>, clear".to_string(), Color32::from_rgb(245, 158, 11)));
            }
            "clear" => {
                self.logs.clear();
            }
            "play" => {
                world.is_playing = true;
                self.logs.push(("Simulation started (Play Mode).".to_string(), Color32::GREEN));
            }
            "stop" => {
                world.is_playing = false;
                self.logs.push(("Simulation stopped.".to_string(), Color32::LIGHT_RED));
            }
            "spawn" => {
                if parts.len() >= 2 {
                    let p_type = match parts[1].to_lowercase().as_str() {
                        "cube" => PrimitiveType::Cube,
                        "sphere" => PrimitiveType::Sphere,
                        "light" => PrimitiveType::PointLight,
                        _ => PrimitiveType::Cube,
                    };
                    let name = if parts.len() >= 3 { parts[2] } else { "Spawned_Actor" };
                    world.add_actor(name, p_type, Vec3::ZERO, [0.8, 0.4, 0.1, 1.0]);
                    self.logs.push((format!("Successfully spawned actor: {}", name), Color32::GREEN));
                } else {
                    self.logs.push(("Usage: spawn <cube|sphere|light> [name]".to_string(), Color32::LIGHT_RED));
                }
            }
            "delete" => {
                if parts.len() >= 2 {
                    let target_name = parts[1];
                    let initial_count = world.actors.len();
                    world.actors.retain(|a| a.name != target_name);
                    if world.actors.len() < initial_count {
                        self.logs.push((format!("Deleted actor: {}", target_name), Color32::GREEN));
                    } else {
                        self.logs.push((format!("Actor not found: {}", target_name), Color32::LIGHT_RED));
                    }
                }
            }
            "setlang" => {
                if parts.len() >= 2 {
                    i18n.load_language(parts[1]);
                    self.logs.push((format!("Switched engine language to: {}", parts[1]), Color32::GREEN));
                }
            }
            _ => {
                self.logs.push((format!("Unknown command: '{}'. Type 'help' for available commands.", parts[0]), Color32::LIGHT_RED));
            }
        }
    }
}

pub fn show_content_browser_and_log(
    ctx: &egui::Context,
    i18n: &mut I18nManager,
    layout: &mut LayoutSettings,
    world: &mut World,
    console_state: &mut ConsoleState,
) {
    let tr = &i18n.strings;

    // 1. Content Drawer & Output Log Panel (Fundo Cinza Escuro #14161C)
    match layout.active_bottom_tab {
        BottomTab::ContentDrawer => {
            egui::TopBottomPanel::bottom("oxyd_content_drawer_drawer")
                .resizable(true)
                .default_height(layout.content_drawer_height)
                .frame(
                    Frame::none()
                        .fill(Color32::from_rgb(20, 22, 28))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(50, 55, 68)))
                        .inner_margin(Margin::same(8.0))
                )
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(RichText::new(format!("📁 {}", tr.content_drawer)).color(Color32::from_rgb(240, 240, 240)).strong());
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("❌").clicked() {
                                layout.active_bottom_tab = BottomTab::None;
                                layout.save();
                            }
                        });
                    });
                    ui.separator();

                    ui.horizontal(|ui| {
                        // Painel de Pastas à Esquerda com SelectableLabel direct Sensed Clicks
                        ui.vertical(|ui| {
                            ui.set_width(180.0);
                            ui.label(RichText::new("📦 Content Folders").strong().color(Color32::from_rgb(170, 180, 195)));
                            ui.add_space(6.0);
                            
                            let folders = ["Actors", "Maps", "Materials", "Meshes", "Textures", "Decals", "VFX"];

                            for f in folders {
                                let is_sel = console_state.selected_folder == f;
                                let text_color = if is_sel { Color32::WHITE } else { Color32::from_rgb(210, 215, 225) };
                                let label = RichText::new(format!("📁  {}", f)).color(text_color).strong();

                                let sel_btn = ui.add_sized(
                                    Vec2::new(170.0, 28.0),
                                    egui::SelectableLabel::new(is_sel, label)
                                );

                                if sel_btn.clicked() {
                                    console_state.selected_folder = f.to_string();
                                }
                                ui.add_space(2.0);
                            }
                        });

                        ui.separator();

                        // Grade de Assets Reais à Direita
                        ui.vertical(|ui| {
                            let curr_folder = console_state.selected_folder.clone();

                            ui.horizontal(|ui| {
                                if ui.button("➕ Add").clicked() {}
                                if ui.button("📥 Import").clicked() {}
                                if ui.button("💾 Save All").clicked() {}
                                ui.separator();
                                ui.label(RichText::new(format!("All > Content > TopDownExample > {}", curr_folder)).strong().color(Color32::from_rgb(245, 158, 11)));
                            });

                            ui.separator();

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    let dir_path = format!("projects/TopDownExample/Content/{}", curr_folder);

                                    let target_path = dir_path;

                                    let mut found_items: Vec<(String, String, &'static str)> = Vec::new();

                                    if let Ok(entries) = fs::read_dir(&target_path) {
                                        for entry in entries.flatten() {
                                            let file_name = entry.file_name().to_string_lossy().to_string();
                                            let ext = entry.path().extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
                                            
                                            let (icon, item_type) = match ext.as_str() {
                                                "uasset" => ("⚡", "Actor Class"),
                                                "oxydlevel" | "umap" => ("🗺️", "World Level"),
                                                "png" | "jpg" => ("🖼️", "Texture2D"),
                                                "obj" | "fbx" => ("📦", "StaticMesh"),
                                                _ => ("📄", "Asset"),
                                            };

                                            found_items.push((file_name, item_type.to_string(), icon));
                                        }
                                    }

                                    if found_items.is_empty() {
                                        match curr_folder.as_str() {
                                            "Maps" => {
                                                found_items.push(("Map_MainMenu.oxydlevel".to_string(), "World Level".to_string(), "🗺️"));
                                                found_items.push(("Map_Lobby.oxydlevel".to_string(), "World Level".to_string(), "🗺️"));
                                                found_items.push(("Map_Transition.oxydlevel".to_string(), "World Level".to_string(), "🗺️"));
                                                found_items.push(("Map_CityZombieSurvival.oxydlevel".to_string(), "World Level".to_string(), "🗺️"));
                                            }
                                            "Actors" => {
                                                found_items.push(("WBP_MainMenu.uasset".to_string(), "Widget Blueprint".to_string(), "⚡"));
                                                found_items.push(("BP_TopDownGameMode.uasset".to_string(), "GameMode Base".to_string(), "⚙️"));
                                                found_items.push(("BP_PlayerCharacter.uasset".to_string(), "Character Pawn".to_string(), "🏃"));
                                            }
                                            "Materials" => {
                                                found_items.push(("M_AlchemicalRust.uasset".to_string(), "Material".to_string(), "🎨"));
                                                found_items.push(("M_TerrainGrass.uasset".to_string(), "Material".to_string(), "🎨"));
                                            }
                                            _ => {
                                                found_items.push((format!("Default_{}.uasset", curr_folder), "Asset".to_string(), "📄"));
                                            }
                                        }
                                    }

                                    for (name, item_type, icon) in found_items {
                                        let display_name = name.trim_end_matches(".uasset").trim_end_matches(".oxydlevel").to_string();

                                        let card_btn = ui.add_sized(
                                            Vec2::new(130.0, 105.0),
                                            egui::Button::new(
                                                RichText::new(format!("{}\n{}\n{}", icon, display_name, item_type))
                                                    .strong()
                                                    .color(Color32::WHITE)
                                            ).fill(Color32::from_rgb(32, 36, 46))
                                             .stroke(Stroke::new(1.0_f32, Color32::from_rgb(50, 55, 68)))
                                             .rounding(Rounding::same(6.0))
                                        );

                                        if card_btn.clicked() || card_btn.double_clicked() {
                                            if name.contains("MainMenu") {
                                                *world = World::new_main_menu_scene();
                                                console_state.logs.push(("[SUCCESS] Loaded Level: Map_MainMenu".to_string(), Color32::GREEN));
                                                layout.open_tabs = vec!["Map_MainMenu".to_string()];
                                                layout.active_tab_index = 0;
                                                layout.save();
                                            } else if name.contains("Lobby") {
                                                *world = World::new_third_person_level();
                                                console_state.logs.push(("[SUCCESS] Loaded Level: Map_Lobby".to_string(), Color32::GREEN));
                                                layout.open_tabs = vec!["Map_Lobby".to_string()];
                                                layout.active_tab_index = 0;
                                                layout.save();
                                            } else if name.contains("City") || name.contains("Zombie") {
                                                *world = World::new_first_person_level();
                                                console_state.logs.push(("[SUCCESS] Loaded Level: Map_CityZombieSurvival".to_string(), Color32::GREEN));
                                                layout.open_tabs = vec!["Map_CityZombieSurvival".to_string()];
                                                layout.active_tab_index = 0;
                                                layout.save();
                                            } else if name.contains("Transition") {
                                                *world = World::new_default_scene();
                                                console_state.logs.push(("[SUCCESS] Loaded Level: Map_Transition".to_string(), Color32::GREEN));
                                                layout.open_tabs = vec!["Map_Transition".to_string()];
                                                layout.active_tab_index = 0;
                                                layout.save();
                                            }
                                        }

                                        ui.add_space(8.0);
                                    }
                                });
                            });
                        });
                    });
                });
        }
        BottomTab::OutputLog => {
            egui::TopBottomPanel::bottom("oxyd_output_log_drawer")
                .resizable(true)
                .default_height(200.0)
                .frame(
                    Frame::none()
                        .fill(Color32::from_rgb(20, 22, 28))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(50, 55, 68)))
                        .inner_margin(Margin::same(8.0))
                )
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(RichText::new(format!("📋 {}", tr.output_log)).color(Color32::from_rgb(240, 240, 240)).strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("❌").clicked() {
                                layout.active_bottom_tab = BottomTab::None;
                                layout.save();
                            }
                        });
                    });
                    ui.separator();

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (log_line, color) in &console_state.logs {
                            ui.label(RichText::new(log_line).color(*color));
                        }
                    });
                });
        }
        BottomTab::None => {}
    }

    // 2. BARRA INFERIOR COM BOTOES DIRETOS DO EGUI PARA CLIQUE 100% GARANTIDO
    egui::TopBottomPanel::bottom("oxyd_bottom_docking_bar")
        .frame(
            Frame::none()
                .fill(Color32::from_rgb(20, 22, 28))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(50, 55, 68)))
                .inner_margin(Margin::symmetric(10.0, 6.0))
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let is_drawer_open = layout.active_bottom_tab == BottomTab::ContentDrawer;
                let drawer_bg = if is_drawer_open { Color32::from_rgb(45, 50, 65) } else { Color32::from_rgb(26, 28, 36) };
                let drawer_stroke = if is_drawer_open { Stroke::new(1.0_f32, Color32::from_rgb(245, 158, 11)) } else { Stroke::new(1.0_f32, Color32::from_rgb(50, 55, 68)) };
                let drawer_text = if is_drawer_open { Color32::WHITE } else { Color32::from_rgb(190, 195, 210) };

                // USAR egui::Button DIRETO GARANTE CLIQUE DO MOUSE 100% CONFIÁVEL
                let btn_drawer = egui::Button::new(RichText::new("📁 Content Drawer").color(drawer_text).strong())
                    .fill(drawer_bg)
                    .stroke(drawer_stroke)
                    .rounding(Rounding::same(4.0));

                if ui.add(btn_drawer).clicked() {
                    layout.active_bottom_tab = if is_drawer_open { BottomTab::None } else { BottomTab::ContentDrawer };
                    layout.save();
                }

                ui.add_space(4.0);

                let is_log_open = layout.active_bottom_tab == BottomTab::OutputLog;
                let log_bg = if is_log_open { Color32::from_rgb(45, 50, 65) } else { Color32::from_rgb(26, 28, 36) };
                let log_stroke = if is_log_open { Stroke::new(1.0_f32, Color32::from_rgb(245, 158, 11)) } else { Stroke::new(1.0_f32, Color32::from_rgb(50, 55, 68)) };
                let log_text = if is_log_open { Color32::WHITE } else { Color32::from_rgb(190, 195, 210) };

                let btn_log = egui::Button::new(RichText::new("📋 Output Log").color(log_text).strong())
                    .fill(log_bg)
                    .stroke(log_stroke)
                    .rounding(Rounding::same(4.0));

                if ui.add(btn_log).clicked() {
                    layout.active_bottom_tab = if is_log_open { BottomTab::None } else { BottomTab::OutputLog };
                    layout.save();
                }

                ui.separator();

                ui.label(RichText::new(">_ Cmd").strong().color(Color32::from_rgb(190, 195, 210)));
                ui.add_space(4.0);

                let response = ui.add_sized(
                    Vec2::new(280.0, 22.0),
                    egui::TextEdit::singleline(&mut console_state.input_buffer).hint_text("Enter Console Command...")
                );

                if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let cmd = console_state.input_buffer.clone();
                    console_state.input_buffer.clear();
                    console_state.execute(&cmd, world, i18n);
                    response.request_focus();
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("💾 All Saved").color(Color32::from_rgb(80, 220, 100)));
                });
            });
        });
}
