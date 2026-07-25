use egui::{RichText, Color32, Vec2, Frame, Margin, Rounding};
use crate::scene::World;

pub enum MainMenuAction {
    None,
    HostSoloGame,
    JoinLobby,
    OpenSettings,
    QuitGame,
}

pub fn show_main_menu_widget(
    ctx: &egui::Context,
    world: &mut World,
) -> MainMenuAction {
    let mut action = MainMenuAction::None;

    // Overlay Centralizado do WBP_MainMenu.uasset sobre a Viewport 3D
    egui::Area::new(egui::Id::new("wbp_main_menu_overlay"))
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            Frame::none()
                .fill(Color32::from_rgba_unmultiplied(18, 20, 26, 220))
                .rounding(Rounding::same(12.0))
                .inner_margin(Margin::same(24.0))
                .show(ui, |ui| {
                    ui.set_width(320.0);
                    ui.vertical_centered(|ui| {
                        // Título do WBP_MainMenu
                        ui.heading(RichText::new("⚔️ ALCHEMY SURVIVAL").font(egui::FontId::proportional(26.0)).color(Color32::from_rgb(255, 107, 53)).strong());
                        ui.label(RichText::new("v0.0.1 - Unreal Engine Replica").color(Color32::from_rgb(160, 170, 190)));

                        ui.add_space(20.0);

                        // Botão 1: Host / Play Solo (WBP_ButtonStandard)
                        if ui.add_sized(Vec2::new(260.0, 42.0), egui::Button::new(RichText::new("▶ PLAY SOLO / HOST MATCH").font(egui::FontId::proportional(16.0)).strong())).clicked() {
                            world.is_playing = true;
                            action = MainMenuAction::HostSoloGame;
                        }

                        ui.add_space(10.0);

                        // Botão 2: Lobby (WBP_ButtonStandard)
                        if ui.add_sized(Vec2::new(260.0, 42.0), egui::Button::new(RichText::new("🌐 MULTIPLAYER LOBBY").font(egui::FontId::proportional(16.0)).strong())).clicked() {
                            action = MainMenuAction::JoinLobby;
                        }

                        ui.add_space(10.0);

                        // Botão 3: Configurações (SG_Settings)
                        if ui.add_sized(Vec2::new(260.0, 42.0), egui::Button::new(RichText::new("⚙ SETTINGS").font(egui::FontId::proportional(16.0)).strong())).clicked() {
                            action = MainMenuAction::OpenSettings;
                        }

                        ui.add_space(10.0);

                        // Botão 4: Sair
                        if ui.add_sized(Vec2::new(260.0, 42.0), egui::Button::new(RichText::new("🚪 QUIT GAME").font(egui::FontId::proportional(16.0)).strong())).clicked() {
                            action = MainMenuAction::QuitGame;
                        }
                    });
                });
        });

    action
}
