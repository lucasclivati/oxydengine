use egui::{RichText, Color32, Frame, Margin, Rounding, Stroke, Vec2};
use crate::scene::{World, PrimitiveType};
use crate::editor::{I18nManager, LayoutSettings};

pub fn show_outliner_panel(
    ctx: &egui::Context,
    world: &mut World,
    _i18n: &I18nManager,
    layout: &mut LayoutSettings,
) -> Option<glam::Vec3> {
    let mut camera_focus_target: Option<glam::Vec3> = None;

    egui::SidePanel::right("oxyd_outliner_panel")
        .resizable(true)
        .default_width(layout.outliner_width)
        .frame(
            Frame::none()
                .fill(Color32::from_rgb(18, 20, 26))
                .stroke(Stroke::new(1.0, Color32::from_rgb(45, 50, 62)))
                .inner_margin(Margin::same(8.0))
        )
        .show(ctx, |ui| {
            // Cabeçalho de Aba (📑 Map Assets [🗑] [➕])
            Frame::none()
                .fill(Color32::from_rgb(26, 29, 38))
                .rounding(Rounding::same(4.0))
                .stroke(Stroke::new(1.0, Color32::from_rgb(45, 50, 62)))
                .inner_margin(Margin::symmetric(10.0, 6.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(RichText::new("📑 Map Assets").color(Color32::WHITE).strong());
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("➕").on_hover_text("Add Actor").clicked() {
                                world.add_actor("New_Cube", PrimitiveType::Cube, glam::Vec3::ZERO, [0.8, 0.4, 0.2, 1.0]);
                            }

                            if world.selected_actor_id.is_some() {
                                ui.add_space(4.0);
                                if ui.small_button(RichText::new("🗑").color(Color32::LIGHT_RED)).on_hover_text("Delete Selected Actor").clicked() {
                                    world.delete_selected();
                                }
                            }
                        });
                    });
                });

            ui.add_space(6.0);

            // Barra de Busca
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.add_sized(
                    Vec2::new(ui.available_width() - 10.0, 20.0),
                    egui::TextEdit::singleline(&mut world.search_filter).hint_text("Search Map Assets...")
                );
            });

            ui.add_space(6.0);

            // Cabeçalho da Tabela (Item Label | Type)
            ui.horizontal(|ui| {
                ui.add_space(24.0);
                ui.label(RichText::new("Item Label").small().strong().color(Color32::from_rgb(170, 180, 195)));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    ui.label(RichText::new("Type").small().strong().color(Color32::from_rgb(170, 180, 195)));
                });
            });
            ui.separator();

            let filter = world.search_filter.to_lowercase();
            let selected_id = world.selected_actor_id;
            let mut actor_to_select: Option<Option<u64>> = None;
            let mut actor_to_toggle_vis: Option<u64> = None;
            let mut actor_to_duplicate: Option<u64> = None;
            let mut actor_to_delete: Option<u64> = None;

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, actor) in world.actors.iter().enumerate() {
                    if !filter.is_empty() && !actor.name.to_lowercase().contains(&filter) {
                        continue;
                    }

                    let is_selected = selected_id == Some(actor.id);

                    let bg_color = if is_selected {
                        Color32::from_rgb(45, 52, 68)
                    } else if idx % 2 == 0 {
                        Color32::from_rgb(20, 22, 28)
                    } else {
                        Color32::from_rgb(24, 27, 35)
                    };

                    let icon = match actor.primitive {
                        PrimitiveType::Cube => "📦",
                        PrimitiveType::Sphere => "🔮",
                        PrimitiveType::PointLight => "💡",
                        PrimitiveType::DirectionalLight => "☀️",
                        PrimitiveType::SkyLight => "🌤️",
                        PrimitiveType::ExponentialHeightFog => "🌫️",
                        PrimitiveType::SkyAtmosphere => "🌅",
                        PrimitiveType::VolumetricCloud => "☁️",
                        PrimitiveType::CameraActor => "🎥",
                        _ => "📄",
                    };

                    let type_label = match actor.primitive {
                        PrimitiveType::Cube | PrimitiveType::Sphere => "StaticMeshActor",
                        PrimitiveType::PointLight => "PointLight",
                        PrimitiveType::DirectionalLight => "DirectionalLight",
                        PrimitiveType::SkyLight => "SkyLight",
                        PrimitiveType::ExponentialHeightFog => "ExponentialHeightFog",
                        PrimitiveType::SkyAtmosphere => "SkyAtmosphere",
                        PrimitiveType::VolumetricCloud => "VolumetricCloud",
                        PrimitiveType::CameraActor => "CameraActor",
                        _ => "Actor",
                    };

                    let vis_icon = if actor.visible { "👁" } else { "🚫" };

                    let frame_res = Frame::none()
                        .fill(bg_color)
                        .rounding(Rounding::same(3.0))
                        .stroke(if is_selected { Stroke::new(1.0, Color32::from_rgb(245, 158, 11)) } else { Stroke::NONE })
                        .inner_margin(Margin::symmetric(6.0, 4.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                if ui.small_button(vis_icon).clicked() {
                                    actor_to_toggle_vis = Some(actor.id);
                                }

                                ui.label(icon);

                                // NOME DO ATOR COM ENCOLHIMENTO/TRUNCAÇÃO PARA NUNCA SOBREPOR O TYPE
                                let text_color = if is_selected { Color32::WHITE } else { Color32::from_rgb(220, 225, 235) };
                                let name_lbl = ui.add(
                                    egui::Label::new(RichText::new(&actor.name).color(text_color).strong())
                                        .truncate()
                                );

                                if name_lbl.clicked() {
                                    actor_to_select = Some(Some(actor.id));
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(RichText::new(type_label).small().color(Color32::from_rgb(245, 158, 11)));
                                });
                            });
                        }).response;

                    let row_sense = ui.interact(frame_res.rect, ui.make_persistent_id(actor.id), egui::Sense::click());

                    if row_sense.clicked() {
                        actor_to_select = Some(Some(actor.id));
                    }

                    if row_sense.double_clicked() {
                        actor_to_select = Some(Some(actor.id));
                        camera_focus_target = Some(actor.transform.position);
                    }

                    row_sense.context_menu(|ui| {
                        if ui.button("🎯 Focus Camera (F)").clicked() {
                            camera_focus_target = Some(actor.transform.position);
                            ui.close_menu();
                        }
                        if ui.button("📋 Duplicate (Ctrl+D)").clicked() {
                            actor_to_duplicate = Some(actor.id);
                            ui.close_menu();
                        }
                        if ui.button("👁 Toggle Visibility").clicked() {
                            actor_to_toggle_vis = Some(actor.id);
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button(RichText::new("🗑 Delete").color(Color32::LIGHT_RED)).clicked() {
                            actor_to_delete = Some(actor.id);
                            ui.close_menu();
                        }
                    });

                    ui.add_space(2.0);
                }
            });

            if let Some(selection) = actor_to_select {
                world.selected_actor_id = selection;
            }

            if let Some(id) = actor_to_toggle_vis {
                if let Some(actor) = world.actors.iter_mut().find(|a| a.id == id) {
                    actor.visible = !actor.visible;
                    actor.is_visible = actor.visible;
                }
            }

            if let Some(id) = actor_to_duplicate {
                if let Some(actor) = world.actors.iter().find(|a| a.id == id).cloned() {
                    let new_id = world.next_actor_id;
                    world.next_actor_id += 1;
                    let mut new_actor = actor;
                    new_actor.id = new_id;
                    new_actor.name = format!("{}_Copy", new_actor.name);
                    new_actor.transform.position += glam::Vec3::new(1.0, 0.0, 1.0);
                    world.actors.push(new_actor);
                    world.selected_actor_id = Some(new_id);
                }
            }

            if let Some(id) = actor_to_delete {
                world.push_undo_state();
                world.actors.retain(|a| a.id != id);
                if world.selected_actor_id == Some(id) {
                    world.selected_actor_id = None;
                }
            }

            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("{} actors", world.actors.len())).small().color(Color32::GRAY));
                if world.selected_actor_id.is_some() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("❌ Clear Selection").clicked() {
                            world.selected_actor_id = None;
                        }
                    });
                }
            });
        });

    camera_focus_target
}
