use egui::{RichText, Color32, Frame, Margin, Stroke};
use crate::scene::{World, PrimitiveType};
use crate::editor::{I18nManager, LayoutSettings, BottomTab};
use glam::Vec3;
use std::fs;
use std::path::Path;

pub fn show_top_bars(
    ctx: &egui::Context,
    world: &mut World,
    i18n: &mut I18nManager,
    layout: &mut LayoutSettings,
    project_name: &str,
) -> Option<bool> {
    let mut switch_lang: Option<&'static str> = None;
    let mut switch_project_requested = false;

    let mut style = (*ctx.style()).clone();
    style.visuals.button_frame = false;
    style.spacing.item_spacing.x = 14.0;
    ctx.set_style(style);

    // 1. Barra Principal Superior (Mais escura #0E1015)
    egui::TopBottomPanel::top("oxyd_top_menu_bar")
        .frame(
            Frame::none()
                .fill(Color32::from_rgb(14, 16, 21))
                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 44, 56)))
                .inner_margin(Margin::symmetric(10.0, 6.0))
        )
        .show(ctx, |ui| {
            let tr = &i18n.strings;

            ui.horizontal(|ui| {
                ui.menu_button(&tr.file_menu, |ui| {
                    if ui.button(format!("📄 {}", tr.new_level)).clicked() {
                        world.push_undo_state();
                        *world = World::new_default_scene();
                        ui.close_menu();
                    }
                    if ui.button("📂 Open Level...").clicked() {
                        log::info!("Opening level dialog...");
                        ui.close_menu();
                    }
                    if ui.button(format!("💾 {}", tr.save_level)).clicked() {
                        log::info!("{}", tr.save_level);
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button(format!("🔄 {}", tr.switch_project)).clicked() {
                        switch_project_requested = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button(format!("🚪 {}", tr.exit)).clicked() {
                        std::process::exit(0);
                    }
                });

                // Menu Edit
                ui.menu_button(&tr.edit_menu, |ui| {
                    if ui.button("↩ Undo                  Ctrl+Z").clicked() {
                        world.undo();
                        ui.close_menu();
                    }
                    if ui.button("↪ Redo          Ctrl+Shift+Z").clicked() {
                        world.redo();
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.label("✂ Cut                     Ctrl+X");
                    ui.label("📋 Copy                  Ctrl+C");
                    ui.label("📄 Paste                  Ctrl+V");
                });

                ui.menu_button(&tr.window_menu, |ui| {
                    let mut drawer_active = layout.active_bottom_tab == BottomTab::ContentDrawer;
                    if ui.checkbox(&mut drawer_active, format!("📁 {}", tr.content_drawer)).changed() {
                        layout.active_bottom_tab = if drawer_active { BottomTab::ContentDrawer } else { BottomTab::None };
                        layout.save();
                    }

                    let mut log_active = layout.active_bottom_tab == BottomTab::OutputLog;
                    if ui.checkbox(&mut log_active, format!("📋 {}", tr.output_log)).changed() {
                        layout.active_bottom_tab = if log_active { BottomTab::OutputLog } else { BottomTab::None };
                        layout.save();
                    }
                });

                // Menu Build em Amarelo Ouro (#F59E0B)
                ui.menu_button("Build", |ui| {
                    if ui.button(RichText::new("🔨 Build Game Executable (Package Game)...").color(Color32::from_rgb(245, 158, 11)).strong()).clicked() {
                        let build_dir = format!("builds/{}", project_name);
                        let _ = fs::create_dir_all(&build_dir);
                        let target_exe = format!("{}/{}.exe", build_dir, project_name);

                        if Path::new("OxydEngine.exe").exists() {
                            let _ = fs::copy("OxydEngine.exe", &target_exe);
                        }

                        log::info!("Jogo empacotado com sucesso em: {}", target_exe);
                        let _ = std::process::Command::new("explorer").arg(&build_dir).spawn();
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.label("Build Lighting");
                    ui.label("Build Navigation Mesh");
                    ui.label("Build Geometry");
                });

                // Menu Help
                ui.menu_button(&tr.help_menu, |ui| {
                    ui.label("Oxyd Engine v0.0.1 - Open Source Game Engine");
                    ui.separator();
                    if ui.button("🌐 GitHub Repository (Open Source)").clicked() {
                        let _ = std::process::Command::new("cmd")
                            .args(["/c", "start", "https://github.com/lucasclivati/oxydengine"])
                            .spawn();
                        ui.close_menu();
                    }
                });

                // Nome do Projeto e LANGUAGE em Amarelo Ouro (#F59E0B)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.menu_button(RichText::new("LANGUAGE").color(Color32::from_rgb(245, 158, 11)).strong(), |ui| {
                        egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
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

                    ui.label(RichText::new(project_name).color(Color32::from_rgb(180, 190, 210)).strong());
                });
            });
        });

    if let Some(lang) = switch_lang {
        i18n.load_language(lang);
    }

    let open_tabs: Vec<String> = layout.open_tabs.clone();
    let active_idx = layout.active_tab_index;
    let mut new_active_idx: Option<usize> = None;
    let mut close_tab_index: Option<usize> = None;

    // 2. Barra de Abas Superiores
    egui::TopBottomPanel::top("oxyd_tabs_bar")
        .frame(
            Frame::none()
                .fill(Color32::from_rgb(18, 20, 26))
                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 44, 56)))
                .inner_margin(Margin::symmetric(8.0, 4.0))
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("🏠").color(Color32::GRAY));
                ui.separator();

                for (idx, tab_title) in open_tabs.iter().enumerate() {
                    let is_active = active_idx == idx;
                    let bg_fill = if is_active { Color32::from_rgb(40, 45, 58) } else { Color32::from_rgb(22, 24, 32) };
                    let stroke = if is_active { Stroke::new(1.0, Color32::from_rgb(245, 158, 11)) } else { Stroke::NONE };
                    let text_color = if is_active { Color32::WHITE } else { Color32::GRAY };

                    egui::Frame::none()
                        .fill(bg_fill)
                        .stroke(stroke)
                        .rounding(egui::Rounding::same(4.0))
                        .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let btn_res = ui.button(RichText::new(format!("⛰ {}", tab_title)).color(text_color).strong());
                                if btn_res.clicked() {
                                    new_active_idx = Some(idx);
                                }

                                if open_tabs.len() > 1 {
                                    if ui.small_button("×").clicked() {
                                        close_tab_index = Some(idx);
                                    }
                                }
                            });
                        });

                    ui.add_space(2.0);
                }
            });
        });

    if let Some(n_idx) = new_active_idx {
        layout.active_tab_index = n_idx;
        layout.save();
    }

    if let Some(c_idx) = close_tab_index {
        if layout.open_tabs.len() > 1 {
            layout.open_tabs.remove(c_idx);
            if layout.active_tab_index >= layout.open_tabs.len() {
                layout.active_tab_index = layout.open_tabs.len().saturating_sub(1);
            }
            layout.save();
        }
    }

    let mut style_toolbar = (*ctx.style()).clone();
    style_toolbar.visuals.button_frame = true;
    style_toolbar.spacing.item_spacing.x = 8.0;
    ctx.set_style(style_toolbar);

    // 3. Sub-toolbar do Viewport (Ícones de Selection Mode limpos sem quadrados)
    egui::TopBottomPanel::top("oxyd_viewport_toolbar")
        .frame(
            Frame::none()
                .fill(Color32::from_rgb(22, 25, 32))
                .stroke(Stroke::new(1.0, Color32::from_rgb(40, 44, 56)))
                .inner_margin(Margin::symmetric(8.0, 4.0))
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("💾").on_hover_text("Save Current Level").clicked() {}
                if ui.button("📂").on_hover_text("Open Content Drawer").clicked() {
                    layout.active_bottom_tab = if layout.active_bottom_tab == BottomTab::ContentDrawer { BottomTab::None } else { BottomTab::ContentDrawer };
                    layout.save();
                }

                ui.separator();

                // Selection Mode Limpo sem quadrados de unicode bugados
                ui.menu_button(RichText::new("🎯 Selection Mode ⏷").strong(), |ui| {
                    ui.set_width(220.0);

                    if ui.selectable_label(true, "🎯  Selection           SHIFT+1").clicked() { ui.close_menu(); }
                    if ui.selectable_label(false, "🏔️  Landscape           SHIFT+2").clicked() { ui.close_menu(); }
                    if ui.selectable_label(false, "🌿  Foliage             SHIFT+3").clicked() { ui.close_menu(); }
                    if ui.selectable_label(false, "🎨  Mesh Paint          SHIFT+4").clicked() { ui.close_menu(); }
                    if ui.selectable_label(false, "🧊  Modeling            SHIFT+5").clicked() { ui.close_menu(); }
                    if ui.selectable_label(false, "💥  Fracture            SHIFT+6").clicked() { ui.close_menu(); }
                    if ui.selectable_label(false, "📦  Brush Editing       SHIFT+7").clicked() { ui.close_menu(); }
                    if ui.selectable_label(false, "🏃  Animation           SHIFT+8").clicked() { ui.close_menu(); }
                    if ui.selectable_label(false, "🌐  PCG                 SHIFT+9").clicked() { ui.close_menu(); }
                });

                ui.separator();

                // Quick Add em Amarelo Ouro (#F59E0B)
                ui.menu_button(RichText::new("📦+ ⏷").color(Color32::from_rgb(245, 158, 11)).strong(), |ui| {
                    ui.heading("Primitives");
                    if ui.button(format!("📦 Cube")).clicked() {
                        world.add_actor("New_Cube", PrimitiveType::Cube, Vec3::ZERO, [0.8, 0.4, 0.2, 1.0]);
                        ui.close_menu();
                    }
                    if ui.button(format!("🔮 Sphere")).clicked() {
                        world.add_actor("New_Sphere", PrimitiveType::Sphere, Vec3::ZERO, [0.9, 0.3, 0.3, 1.0]);
                        ui.close_menu();
                    }
                    if ui.button(format!("💡 Point Light")).clicked() {
                        world.add_actor("Point_Light", PrimitiveType::PointLight, Vec3::new(0.0, 3.0, 0.0), [1.0, 0.9, 0.6, 1.0]);
                        ui.close_menu();
                    }
                    if ui.button("🎥 Camera Actor").clicked() {
                        world.add_actor("Camera_Actor", PrimitiveType::CameraActor, Vec3::new(0.0, 2.0, 5.0), [0.4, 0.6, 1.0, 1.0]);
                        ui.close_menu();
                    }
                    ui.separator();
                    ui.heading("Atmosphere & Lights");
                    if ui.button("☀️ Directional Light").clicked() {
                        let id = world.next_actor_id;
                        world.next_actor_id += 1;
                        world.actors.push(crate::scene::Actor::new_directional_light(id, "DirectionalLight", Vec3::new(0.0, 10.0, 0.0)));
                        ui.close_menu();
                    }
                    if ui.button("🌫️ Exponential Height Fog").clicked() {
                        let id = world.next_actor_id;
                        world.next_actor_id += 1;
                        world.actors.push(crate::scene::Actor::new_fog(id, "ExponentialHeightFog", Vec3::ZERO));
                        ui.close_menu();
                    }
                    if ui.button("🌅 Sky Atmosphere").clicked() {
                        let id = world.next_actor_id;
                        world.next_actor_id += 1;
                        world.actors.push(crate::scene::Actor::new_sky_atmosphere(id, "SkyAtmosphere", Vec3::ZERO));
                        ui.close_menu();
                    }
                    if ui.button("☁️ Volumetric Cloud").clicked() {
                        let id = world.next_actor_id;
                        world.next_actor_id += 1;
                        world.actors.push(crate::scene::Actor::new_volumetric_cloud(id, "VolumetricCloud", Vec3::ZERO));
                        ui.close_menu();
                    }
                });

                ui.separator();

                // Botão "Test Map" e "Stop Map" Claros
                if world.is_playing {
                    if ui.button(RichText::new("⏹ Stop Map").color(Color32::from_rgb(255, 80, 80)).strong()).on_hover_text("Stop Game Test Mode").clicked() {
                        world.is_playing = false;
                    }
                } else {
                    if ui.button(RichText::new("▶ Test Map").color(Color32::from_rgb(80, 220, 100)).strong()).on_hover_text("Run & Test Game Level").clicked() {
                        world.is_playing = true;
                    }
                }

                ui.separator();

                // Grid Snapping Editável (📐 Location Snap | 🔄 Rotation Snap | 📏 Scale Snap)
                ui.horizontal(|ui| {
                    ui.label("📐");
                    ui.add(egui::DragValue::new(&mut layout.location_snap).speed(1.0).range(1.0..=500.0));
                    
                    ui.label("🔄");
                    ui.add(egui::DragValue::new(&mut layout.rotation_snap).speed(1.0).range(1.0..=180.0).suffix("°"));
                    
                    ui.label("📏");
                    ui.add(egui::DragValue::new(&mut layout.scale_snap).speed(0.05).range(0.01..=10.0));
                });
            });
        });

    if switch_project_requested {
        Some(true)
    } else {
        None
    }
}
