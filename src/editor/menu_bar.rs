use egui::{RichText, Color32};
use crate::scene::{World, PrimitiveType};
use crate::editor::{I18nManager, LayoutSettings, BottomTab};
use glam::Vec3;

pub fn show_top_bars(
    ctx: &egui::Context,
    world: &mut World,
    i18n: &mut I18nManager,
    layout: &mut LayoutSettings,
    project_name: &str,
) -> Option<bool> {
    let mut switch_lang: Option<&'static str> = None;
    let mut switch_project_requested = false;

    // Estilo dos Menus estilo Unreal Engine 5.8: Flat, espaçamento homogêneo de 14px
    let mut style = (*ctx.style()).clone();
    style.visuals.button_frame = false;
    style.spacing.item_spacing.x = 14.0;
    ctx.set_style(style);

    // 1. Barra Principal Superior (Unreal Main Menu Bar: File, Edit, Window, Tools, Build, Platforms, Select, Actor, Help)
    egui::TopBottomPanel::top("oxyd_top_menu_bar").show(ctx, |ui| {
        let tr = &i18n.strings;

        ui.horizontal(|ui| {
            ui.label(RichText::new("⬢").font(egui::FontId::proportional(18.0)).color(Color32::WHITE));

            ui.menu_button(&tr.file_menu, |ui| {
                if ui.button(format!("📄 {}", tr.new_level)).clicked() {
                    *world = World::new_default_scene();
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

            ui.menu_button(&tr.edit_menu, |ui| {
                if ui.button(format!("🗑 {}", tr.delete_selected)).clicked() {
                    world.delete_selected();
                    ui.close_menu();
                }
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

            ui.menu_button("Tools", |ui| {
                ui.label("Modeling Tools");
                ui.label("Animation Editor");
                ui.label("Niagara VFX Studio");
            });

            ui.menu_button("Build", |ui| {
                ui.label("Build Lighting");
                ui.label("Build Navigation Mesh");
                ui.label("Build Geometry");
            });

            ui.menu_button("Platforms", |ui| {
                ui.label("Windows (x86_64)");
                ui.label("Linux (x86_64)");
                ui.label("Android (Vulkan)");
            });

            ui.menu_button("Select", |ui| {
                ui.label("Select All Actors");
                ui.label("Select Invert");
            });

            ui.menu_button("Actor", |ui| {
                ui.label("Group Selected");
                ui.label("Snap to Ground");
            });

            ui.menu_button(&tr.help_menu, |ui| {
                ui.label("Oxyd Engine v0.0.1 - Open Source Game Engine");
            });

            // Nome do Projeto e REGRA CRÍTICA UNTRANSLATED MENU LANGUAGE na extremidade direita
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.menu_button(RichText::new("LANGUAGE").color(Color32::from_rgb(255, 200, 50)).strong(), |ui| {
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

    // 2. Barra de Abas Superiores (Unreal Document Tabs Bar)
    egui::TopBottomPanel::top("oxyd_tabs_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("🏠").color(Color32::GRAY));
            ui.separator();

            for (idx, tab_title) in open_tabs.iter().enumerate() {
                let is_active = active_idx == idx;
                let bg_fill = if is_active { Color32::from_rgb(45, 50, 65) } else { Color32::from_rgb(28, 30, 38) };
                let text_color = if is_active { Color32::WHITE } else { Color32::GRAY };

                egui::Frame::none()
                    .fill(bg_fill)
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

    // 3. Sub-toolbar do Viewport (Play/Pause, Selection Mode Popup, Quick Add, Viewport Modes)
    egui::TopBottomPanel::top("oxyd_viewport_toolbar").show(ctx, |ui| {
        let tr = &i18n.strings;

        ui.horizontal(|ui| {
            if ui.button("💾").on_hover_text("Save Current Level").clicked() {}
            if ui.button("📂").on_hover_text("Open Content Drawer").clicked() {
                layout.active_bottom_tab = if layout.active_bottom_tab == BottomTab::ContentDrawer { BottomTab::None } else { BottomTab::ContentDrawer };
                layout.save();
            }

            ui.separator();

            ui.menu_button(RichText::new("🎯 Selection Mode ⏷").strong(), |ui| {
                ui.set_width(220.0);

                if ui.selectable_label(true, "🎯  Selection           SHIFT+1").clicked() {
                    ui.close_menu();
                }
                if ui.selectable_label(false, "🏔️  Landscape           SHIFT+2").clicked() {
                    ui.close_menu();
                }
                if ui.selectable_label(false, "🌿  Foliage             SHIFT+3").clicked() {
                    ui.close_menu();
                }
                if ui.selectable_label(false, "🖌️  Mesh Paint          SHIFT+4").clicked() {
                    ui.close_menu();
                }
                if ui.selectable_label(false, "🧊  Modeling            SHIFT+5").clicked() {
                    ui.close_menu();
                }
                if ui.selectable_label(false, "💥  Fracture            SHIFT+6").clicked() {
                    ui.close_menu();
                }
                if ui.selectable_label(false, "📦  Brush Editing       SHIFT+7").clicked() {
                    ui.close_menu();
                }
                if ui.selectable_label(false, "🏃  Animation           SHIFT+8").clicked() {
                    ui.close_menu();
                }
                if ui.selectable_label(false, "🌐  PCG                 SHIFT+9").clicked() {
                    ui.close_menu();
                }
            });

            ui.separator();

            ui.menu_button(RichText::new(format!("📦+ ⏷")).color(Color32::from_rgb(0, 180, 255)).strong(), |ui| {
                ui.heading(&tr.actors_primitives);
                if ui.button(format!("📦 {}", tr.cube_actor)).clicked() {
                    world.add_actor("New_Cube", PrimitiveType::Cube, Vec3::ZERO, [0.2, 0.6, 1.0, 1.0]);
                    ui.close_menu();
                }
                if ui.button(format!("🔮 {}", tr.sphere_actor)).clicked() {
                    world.add_actor("New_Sphere", PrimitiveType::Sphere, Vec3::ZERO, [0.9, 0.3, 0.3, 1.0]);
                    ui.close_menu();
                }
                if ui.button(format!("💡 {}", tr.point_light)).clicked() {
                    world.add_actor("Point_Light", PrimitiveType::PointLight, Vec3::new(0.0, 3.0, 0.0), [1.0, 0.9, 0.6, 1.0]);
                    ui.close_menu();
                }
            });

            ui.menu_button("🔗 ⏷", |_ui| {});
            ui.menu_button("🎬 ⏷", |_ui| {});

            ui.separator();

            if world.is_playing {
                if ui.button(RichText::new("⏹").color(Color32::from_rgb(255, 80, 80)).strong()).on_hover_text("Stop Simulation").clicked() {
                    world.is_playing = false;
                }
                if ui.button("⏸").on_hover_text("Pause").clicked() {}
            } else {
                if ui.button(RichText::new("▶").color(Color32::from_rgb(80, 220, 100)).strong()).on_hover_text("Play Simulation").clicked() {
                    world.is_playing = true;
                }
                if ui.button("🍿").on_hover_text("Simulate in Viewport").clicked() {}
            }

            ui.separator();

            ui.label("📐 10");
            ui.label("🔄 10°");
            ui.label("📏 0.25");
        });
    });

    if switch_project_requested {
        Some(true)
    } else {
        None
    }
}
