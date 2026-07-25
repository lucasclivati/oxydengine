use egui::{Color32, Stroke, Style, Visuals, Rounding, Vec2};
use std::fs;

/// Paleta "Ferrugem Industrial" (Rust & Steel)
/// Fundo: Cinza ardósia / azulado frio (#181A20)
/// Destaques / Botões: Laranja Queimado / Terracota (#FF6B35 / #E05A47)
/// Fonte: Google Inter (Open Source SIL Open Font License)
pub fn apply_oxyd_theme(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    if let Ok(font_bytes) = fs::read("assets/fonts/Inter-Regular.ttf") {
        fonts.font_data.insert(
            "google_inter".to_string(),
            egui::FontData::from_owned(font_bytes),
        );
        if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            family.insert(0, "google_inter".to_string());
        }
    }

    ctx.set_fonts(fonts);

    let mut style = Style::default();

    let mut visuals = Visuals::dark();
    // Fundo da Interface: Cinza ardósia / azulado bem frio (#181A20)
    visuals.window_fill = Color32::from_rgb(24, 26, 32);
    visuals.panel_fill = Color32::from_rgb(30, 32, 40);
    visuals.faint_bg_color = Color32::from_rgb(38, 41, 51);
    visuals.extreme_bg_color = Color32::from_rgb(18, 20, 24);

    // Borda e Cantos estilo Ferrugem Industrial
    visuals.window_rounding = Rounding::same(5.0);
    visuals.widgets.noninteractive.rounding = Rounding::same(4.0);
    visuals.widgets.inactive.rounding = Rounding::same(4.0);
    visuals.widgets.hovered.rounding = Rounding::same(4.0);
    visuals.widgets.active.rounding = Rounding::same(4.0);

    // Destaque / Botões: Laranja Queimado / Terracota (#FF6B35 & #E05A47)
    visuals.selection.bg_fill = Color32::from_rgb(255, 107, 53); // #FF6B35 (Laranja Terracota Rust)
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(224, 90, 71); // #E05A47 (Laranja Queimado)
    visuals.widgets.active.bg_fill = Color32::from_rgb(255, 107, 53);
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(42, 46, 56);

    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0_f32, Color32::from_rgb(52, 57, 70));

    style.spacing.item_spacing = Vec2::new(7.0, 7.0);
    style.spacing.window_margin = egui::Margin::same(10.0);
    style.visuals = visuals;

    ctx.set_style(style);
}
