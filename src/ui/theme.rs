use egui::{Color32, Visuals, Rounding, Stroke, Shadow, Vec2};
use egui::style::{Widgets, WidgetVisuals, Selection, Spacing};

pub fn apply_oxyd_theme(ctx: &egui::Context) {
    let mut font_defs = egui::FontDefinitions::default();
    if let Ok(font_data) = std::fs::read("assets/fonts/Inter-Regular.ttf") {
        font_defs.font_data.insert(
            "Inter-Regular".to_string(),
            egui::FontData::from_owned(font_data),
        );
        font_defs.families.get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "Inter-Regular".to_string());
    }
    ctx.set_fonts(font_defs);

    // Paleta Antigravity Rust & Warm Gold
    let bg_dark = Color32::from_rgb(24, 26, 32);       // Fundo da interface (#181A20 - Cinza Ardoisia Frio)
    let bg_panel = Color32::from_rgb(30, 33, 42);      // Painéis e Janelas
    let bg_widget = Color32::from_rgb(38, 42, 54);     // Botões e Caixas
    let rust_orange = Color32::from_rgb(255, 107, 53);  // Destaques Ferrugem Antigravity (#FF6B35)
    let warm_gold = Color32::from_rgb(245, 158, 11);   // Destaques Ouro / Âmbar (#F59E0B)
    let border_color = Color32::from_rgb(48, 54, 70);

    let mut visuals = Visuals::dark();
    visuals.panel_fill = bg_dark;
    visuals.window_fill = bg_panel;
    visuals.window_shadow = Shadow::NONE;
    visuals.window_rounding = Rounding::same(6.0);
    visuals.window_stroke = Stroke::new(1.0, border_color);

    let inactive = WidgetVisuals {
        bg_fill: bg_widget,
        weak_bg_fill: bg_widget,
        bg_stroke: Stroke::new(1.0, border_color),
        rounding: Rounding::same(4.0),
        fg_stroke: Stroke::new(1.0, Color32::from_rgb(210, 215, 225)),
        expansion: 0.0,
    };

    let hovered = WidgetVisuals {
        bg_fill: Color32::from_rgb(52, 58, 74),
        weak_bg_fill: Color32::from_rgb(52, 58, 74),
        bg_stroke: Stroke::new(1.0, rust_orange),
        rounding: Rounding::same(4.0),
        fg_stroke: Stroke::new(1.0, Color32::WHITE),
        expansion: 1.0,
    };

    let active = WidgetVisuals {
        bg_fill: rust_orange,
        weak_bg_fill: rust_orange,
        bg_stroke: Stroke::new(1.0, warm_gold),
        rounding: Rounding::same(4.0),
        fg_stroke: Stroke::new(1.0, Color32::WHITE),
        expansion: 1.0,
    };

    visuals.widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: bg_panel,
            weak_bg_fill: bg_panel,
            bg_stroke: Stroke::NONE,
            rounding: Rounding::same(4.0),
            fg_stroke: Stroke::new(1.0, Color32::from_rgb(170, 180, 195)),
            expansion: 0.0,
        },
        inactive,
        hovered,
        active,
        open: active,
    };

    visuals.selection = Selection {
        bg_fill: rust_orange,
        stroke: Stroke::new(1.0, warm_gold),
    };

    let mut style = (*ctx.style()).clone();
    style.visuals = visuals;
    style.spacing = Spacing {
        item_spacing: Vec2::new(8.0, 6.0),
        window_margin: egui::Margin::same(8.0),
        button_padding: Vec2::new(10.0, 6.0),
        indent: 14.0,
        interact_size: Vec2::new(40.0, 24.0),
        ..Default::default()
    };

    ctx.set_style(style);
}
