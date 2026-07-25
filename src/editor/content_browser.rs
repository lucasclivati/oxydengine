use egui::{RichText, Color32, Vec2, Frame, Rounding, Margin};
use crate::editor::{I18nManager, LayoutSettings, BottomTab};
use crate::scene::{World, PrimitiveType};
use glam::Vec3;

pub struct ConsoleState {
    pub input_buffer: String,
    pub logs: Vec<(String, Color32)>,
}

impl ConsoleState {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            logs: vec![
                ("[INFO] Oxyd Engine v0.0.1 Console Ready.".to_string(), Color32::from_rgb(80, 220, 100)),
                ("[INFO] Type 'help' for a list of console commands.".to_string(), Color32::from_rgb(100, 180, 255)),
            ],
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

    // 1. Painel Inferior Deslizante (Gaveta Compartilhada entre Content Drawer e Output Log estilo Unreal 5.8)
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
                        // Árvore de Pastas à Esquerda (Content Tree)
                        cols[0].group(|ui| {
                            ui.set_max_width(200.0);
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.label(RichText::new("⭐ Favorites").strong());
                                ui.indent("fav_indent", |ui| {
                                    ui.label("📁 OxydTopDownRoguelike");
                                });

                                ui.add_space(4.0);

                                ui.label(RichText::new("📦 All / Content").strong());
                                ui.indent("content_indent", |ui| {
                                    ui.label("📁 Blueprints");
                                    ui.label("📁 Maps");
                                    ui.label("📁 Materials");
                                    ui.label("📁 Meshes");
                                    ui.label("📁 Textures");
                                    ui.label("📁 VFX_Niagara");
                                });
                            });
                        });

                        // Grade de Assets à Direita
                        cols[1].vertical(|ui| {
                            ui.horizontal(|ui| {
                                if ui.button("➕ Add").clicked() {}
                                if ui.button("📥 Import").clicked() {}
                                if ui.button("💾 Save All").clicked() {}
                                ui.separator();
                                ui.label("Path: All > Content > Maps");
                            });

                            ui.separator();

                            egui::ScrollArea::vertical().show(ui, |ui| {
                                ui.horizontal_wrapped(|ui| {
                                    Frame::none()
                                        .fill(Color32::from_rgb(38, 42, 54))
                                        .rounding(Rounding::same(6.0))
                                        .inner_margin(Margin::same(8.0))
                                        .show(ui, |ui| {
                                            ui.set_width(110.0);
                                            ui.vertical_centered(|ui| {
                                                ui.label(RichText::new("🗺️").font(egui::FontId::proportional(36.0)));
                                                ui.label(RichText::new("Map_Main").strong());
                                                ui.label(RichText::new("Level").small().color(Color32::GRAY));
                                            });
                                        });

                                    ui.add_space(8.0);

                                    Frame::none()
                                        .fill(Color32::from_rgb(38, 42, 54))
                                        .rounding(Rounding::same(6.0))
                                        .inner_margin(Margin::same(8.0))
                                        .show(ui, |ui| {
                                            ui.set_width(110.0);
                                            ui.vertical_centered(|ui| {
                                                ui.label(RichText::new("📦").font(egui::FontId::proportional(36.0)));
                                                ui.label(RichText::new("SM_Cube").strong());
                                                ui.label(RichText::new("StaticMesh").small().color(Color32::GRAY));
                                            });
                                        });

                                    ui.add_space(8.0);

                                    Frame::none()
                                        .fill(Color32::from_rgb(38, 42, 54))
                                        .rounding(Rounding::same(6.0))
                                        .inner_margin(Margin::same(8.0))
                                        .show(ui, |ui| {
                                            ui.set_width(110.0);
                                            ui.vertical_centered(|ui| {
                                                ui.label(RichText::new("⚡").font(egui::FontId::proportional(36.0)));
                                                ui.label(RichText::new("BP_Player").strong());
                                                ui.label(RichText::new("Blueprint").small().color(Color32::GRAY));
                                            });
                                        });
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

    // 2. Barra de Abas Conectadas Inferior (Unreal Engine 5.8 Bottom Docking Bar)
    egui::TopBottomPanel::bottom("oxyd_bottom_docking_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // Aba 1: Content Drawer
            let is_drawer_open = layout.active_bottom_tab == BottomTab::ContentDrawer;
            let drawer_bg = if is_drawer_open { Color32::from_rgb(45, 52, 68) } else { Color32::from_rgb(26, 28, 36) };
            let drawer_text = if is_drawer_open { Color32::WHITE } else { Color32::GRAY };

            Frame::none()
                .fill(drawer_bg)
                .rounding(Rounding::same(4.0))
                .inner_margin(Margin::symmetric(10.0, 4.0))
                .show(ui, |ui| {
                    if ui.button(RichText::new("📁 Content Drawer").color(drawer_text).strong()).clicked() {
                        layout.active_bottom_tab = if is_drawer_open { BottomTab::None } else { BottomTab::ContentDrawer };
                        layout.save();
                    }
                });

            // Aba 2: Output Log
            let is_log_open = layout.active_bottom_tab == BottomTab::OutputLog;
            let log_bg = if is_log_open { Color32::from_rgb(45, 52, 68) } else { Color32::from_rgb(26, 28, 36) };
            let log_text = if is_log_open { Color32::WHITE } else { Color32::GRAY };

            Frame::none()
                .fill(log_bg)
                .rounding(Rounding::same(4.0))
                .inner_margin(Margin::symmetric(10.0, 4.0))
                .show(ui, |ui| {
                    if ui.button(RichText::new("📋 Output Log").color(log_text).strong()).clicked() {
                        layout.active_bottom_tab = if is_log_open { BottomTab::None } else { BottomTab::OutputLog };
                        layout.save();
                    }
                });

            ui.separator();

            // Aba 3: Cmd Console Input Interativo
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
