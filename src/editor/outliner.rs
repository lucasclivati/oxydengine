use egui::{RichText, Color32};
use crate::scene::World;
use crate::editor::{I18nManager, LayoutSettings};

pub fn show_outliner_panel(
    ctx: &egui::Context,
    world: &mut World,
    i18n: &I18nManager,
    layout: &mut LayoutSettings,
) {
    let tr = &i18n.strings;

    egui::SidePanel::right("oxyd_world_outliner")
        .resizable(true)
        .default_width(layout.outliner_width)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new(format!("📑 Outliner")).color(Color32::from_rgb(220, 220, 220)).strong());
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙").clicked() {}
                    if ui.button("➕").clicked() {}
                });
            });

            ui.separator();

            // Campo de busca de atores estilo Unreal Outliner
            ui.horizontal(|ui| {
                ui.label("🔍");
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::singleline(&mut world.search_filter).hint_text("Search Outliner...")
                );
            });

            ui.separator();

            // Cabeçalho da tabela do Outliner (Item Label | Type)
            ui.horizontal(|ui| {
                ui.label(RichText::new("Item Label").strong().small());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(RichText::new("Type").strong().small());
                });
            });
            ui.separator();

            // Lista de atores na cena
            let mut selected_id = world.selected_actor_id;
            let search_filter = world.search_filter.to_lowercase();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for actor in &world.actors {
                    if !search_filter.is_empty() && !actor.name.to_lowercase().contains(&search_filter) {
                        continue;
                    }

                    let is_selected = selected_id == Some(actor.id);
                    let visibility_icon = if actor.visible { "👁" } else { "🚫" };

                    ui.horizontal(|ui| {
                        if ui.small_button(visibility_icon).clicked() {
                            // Toggle visibilidade
                        }

                        let label_text = format!("📦 {}", actor.name);
                        if ui.selectable_label(is_selected, label_text).clicked() {
                            selected_id = Some(actor.id);
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let type_str = match actor.primitive {
                                crate::scene::PrimitiveType::Cube => "StaticMeshActor",
                                crate::scene::PrimitiveType::Sphere => "StaticMeshActor",
                                crate::scene::PrimitiveType::PointLight => "PointLight",
                                _ => "Actor",
                            };
                            ui.label(RichText::new(type_str).small().color(Color32::GRAY));
                        });
                    });
                }
            });

            world.selected_actor_id = selected_id;

            ui.add_space(8.0);
            if world.selected_actor_id.is_some() {
                if ui.button(format!("🗑 {}", tr.clear_selection)).clicked() {
                    world.selected_actor_id = None;
                }
            }
        });
}
