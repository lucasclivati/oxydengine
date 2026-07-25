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

    let accent = Color32::from_rgba_unmultiplied(
        layout.current_theme.accent_color[0],
        layout.current_theme.accent_color[1],
        layout.current_theme.accent_color[2],
        layout.current_theme.accent_color[3],
    );

    let highlight_text = Color32::from_rgba_unmultiplied(
        layout.current_theme.highlight_text[0],
        layout.current_theme.highlight_text[1],
        layout.current_theme.highlight_text[2],
        layout.current_theme.highlight_text[3],
    );

    let text_light = Color32::from_rgba_unmultiplied(
        layout.current_theme.text_light[0],
        layout.current_theme.text_light[1],
        layout.current_theme.text_light[2],
        layout.current_theme.text_light[3],
    );

    // 1. Content Drawer & Output Log Panel (Com resizer na borda superior e salvamento de altura)
    match layout.active_bottom_tab {
        BottomTab::ContentDrawer => {
            let drawer_res = egui::TopBottomPanel::bottom("oxyd_content_drawer_panel")
                .resizable(true)
                .height_range(140.0..=650.0)
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
                        // PAINEL DE PASTAS À ESQUERDA: MAPS EM PRIMEIRA POSIÇÃO + ALINHAMENTO À ESQUERDA + BORDA VISÍVEL 1PX (normal_text) + BUSCA DINÂMICA DO DISCO E CRIAÇÃO DE NOVAS PASTAS
                        ui.vertical(|ui| {
                            ui.set_width(180.0);
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("📦 Content Folders").strong().color(Color32::from_rgb(170, 180, 195)));
                            });
                            ui.add_space(4.0);

                            // Garantir que as pastas padrões e a estrutura no disco existam
                            let content_root = "projects/TopDownExample/Content";
                            let _ = fs::create_dir_all(content_root);

                            let mut folders = vec![
                                "Maps".to_string(),
                                "Actors".to_string(),
                                "Materials".to_string(),
                                "Meshes".to_string(),
                                "Textures".to_string(),
                                "Decals".to_string(),
                                "VFX".to_string(),
                            ];

                            for f in &folders {
                                let _ = fs::create_dir_all(format!("{}/{}", content_root, f));
                            }

                            // Descobrir dinamicamente qualquer pasta customizada criada pelo usuário no disco
                            if let Ok(entries) = fs::read_dir(content_root) {
                                for entry in entries.flatten() {
                                    if entry.path().is_dir() {
                                        let folder_name = entry.file_name().to_string_lossy().to_string();
                                        if !folders.contains(&folder_name) {
                                            folders.push(folder_name);
                                        }
                                    }
                                }
                            }

                            egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
                                for f in &folders {
                                    let is_sel = console_state.selected_folder == *f;
                                    let bg_color = if is_sel { accent } else { Color32::from_rgb(26, 30, 40) };
                                    let stroke_color = if is_sel { accent } else { text_light };
                                    let folder_text_color = if is_sel { highlight_text } else { text_light };

                                    let btn_frame = Frame::none()
                                        .fill(bg_color)
                                        .rounding(Rounding::same(4.0))
                                        .stroke(Stroke::new(1.0_f32, stroke_color))
                                        .inner_margin(Margin::symmetric(10.0, 5.0));

                                    let res = btn_frame.show(ui, |ui| {
                                        ui.set_width(155.0);
                                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                            ui.label(RichText::new(format!("📁  {}", f)).color(folder_text_color).strong());
                                        });
                                    });

                                    // REGISTRO DE CLIQUE 100% GARANTIDO ATRAVÉS DE SENSE::CLICK NO RECT DO BOTÃO
                                    let click_resp = ui.interact(res.response.rect, ui.id().with(f), egui::Sense::click());
                                    if click_resp.clicked() {
                                        console_state.selected_folder = f.clone();
                                    }
                                    ui.add_space(3.0);
                                }
                            });

                            ui.add_space(6.0);
                            ui.separator();
                            ui.add_space(4.0);

                            // BOTÃO PARA CRIAR UMA NOVA PASTA CUSTOMIZADA QUALQUER (ENGINE TOTALMENTE CUSTOMIZÁVEL)
                            ui.horizontal(|ui| {
                                if ui.button("➕ New Folder").clicked() {
                                    let new_folder_name = format!("CustomFolder_{}", folders.len() - 6);
                                    let new_path = format!("{}/{}", content_root, new_folder_name);
                                    if let Ok(_) = fs::create_dir_all(&new_path) {
                                        console_state.selected_folder = new_folder_name;
                                    }
                                }
                            });
                        });

                        ui.separator();

                        // GRADE DE ASSETS REAIS À DIREITA
                        ui.vertical(|ui| {
                            let curr_folder = console_state.selected_folder.clone();

                            ui.horizontal(|ui| {
                                if ui.button("➕ Add").clicked() {}
                                if ui.button("📥 Import").clicked() {}
                                if ui.button("💾 Save All").clicked() {}
                                ui.separator();
                                ui.label(RichText::new(format!("All > Content > TopDownExample > {}", curr_folder)).strong().color(accent));
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
                                                "obj" | "fbx" | "glb" => ("📦", "StaticMesh"),
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
                                                found_items.push(("M_GridBase.uasset".to_string(), "Material".to_string(), "🎨"));
                                                found_items.push(("M_GoldArmor.uasset".to_string(), "Material".to_string(), "🎨"));
                                            }
                                            "Meshes" => {
                                                found_items.push(("SM_Building_A.fbx".to_string(), "StaticMesh".to_string(), "🧱"));
                                                found_items.push(("SM_Chest.glb".to_string(), "StaticMesh".to_string(), "📦"));
                                            }
                                            "Textures" => {
                                                found_items.push(("T_Floor_Normal.png".to_string(), "Texture2D".to_string(), "🖼️"));
                                                found_items.push(("T_Gold_Albedo.png".to_string(), "Texture2D".to_string(), "🖼️"));
                                            }
                                            "Decals" => {
                                                found_items.push(("M_Decal_BloodSplatter.uasset".to_string(), "Decal Material".to_string(), "🎯"));
                                            }
                                            "VFX" => {
                                                found_items.push(("NS_DustParticles.uasset".to_string(), "Niagara System".to_string(), "✨"));
                                                found_items.push(("SkyAtmosphere.uasset".to_string(), "Sky Atmosphere Asset".to_string(), "🌅"));
                                            }
                                            _ => {}
                                        }
                                    }

                                    for (name, item_type, icon) in found_items {
                                        let card_bg = Color32::from_rgb(28, 32, 42);
                                        let card_res = Frame::none()
                                            .fill(card_bg)
                                            .rounding(Rounding::same(6.0))
                                            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(50, 56, 70)))
                                            .inner_margin(Margin::same(8.0))
                                            .show(ui, |ui| {
                                                ui.set_width(110.0);
                                                ui.set_height(90.0);
                                                ui.vertical_centered(|ui| {
                                                    ui.add_space(4.0);
                                                    ui.label(RichText::new(icon).size(28.0));
                                                    ui.add_space(2.0);
                                                    ui.label(RichText::new(&name).strong().size(11.0).color(Color32::from_rgb(230, 235, 245)));
                                                    ui.label(RichText::new(&item_type).size(9.0).color(Color32::GRAY));
                                                });
                                            });

                                        let click_resp = ui.interact(card_res.response.rect, ui.id().with(&name), egui::Sense::click());
                                        if click_resp.double_clicked() || click_resp.clicked() {
                                            let clean_name = name.trim_end_matches(".uasset").trim_end_matches(".oxydlevel").to_string();
                                            if !layout.open_tabs.contains(&clean_name) {
                                                layout.open_tabs.push(clean_name.clone());
                                            }
                                            if let Some(pos) = layout.open_tabs.iter().position(|t| t == &clean_name) {
                                                layout.active_tab_index = pos;
                                            }
                                            layout.save();

                                            if clean_name.starts_with("Map_") {
                                                match clean_name.as_str() {
                                                    "Map_MainMenu" => *world = World::new_main_menu_scene(),
                                                    "Map_Lobby" => *world = World::new_third_person_level(),
                                                    "Map_CityZombieSurvival" => *world = World::new_first_person_level(),
                                                    _ => *world = World::new_default_scene(),
                                                }
                                            }
                                        }
                                        ui.add_space(6.0);
                                    }
                                });
                            });
                        });
                    });
                });

            let new_h = drawer_res.response.rect.height();
            if new_h > 50.0 && (new_h - layout.content_drawer_height).abs() > 1.0 {
                layout.content_drawer_height = new_h;
                layout.save();
            }
        }
        BottomTab::OutputLog => {
            let log_res = egui::TopBottomPanel::bottom("oxyd_output_log_panel")
                .resizable(true)
                .height_range(140.0..=650.0)
                .default_height(layout.content_drawer_height)
                .frame(
                    Frame::none()
                        .fill(Color32::from_rgb(14, 16, 22))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(45, 50, 62)))
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

                    egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                        for (log_entry, color) in &console_state.logs {
                            ui.label(RichText::new(log_entry).color(*color).monospace());
                        }
                    });
                });

            let new_h = log_res.response.rect.height();
            if new_h > 50.0 && (new_h - layout.content_drawer_height).abs() > 1.0 {
                layout.content_drawer_height = new_h;
                layout.save();
            }
        }
        BottomTab::None => {}
    }

    // 2. Barra Status Infeiror Fixa com os Botões de Abrir Content Drawer e Output Log
    egui::TopBottomPanel::bottom("oxyd_dock_toggle_bar")
        .frame(
            Frame::none()
                .fill(Color32::from_rgb(16, 18, 24))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(40, 44, 56)))
                .inner_margin(Margin::symmetric(10.0, 4.0))
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let is_drawer_open = layout.active_bottom_tab == BottomTab::ContentDrawer;
                let drawer_bg = if is_drawer_open { accent } else { Color32::from_rgb(26, 28, 36) };
                let drawer_stroke = if is_drawer_open { Stroke::new(1.0_f32, accent) } else { Stroke::new(1.0_f32, text_light) };
                let drawer_text = if is_drawer_open { highlight_text } else { text_light };

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
                let log_bg = if is_log_open { accent } else { Color32::from_rgb(26, 28, 36) };
                let log_stroke = if is_log_open { Stroke::new(1.0_f32, accent) } else { Stroke::new(1.0_f32, text_light) };
                let log_text = if is_log_open { highlight_text } else { text_light };

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
