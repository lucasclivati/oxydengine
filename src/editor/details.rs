use egui::{RichText, Color32};
use crate::scene::{World, PrimitiveType};
use crate::editor::{I18nManager, LayoutSettings};

pub fn show_details_panel(
    ctx: &egui::Context,
    world: &mut World,
    i18n: &I18nManager,
    layout: &mut LayoutSettings,
) {
    let tr = &i18n.strings;

    egui::SidePanel::right("oxyd_details_panel")
        .resizable(true)
        .default_width(layout.outliner_width)
        .show(ctx, |ui| {
            ui.heading(RichText::new(format!("⚙ {}", tr.details)).color(Color32::from_rgb(220, 220, 220)).strong());
            ui.separator();

            if let Some(actor) = world.get_selected_actor_mut() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("{}:", tr.actor_name)).strong());
                        ui.text_edit_singleline(&mut actor.name);
                    });
                });

                ui.add_space(8.0);

                // Transform (Location, Rotation, Scale)
                ui.collapsing(RichText::new(format!("📐 {}", tr.transform)).strong(), |ui| {
                    egui::Grid::new("transform_grid")
                        .num_columns(4)
                        .spacing([10.0, 6.0])
                        .show(ui, |ui| {
                            // Location (Position)
                            ui.label(RichText::new(&tr.location).color(Color32::from_rgb(255, 107, 53)));
                            ui.add(egui::DragValue::new(&mut actor.transform.position.x).speed(0.1).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut actor.transform.position.y).speed(0.1).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut actor.transform.position.z).speed(0.1).prefix("Z: "));
                            ui.end_row();

                            // Rotation
                            ui.label(RichText::new(&tr.rotation).color(Color32::from_rgb(80, 220, 100)));
                            ui.add(egui::DragValue::new(&mut actor.transform.rotation.x).speed(1.0).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut actor.transform.rotation.y).speed(1.0).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut actor.transform.rotation.z).speed(1.0).prefix("Z: "));
                            ui.end_row();

                            // Scale
                            ui.label(RichText::new(&tr.scale).color(Color32::from_rgb(0, 180, 255)));
                            ui.add(egui::DragValue::new(&mut actor.transform.scale.x).speed(0.05).prefix("X: "));
                            ui.add(egui::DragValue::new(&mut actor.transform.scale.y).speed(0.05).prefix("Y: "));
                            ui.add(egui::DragValue::new(&mut actor.transform.scale.z).speed(0.05).prefix("Z: "));
                            ui.end_row();
                        });
                });

                ui.add_space(8.0);

                // Componentes Adicionais
                match actor.primitive {
                    PrimitiveType::Cube | PrimitiveType::Sphere => {
                        ui.collapsing(RichText::new(format!("🎨 {}", tr.material_shading)).strong(), |ui| {
                            ui.horizontal(|ui| {
                                ui.label(&tr.base_color);
                                ui.color_edit_button_rgba_unmultiplied(&mut actor.color);
                            });
                        });
                    }
                    PrimitiveType::PointLight => {
                        ui.collapsing(RichText::new(format!("💡 {}", tr.light_component)).strong(), |ui| {
                            ui.horizontal(|ui| {
                                ui.label(&tr.base_color);
                                ui.color_edit_button_rgba_unmultiplied(&mut actor.color);
                            });
                            ui.horizontal(|ui| {
                                ui.label(&tr.intensity);
                                ui.add(egui::Slider::new(&mut actor.intensity, 0.0..=10.0));
                            });
                        });
                    }
                    _ => {}
                }
            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(RichText::new(format!("🚫 {}", tr.no_actor_selected)).italics().color(Color32::GRAY));
                    ui.label(&tr.select_actor_hint);
                });
            }
        });
}
