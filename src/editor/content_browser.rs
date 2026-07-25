use egui::{RichText, Color32, Vec2, Frame, Rounding, Margin};
use crate::editor::{I18nManager, LayoutSettings, BottomTab};
use crate::scene::{World, PrimitiveType};
use glam::Vec3;
use std::fs;
use std::path::{Path, PathBuf};

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
                ("[INFO] Type 'help' for a list of console commands.".to_string(), Color32::from_rgb(100, 180, 255)),
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
                self.logs.push(("Commands: spawn <cube|sphere|light> [name], delete <name>, play, stop, setlang <en|pt|zh>, clear".to_string(), Color32::YELLOW));
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

    // 1. Painel Inferior Deslizante (Content Drawer NAVEGADOR REAL DE PASTAS E ARQUIVOS)
    match layout.active_bottom_tab {
        BottomTab::ContentDrawer => {
            egui::TopBottomPanel::bottom("oxyd_content_drawer_drawer")
                .resizable(true)
                .default_height(layout.content_drawer_height)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(RichText::new(format!("📁 {}", tr.content_drawer)).color(Color32::from_rgb(220, 220, 220)).strong());
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("❌").clicked() {
                                layout.active_bottom_tab = BottomTab::None;
                                layout.save();
                            }
                        });
                    });
                    ui.separator();

                    ui.columns(2, |cols| {
                        // Árvore de Pastas Real à Esquerda (Interativa!)
                        cols[0].group(|ui| {
                            ui.set_max_width(210.0);
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.label(RichText::new("⭐ Favorites").strong());
                                ui.indent("fav_indent", |ui| {
                                    ui.label("📁 AlchemySurvival57old");
                                });

                                ui.add_space(6.0);

                                ui.label(RichText::new("📦 Content Folders").strong());
                                
                                let folders = ["Blueprints", "Maps", "Materials", "Meshes", "Textures", "Decals", "VFX_Niagara"];
                                for f in folders {
                                    let is_sel = console_state.selected_folder == f;
                                    let text_color = if is_sel { Color32::from_rgb(255, 107, 53) } else { Color32::from_rgb(200, 210, 220) };
                                    
                                    if ui.selectable_label(is_sel, RichText::new(format!("📁 {}", f)).color(text_color).strong()).clicked() {
                                        console_state.selected_folder = f.to_string();
                                    }
                                }
                            });
                        });

                        // Grade de Assets Reais do Disco à Direita
                        cols[1].vertical(|ui| {
                            let curr_folder = console_state.selected_folder.clone();

                            ui.horizontal(|ui| {
                                if ui.button("➕ Add Asset").clicked() {}
                                if ui.button("📥 Import").clicked() {}
                                if ui.button("💾 Save All").clicked() {}
                                ui.separator();
                                ui.label(RichText::new(format!("Path: Content > {}", curr_folder)).strong().color(Color32::from_rgb(255, 107, 53)));
                            });

                            ui.separator();

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    // Escanear arquivos reais do disco
                                    let dir_path = format!("projects/AlchemySurvival57old/Content/{}", curr_folder);
                                    let unreal_dir = format!(r"C:\Users\lukstrike\Documents\Unreal Projects\AlchemySurvival57old\Content\{}", curr_folder);

                                    let target_path = if Path::new(&dir_path).exists() {
                                        dir_path
                                    } else {
                                        unreal_dir
                                    };

                                    let mut found_items: Vec<(String, String, &'static str)> = Vec::new(); // (Nome, Tipo, Ícone)

                                    if let Ok(entries) = fs::read_dir(&target_path) {
                                        for entry in entries.flatten() {
                                            let file_name = entry.file_name().to_string_lossy().to_string();
                                            let ext = entry.path().extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
                                            
                                            let (icon, item_type) = match ext.as_str() {
                                                "uasset" => ("⚡", "Blueprint Class"),
                                                "oxydlevel" | "umap" => ("🗺️", "World Level"),
                                                "png" | "jpg" => ("🖼️", "Texture2D"),
                                                "obj" | "fbx" => ("📦", "StaticMesh"),
                                                _ => ("📄", "Asset"),
                                            };

                                            found_items.push((file_name, item_type.to_string(), icon));
                                        }
                                    }

                                    if found_items.is_empty() {
                                        // Fallback de itens padronizados se a pasta for recém criada
                                        match curr_folder.as_str() {
                                            "Maps" => {
                                                found_items.push(("Map_MainMenu.oxydlevel".to_string(), "World Level".to_string(), "🗺️"));
                                                found_items.push(("Map_Lobby.oxydlevel".to_string(), "World Level".to_string(), "🗺️"));
                                                found_items.push(("Map_Transition.oxydlevel".to_string(), "World Level".to_string(), "🗺️"));
                                                found_items.push(("Map_CityZombieSurvival.oxydlevel".to_string(), "World Level".to_string(), "🗺️"));
                                            }
                                            "Blueprints" => {
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
                                        let card_res = Frame::none()
                                            .fill(Color32::from_rgb(38, 42, 54))
                                            .rounding(Rounding::same(6.0))
                                            .inner_margin(Margin::same(8.0))
                                            .show(ui, |ui| {
                                                ui.set_width(120.0);
                                                ui.vertical_centered(|ui| {
                                                    ui.label(RichText::new(icon).font(egui::FontId::proportional(34.0)));
                                                    let display_name = name.trim_end_matches(".uasset").trim_end_matches(".oxydlevel");
                                                    ui.label(RichText::new(display_name).strong().color(Color32::WHITE));
                                                    ui.label(RichText::new(&item_type).small().color(Color32::GRAY));
                                                });
                                            }).response;

                                        // Duplo-clique em qualquer mapa no Content Drawer carrega a fase no editor
                                        if card_res.double_clicked() {
                                            if name.contains("MainMenu") {
                                                *world = World::new_main_menu_scene();
                                                console_state.logs.push(("Loaded Level: Map_MainMenu".to_string(), Color32::GREEN));
                                            } else if name.contains("Lobby") {
                                                *world = World::new_third_person_level();
                                                console_state.logs.push(("Loaded Level: Map_Lobby".to_string(), Color32::GREEN));
                                            } else if name.contains("City") || name.contains("Zombie") {
                                                *world = World::new_first_person_level();
                                                console_state.logs.push(("Loaded Level: Map_CityZombieSurvival".to_string(), Color32::GREEN));
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
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(RichText::new(format!("📋 {}", tr.output_log)).color(Color32::from_rgb(220, 220, 220)).strong());
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

    // 2. Barra de Abas Conectadas Inferior Limpa (Estilo Unreal Engine 5.8)
    egui::TopBottomPanel::bottom("oxyd_bottom_docking_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            let is_drawer_open = layout.active_bottom_tab == BottomTab::ContentDrawer;
            let drawer_bg = if is_drawer_open { Color32::from_rgb(48, 54, 70) } else { Color32::from_rgb(24, 26, 34) };
            let drawer_text = if is_drawer_open { Color32::WHITE } else { Color32::from_rgb(170, 180, 195) };

            let drawer_tab_res = Frame::none()
                .fill(drawer_bg)
                .rounding(Rounding::same(3.0))
                .inner_margin(Margin::symmetric(12.0, 6.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("📁 Content Drawer").color(drawer_text).strong())
                }).response;

            if drawer_tab_res.clicked() {
                layout.active_bottom_tab = if is_drawer_open { BottomTab::None } else { BottomTab::ContentDrawer };
                layout.save();
            }

            ui.add_space(2.0);

            let is_log_open = layout.active_bottom_tab == BottomTab::OutputLog;
            let log_bg = if is_log_open { Color32::from_rgb(48, 54, 70) } else { Color32::from_rgb(24, 26, 34) };
            let log_text = if is_log_open { Color32::WHITE } else { Color32::from_rgb(170, 180, 195) };

            let log_tab_res = Frame::none()
                .fill(log_bg)
                .rounding(Rounding::same(3.0))
                .inner_margin(Margin::symmetric(12.0, 6.0))
                .show(ui, |ui| {
                    ui.label(RichText::new("📋 Output Log").color(log_text).strong())
                }).response;

            if log_tab_res.clicked() {
                layout.active_bottom_tab = if is_log_open { BottomTab::None } else { BottomTab::OutputLog };
                layout.save();
            }

            ui.separator();

            ui.label(RichText::new(">_ Cmd ⏷").strong().color(Color32::from_rgb(180, 190, 210)));
            ui.add_space(4.0);

            let response = ui.add_sized(
                Vec2::new(260.0, 20.0),
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
