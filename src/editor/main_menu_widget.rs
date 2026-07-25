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
    ui: &mut egui::Ui,
    world: &mut World,
) -> MainMenuAction {
    let mut action = MainMenuAction::None;

    // Renderizado estritamente dentro da área central do Viewport 3D (Sem sobrepor outliner ou gavetas)
    ui.centered_and_justified(|ui| {
        Frame::none()
            .fill(Color32::from_rgba_unmultiplied(20, 22, 28, 225))
            .rounding(Rounding::same(10.0))
            .inner_margin(Margin::same(24.0))
            .show(ui, |ui| {
                ui.set_width(320.0);
                ui.vertical_centered(|ui| {
                    // Título limpo sem caracteres ou quadrados bugados
                    let accent = ui.visuals().selection.bg_fill;
                    ui.heading(RichText::new("TOP DOWN EXAMPLE").font(egui::FontId::proportional(26.0)).color(accent).strong());
                    ui.label(RichText::new("v0.0.1").color(Color32::from_rgb(160, 170, 190)));

                    ui.add_space(20.0);

                    // Botão 1: Host / Play Solo
                    if ui.add_sized(Vec2::new(260.0, 42.0), egui::Button::new(RichText::new("▶ PLAY SOLO / HOST MATCH").font(egui::FontId::proportional(15.0)).strong())).clicked() {
                        world.is_playing = true;
                        action = MainMenuAction::HostSoloGame;
                    }

                    ui.add_space(10.0);

                    // Botão 2: Multiplayer Lobby
                    if ui.add_sized(Vec2::new(260.0, 42.0), egui::Button::new(RichText::new("🌐 MULTIPLAYER LOBBY").font(egui::FontId::proportional(15.0)).strong())).clicked() {
                        action = MainMenuAction::JoinLobby;
                    }

                    ui.add_space(10.0);

                    // Botão 3: Settings
                    if ui.add_sized(Vec2::new(260.0, 42.0), egui::Button::new(RichText::new("⚙ SETTINGS").font(egui::FontId::proportional(15.0)).strong())).clicked() {
                        action = MainMenuAction::OpenSettings;
                    }

                    ui.add_space(10.0);

                    // Botão 4: Quit
                    if ui.add_sized(Vec2::new(260.0, 42.0), egui::Button::new(RichText::new("🚪 QUIT GAME").font(egui::FontId::proportional(15.0)).strong())).clicked() {
                        action = MainMenuAction::QuitGame;
                    }
                });
            });
    });

    action
}
