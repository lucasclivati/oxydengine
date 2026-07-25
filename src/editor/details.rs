use egui::{RichText, Color32, Grid, Slider, Frame, Margin, Rounding, Stroke};
use crate::scene::{World, PrimitiveType};
use crate::editor::{I18nManager, LayoutSettings};

pub fn show_details_panel(
    ctx: &egui::Context,
    world: &mut World,
    i18n: &I18nManager,
    layout: &mut LayoutSettings,
) {
    let tr = &i18n.strings;

    // SidePanel com Borda 1px Nítida (#374151)
    egui::SidePanel::right("oxyd_details_panel")
        .resizable(true)
        .default_width(layout.outliner_width)
        .frame(
            Frame::none()
                .fill(Color32::from_rgb(26, 28, 34))
                .stroke(Stroke::new(1.0, Color32::from_rgb(55, 65, 81)))
                .inner_margin(Margin::same(8.0))
        )
        .show(ctx, |ui| {
            // Cabeçalho de Aba Estilo UE5 (⚙ Details)
            Frame::none()
                .fill(Color32::from_rgb(34, 37, 46))
                .rounding(Rounding::same(4.0))
                .inner_margin(Margin::symmetric(10.0, 6.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(RichText::new(format!("⚙ {}", tr.details)).color(Color32::WHITE).strong());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new("Details").small().color(Color32::GRAY));
                        });
                    });
                });

            ui.add_space(6.0);

            if let Some(actor) = world.get_selected_actor_mut() {
                // Seção 1: Nome e Tipo do Ator
                Grid::new("actor_name_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                    ui.label(RichText::new(format!("{}:", tr.actor_name)).strong());
                    ui.text_edit_singleline(&mut actor.name);
                    ui.end_row();

                    ui.label(RichText::new("Type:").strong());
                    ui.label(RichText::new(format!("{:?}", actor.primitive)).color(Color32::from_rgb(245, 158, 11)));
                    ui.end_row();
                });

                ui.separator();

                // Seção 2: Transformação (Location, Rotation, Scale com Regra de Cadeado UE5)
                ui.collapsing(RichText::new(format!("📐 {}", tr.transform)).strong(), |ui| {
                    Grid::new("transform_grid").num_columns(5).spacing([6.0, 6.0]).show(ui, |ui| {
                        // Location
                        ui.label(RichText::new("Location").color(Color32::from_rgb(255, 107, 53)).strong());
                        ui.add(egui::DragValue::new(&mut actor.transform.position.x).speed(0.1).prefix("X: "));
                        ui.add(egui::DragValue::new(&mut actor.transform.position.y).speed(0.1).prefix("Y: "));
                        ui.add(egui::DragValue::new(&mut actor.transform.position.z).speed(0.1).prefix("Z: "));
                        ui.label("");
                        ui.end_row();

                        // Rotation
                        ui.label(RichText::new("Rotation").color(Color32::from_rgb(80, 220, 100)).strong());
                        ui.add(egui::DragValue::new(&mut actor.transform.rotation.x).speed(0.5).prefix("X: ").suffix("°"));
                        ui.add(egui::DragValue::new(&mut actor.transform.rotation.y).speed(0.5).prefix("Y: ").suffix("°"));
                        ui.add(egui::DragValue::new(&mut actor.transform.rotation.z).speed(0.5).prefix("Z: ").suffix("°"));
                        ui.label("");
                        ui.end_row();

                        // Scale com Regra de Cadeado (Lock Ratio) da Unreal Engine 5
                        ui.label(RichText::new("Scale").color(Color32::from_rgb(100, 180, 255)).strong());

                        let old_x = actor.transform.scale.x;
                        let old_y = actor.transform.scale.y;
                        let old_z = actor.transform.scale.z;

                        let drag_x = ui.add(egui::DragValue::new(&mut actor.transform.scale.x).speed(0.05).prefix("X: "));
                        let drag_y = ui.add(egui::DragValue::new(&mut actor.transform.scale.y).speed(0.05).prefix("Y: "));
                        let drag_z = ui.add(egui::DragValue::new(&mut actor.transform.scale.z).speed(0.05).prefix("Z: "));

                        let lock_icon = if actor.transform.lock_scale_aspect { "🔒" } else { "🔓" };
                        if ui.button(lock_icon).on_hover_text("Lock Aspect Ratio Scale (Proportional Scale)").clicked() {
                            actor.transform.lock_scale_aspect = !actor.transform.lock_scale_aspect;
                        }

                        // Se o cadeado estiver ativado, propaga proporcionalmente a escala
                        if actor.transform.lock_scale_aspect {
                            if drag_x.changed() && old_x > 0.001 {
                                let ratio = actor.transform.scale.x / old_x;
                                actor.transform.scale.y = old_y * ratio;
                                actor.transform.scale.z = old_z * ratio;
                            } else if drag_y.changed() && old_y > 0.001 {
                                let ratio = actor.transform.scale.y / old_y;
                                actor.transform.scale.x = old_x * ratio;
                                actor.transform.scale.z = old_z * ratio;
                            } else if drag_z.changed() && old_z > 0.001 {
                                let ratio = actor.transform.scale.z / old_z;
                                actor.transform.scale.x = old_x * ratio;
                                actor.transform.scale.y = old_y * ratio;
                            }
                        }

                        ui.end_row();
                    });
                });

                ui.separator();

                // Seção 3: Light Components
                if actor.primitive == PrimitiveType::DirectionalLight || actor.primitive == PrimitiveType::PointLight || actor.primitive == PrimitiveType::SkyLight {
                    ui.collapsing(RichText::new(format!("💡 Light Component ({:?})", actor.primitive)).strong().color(Color32::from_rgb(245, 158, 11)), |ui| {
                        Grid::new("light_component_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                            ui.label("Intensity (Lux):");
                            ui.add(Slider::new(&mut actor.intensity, 0.0..=20.0));
                            ui.end_row();

                            ui.label("Light Color:");
                            ui.color_edit_button_rgba_unmultiplied(&mut actor.color);
                            ui.end_row();
                        });
                    });
                    ui.separator();
                }

                // Seção 4: Atmosphere Components
                if let Some(atm) = &mut actor.atmosphere_component {
                    ui.collapsing(RichText::new(format!("☁️ Atmosphere Component ({:?})", actor.primitive)).strong().color(Color32::from_rgb(100, 180, 255)), |ui| {
                        Grid::new("atm_component_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                            match actor.primitive {
                                PrimitiveType::ExponentialHeightFog => {
                                    ui.label("Fog Density:");
                                    ui.add(Slider::new(&mut atm.fog_density, 0.0..=0.5));
                                    ui.end_row();

                                    ui.label("Fog Height Falloff:");
                                    ui.add(Slider::new(&mut atm.fog_height_falloff, 0.0..=1.0));
                                    ui.end_row();
                                }
                                PrimitiveType::SkyAtmosphere => {
                                    ui.label("Rayleigh Scattering:");
                                    ui.add(Slider::new(&mut atm.rayleigh_scattering, 0.0..=0.1));
                                    ui.end_row();

                                    ui.label("Mie Scattering:");
                                    ui.add(Slider::new(&mut atm.mie_scattering, 0.0..=0.05));
                                    ui.end_row();
                                }
                                PrimitiveType::VolumetricCloud => {
                                    ui.label("Cloud Coverage:");
                                    ui.add(Slider::new(&mut atm.cloud_coverage, 0.0..=1.0));
                                    ui.end_row();

                                    ui.label("Cloud Altitude (m):");
                                    ui.add(Slider::new(&mut atm.cloud_altitude, 1000.0..=12000.0));
                                    ui.end_row();
                                }
                                _ => {}
                            }
                        });
                    });
                    ui.separator();
                }

                // Seção 5: Propriedades de Câmeras
                if let Some(cam) = &mut actor.camera_component {
                    ui.collapsing(RichText::new("🎥 Camera Settings").strong().color(Color32::from_rgb(100, 180, 255)), |ui| {
                        Grid::new("camera_settings_grid").num_columns(2).spacing([12.0, 8.0]).show(ui, |ui| {
                            ui.label("Field of View (FOV):");
                            ui.add(Slider::new(&mut cam.field_of_view, 30.0..=130.0).suffix("°"));
                            ui.end_row();

                            ui.label("Near Clip Plane:");
                            ui.add(egui::DragValue::new(&mut cam.near_clip_plane).speed(0.05));
                            ui.end_row();

                            ui.label("Far Clip Plane:");
                            ui.add(egui::DragValue::new(&mut cam.far_clip_plane).speed(10.0));
                            ui.end_row();

                            ui.label("Active Camera:");
                            ui.checkbox(&mut cam.is_active_camera, "Set Active Viewport");
                            ui.end_row();
                        });
                    });
                    ui.separator();
                }

                // Seção 6: Física 3D & Gravidade
                ui.collapsing(RichText::new("⚡ Physics & Gravity").strong(), |ui| {
                    ui.checkbox(&mut actor.physics.use_gravity, "Simulate Gravity (-9.8 m/s²)");
                    ui.horizontal(|ui| {
                        ui.label("Mass:");
                        ui.add(egui::DragValue::new(&mut actor.physics.mass).speed(0.1).suffix(" kg"));
                    });
                });

                ui.separator();

                // Seção 7: Material & Sombreamento
                let is_env_light = matches!(
                    actor.primitive,
                    PrimitiveType::PointLight | PrimitiveType::DirectionalLight | PrimitiveType::SkyLight |
                    PrimitiveType::ExponentialHeightFog | PrimitiveType::SkyAtmosphere | PrimitiveType::VolumetricCloud |
                    PrimitiveType::CameraActor
                );

                if !is_env_light {
                    ui.collapsing(RichText::new(format!("🎨 {}", tr.material_shading)).strong(), |ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", tr.base_color));
                            ui.color_edit_button_rgba_unmultiplied(&mut actor.color);
                        });
                    });
                }

            } else {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(RichText::new(format!("🚫 {}", tr.no_actor_selected)).color(Color32::GRAY).strong());
                    ui.add_space(6.0);
                    ui.label(RichText::new(&tr.select_actor_hint).small().color(Color32::GRAY));
                });
            }
        });
}
