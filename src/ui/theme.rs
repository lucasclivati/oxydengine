use egui::{Color32, Visuals, Rounding, Stroke, Shadow, Vec2, RichText, Margin, Frame};
use egui::style::{Widgets, WidgetVisuals, Selection, Spacing};
use serde::{Serialize, Deserialize};
use crate::editor::I18nManager;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomTheme {
    pub name: String,
    pub is_preset: bool,
    pub accent_color: [u8; 4],      // Cor de Destaque / Dourado / Laranja / Azul / Verde
    pub bg_dark: [u8; 4],           // Fundo Geral da Janela
    pub bg_panel: [u8; 4],          // Fundo dos Painéis e Janelas Internas
    pub bg_widget: [u8; 4],         // Fundo dos Botões e Inputs Inativos
    pub border_color: [u8; 4],      // Cor das Bordas 1px
    pub text_light: [u8; 4],        // Cor do Texto Normal e Rótulos
    pub text_active_dark: [u8; 4],  // Cor do Texto quando Selecionado / Ativo
    #[serde(default = "default_highlight_text")]
    pub highlight_text: [u8; 4],    // Cor do Texto de Destaque sobre o fundo Accent (ex: título e botão ativo)
}

fn default_highlight_text() -> [u8; 4] {
    [20, 22, 28, 255]
}

impl Default for CustomTheme {
    fn default() -> Self {
        Self::oxyd_gold()
    }
}

impl CustomTheme {
    pub fn oxyd_gold() -> Self {
        Self {
            name: "Oxyd Gold (Yellow)".to_string(),
            is_preset: true,
            accent_color: [253, 199, 52, 255],     // #FDC734
            bg_dark: [24, 26, 32, 255],           // #181A20
            bg_panel: [29, 32, 41, 255],          // #1D2029
            bg_widget: [38, 42, 54, 255],         // #262A36
            border_color: [47, 54, 70, 255],      // #2F3646
            text_light: [230, 235, 245, 255],      // Normal Text & Label Color (#E6EBF5)
            text_active_dark: [114, 114, 104, 255],// #727268
            highlight_text: [20, 22, 28, 255],     // #14161C
        }
    }

    pub fn warm_amber() -> Self {
        Self {
            name: "Warm Amber (Gold)".to_string(),
            is_preset: true,
            accent_color: [245, 158, 11, 255],
            bg_dark: [24, 26, 32, 255],
            bg_panel: [30, 33, 42, 255],
            bg_widget: [38, 42, 54, 255],
            border_color: [48, 54, 70, 255],
            text_light: [230, 235, 245, 255],
            text_active_dark: [255, 255, 255, 255],
            highlight_text: [255, 255, 255, 255],
        }
    }

    pub fn rust_orange() -> Self {
        Self {
            name: "Rust Orange".to_string(),
            is_preset: true,
            accent_color: [255, 107, 53, 255],
            bg_dark: [24, 26, 32, 255],
            bg_panel: [30, 33, 42, 255],
            bg_widget: [38, 42, 54, 255],
            border_color: [48, 54, 70, 255],
            text_light: [230, 235, 245, 255],
            text_active_dark: [255, 255, 255, 255],
            highlight_text: [255, 255, 255, 255],
        }
    }

    pub fn cyber_blue() -> Self {
        Self {
            name: "Cyber Blue".to_string(),
            is_preset: true,
            accent_color: [59, 130, 246, 255],
            bg_dark: [15, 23, 42, 255],
            bg_panel: [30, 41, 59, 255],
            bg_widget: [51, 65, 85, 255],
            border_color: [71, 85, 105, 255],
            text_light: [248, 250, 252, 255],
            text_active_dark: [255, 255, 255, 255],
            highlight_text: [255, 255, 255, 255],
        }
    }

    pub fn emerald_green() -> Self {
        Self {
            name: "Emerald Green".to_string(),
            is_preset: true,
            accent_color: [16, 185, 129, 255],
            bg_dark: [6, 78, 59, 255],
            bg_panel: [6, 95, 70, 255],
            bg_widget: [4, 120, 87, 255],
            border_color: [5, 150, 105, 255],
            text_light: [236, 253, 245, 255],
            text_active_dark: [2, 44, 34, 255],
            highlight_text: [2, 44, 34, 255],
        }
    }

    pub fn unreal_dark() -> Self {
        Self {
            name: "Other Engine".to_string(),
            is_preset: true,
            accent_color: [0, 122, 204, 255],
            bg_dark: [18, 18, 18, 255],
            bg_panel: [30, 30, 30, 255],
            bg_widget: [45, 45, 45, 255],
            border_color: [63, 63, 70, 255],
            text_light: [228, 228, 231, 255],
            text_active_dark: [255, 255, 255, 255],
            highlight_text: [255, 255, 255, 255],
        }
    }

    pub fn light_theme() -> Self {
        Self {
            name: "Clean Light".to_string(),
            is_preset: true,
            accent_color: [37, 99, 235, 255],
            bg_dark: [229, 229, 229, 255],
            bg_panel: [229, 229, 229, 255],
            bg_widget: [163, 163, 163, 255],
            border_color: [208, 220, 235, 255],
            text_light: [15, 23, 42, 255],
            text_active_dark: [51, 51, 51, 255],
            highlight_text: [255, 255, 255, 255],
        }
    }

    pub fn builtin_presets() -> Vec<Self> {
        vec![
            Self::oxyd_gold(),
            Self::warm_amber(),
            Self::rust_orange(),
            Self::cyber_blue(),
            Self::emerald_green(),
            Self::unreal_dark(),
            Self::light_theme(),
        ]
    }

    pub fn revert_to_default(&mut self) {
        let name_match = self.name.clone();
        for p in Self::builtin_presets() {
            if p.name == name_match {
                *self = p;
                return;
            }
        }
        *self = Self::oxyd_gold();
    }
}

pub fn apply_custom_theme(ctx: &egui::Context, theme: &CustomTheme) {
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

    let accent = Color32::from_rgba_unmultiplied(theme.accent_color[0], theme.accent_color[1], theme.accent_color[2], theme.accent_color[3]);
    let bg_dark = Color32::from_rgba_unmultiplied(theme.bg_dark[0], theme.bg_dark[1], theme.bg_dark[2], theme.bg_dark[3]);
    let bg_panel = Color32::from_rgba_unmultiplied(theme.bg_panel[0], theme.bg_panel[1], theme.bg_panel[2], theme.bg_panel[3]);
    let bg_widget = Color32::from_rgba_unmultiplied(theme.bg_widget[0], theme.bg_widget[1], theme.bg_widget[2], theme.bg_widget[3]);
    let border_color = Color32::from_rgba_unmultiplied(theme.border_color[0], theme.border_color[1], theme.border_color[2], theme.border_color[3]);
    let text_light = Color32::from_rgba_unmultiplied(theme.text_light[0], theme.text_light[1], theme.text_light[2], theme.text_light[3]);
    let _text_active_dark = Color32::from_rgba_unmultiplied(theme.text_active_dark[0], theme.text_active_dark[1], theme.text_active_dark[2], theme.text_active_dark[3]);
    let highlight_text = Color32::from_rgba_unmultiplied(theme.highlight_text[0], theme.highlight_text[1], theme.highlight_text[2], theme.highlight_text[3]);

    let mut visuals = Visuals::dark();
    visuals.panel_fill = bg_dark;
    visuals.window_fill = bg_panel;
    visuals.window_shadow = Shadow::NONE;
    visuals.window_rounding = Rounding::same(6.0_f32);
    visuals.window_stroke = Stroke::new(1.0_f32, border_color);

    let inactive = WidgetVisuals {
        bg_fill: bg_widget,
        weak_bg_fill: bg_widget,
        bg_stroke: Stroke::new(1.0_f32, border_color),
        rounding: Rounding::same(4.0_f32),
        fg_stroke: Stroke::new(1.0_f32, text_light),
        expansion: 0.0_f32,
    };

    let hovered = WidgetVisuals {
        bg_fill: Color32::from_rgba_unmultiplied(
            bg_widget.r().saturating_add(15),
            bg_widget.g().saturating_add(15),
            bg_widget.b().saturating_add(15),
            bg_widget.a(),
        ),
        weak_bg_fill: Color32::from_rgba_unmultiplied(
            bg_widget.r().saturating_add(15),
            bg_widget.g().saturating_add(15),
            bg_widget.b().saturating_add(15),
            bg_widget.a(),
        ),
        bg_stroke: Stroke::new(1.0_f32, accent),
        rounding: Rounding::same(4.0_f32),
        fg_stroke: Stroke::new(1.0_f32, text_light),
        expansion: 1.0_f32,
    };

    let active = WidgetVisuals {
        bg_fill: accent,
        weak_bg_fill: accent,
        bg_stroke: Stroke::new(1.0_f32, accent),
        rounding: Rounding::same(4.0_f32),
        fg_stroke: Stroke::new(1.0_f32, highlight_text),
        expansion: 1.0_f32,
    };

    visuals.widgets = Widgets {
        noninteractive: WidgetVisuals {
            bg_fill: bg_panel,
            weak_bg_fill: bg_panel,
            bg_stroke: Stroke::NONE,
            rounding: Rounding::same(4.0_f32),
            fg_stroke: Stroke::new(1.0_f32, text_light),
            expansion: 0.0_f32,
        },
        inactive,
        hovered,
        active,
        open: active,
    };

    visuals.selection = Selection {
        bg_fill: accent,
        stroke: Stroke::new(1.0_f32, highlight_text),
    };

    let mut style = (*ctx.style()).clone();
    style.visuals = visuals;
    style.spacing = Spacing {
        item_spacing: Vec2::new(8.0_f32, 6.0_f32),
        window_margin: Margin::same(8.0_f32),
        button_padding: Vec2::new(10.0_f32, 6.0_f32),
        indent: 14.0_f32,
        interact_size: Vec2::new(40.0_f32, 24.0_f32),
        ..Default::default()
    };

    ctx.set_style(style);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorPickerTab {
    ColorModels,
    ColorSwatches,
}

// ESTADO DO SELETOR DE CORES POPUP ESTILO ILLUSTRATOR
#[derive(Debug, Clone)]
pub struct ColorPickerPopupState {
    pub active_prop: Option<String>,
    pub original_color: [u8; 4],
    pub temp_color: [u8; 4],
    pub hex_input: String,
    pub active_tab: ColorPickerTab,
}

impl Default for ColorPickerPopupState {
    fn default() -> Self {
        Self {
            active_prop: None,
            original_color: [0, 0, 0, 255],
            temp_color: [0, 0, 0, 255],
            hex_input: String::new(),
            active_tab: ColorPickerTab::ColorModels,
        }
    }
}

pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (u16, u8, u8) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == rf {
        60.0 * (((gf - bf) / delta) % 6.0)
    } else if max == gf {
        60.0 * (((bf - rf) / delta) + 2.0)
    } else {
        60.0 * (((rf - gf) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h.round() as u16, (s * 100.0).round() as u8, (v * 100.0).round() as u8)
}

pub fn rgb_to_cmyk(r: u8, g: u8, b: u8) -> (u8, u8, u8, u8) {
    let rf = r as f32 / 255.0;
    let gf = g as f32 / 255.0;
    let bf = b as f32 / 255.0;
    let k_f = 1.0 - rf.max(gf).max(bf);
    if k_f >= 1.0 {
        return (0, 0, 0, 100);
    }
    let c = ((1.0 - rf - k_f) / (1.0 - k_f) * 100.0).clamp(0.0, 100.0) as u8;
    let m = ((1.0 - gf - k_f) / (1.0 - k_f) * 100.0).clamp(0.0, 100.0) as u8;
    let y = ((1.0 - bf - k_f) / (1.0 - k_f) * 100.0).clamp(0.0, 100.0) as u8;
    let k = (k_f * 100.0).clamp(0.0, 100.0) as u8;
    (c, m, y, k)
}

pub fn parse_hex_color(hex: &str) -> Option<[u8; 3]> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() == 6 {
        let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
        let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
        let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
        Some([r, g, b])
    } else {
        None
    }
}

// JANELA INTERATIVA DE PERSONALIZAÇÃO COMPLETA DO TEMA E CORES
pub fn show_theme_manager_window(
    ctx: &egui::Context,
    open: &mut bool,
    current_theme: &mut CustomTheme,
    custom_themes: &mut Vec<CustomTheme>,
    theme_backup: &mut Option<CustomTheme>,
    picker_state: &mut ColorPickerPopupState,
    i18n: &I18nManager,
) {
    if !*open {
        *theme_backup = None;
        picker_state.active_prop = None;
        return;
    }

    if theme_backup.is_none() {
        *theme_backup = Some(current_theme.clone());
    }

    let tr = &i18n.strings;
    let mut theme_changed = false;

    let accent_color = Color32::from_rgba_unmultiplied(
        current_theme.accent_color[0],
        current_theme.accent_color[1],
        current_theme.accent_color[2],
        current_theme.accent_color[3],
    );

    let highlight_text_color = Color32::from_rgba_unmultiplied(
        current_theme.highlight_text[0],
        current_theme.highlight_text[1],
        current_theme.highlight_text[2],
        current_theme.highlight_text[3],
    );

    let normal_text_color = Color32::from_rgba_unmultiplied(
        current_theme.text_light[0],
        current_theme.text_light[1],
        current_theme.text_light[2],
        current_theme.text_light[3],
    );

    // O título da janela quando o header está em Destaque/Amarelo segue HIGHLIGHT TEXT COLOR
    egui::Window::new(RichText::new(&tr.theme_customizer_title).color(highlight_text_color).strong())
        .open(open)
        .resizable(true)
        .default_width(460.0)
        .default_height(560.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading(RichText::new(&tr.theme_presets_heading).color(accent_color).strong());
            ui.label(RichText::new(&tr.theme_subtitle).color(normal_text_color));
            ui.separator();
            ui.add_space(6.0);

            // SEÇÃO 1: PRESETS OFICIAIS
            ui.label(RichText::new(&tr.standard_presets).color(normal_text_color).strong());
            ui.horizontal_wrapped(|ui| {
                for preset in CustomTheme::builtin_presets() {
                    let is_sel = current_theme.name == preset.name;
                    let label_text = if is_sel {
                        RichText::new(&preset.name).color(highlight_text_color).strong()
                    } else {
                        RichText::new(&preset.name).color(normal_text_color)
                    };

                    if ui.selectable_label(is_sel, label_text).clicked() {
                        *current_theme = preset;
                        theme_changed = true;
                    }
                }
            });

            // SEÇÃO 2: TEMAS PERSONALIZADOS DO USUÁRIO
            if !custom_themes.is_empty() {
                ui.add_space(6.0);
                ui.label(RichText::new(&tr.custom_saved_themes).color(normal_text_color).strong());
                ui.horizontal_wrapped(|ui| {
                    let mut to_delete: Option<usize> = None;
                    for (idx, ctheme) in custom_themes.iter().enumerate() {
                        let is_sel = current_theme.name == ctheme.name;
                        ui.horizontal(|ui| {
                            let label_text = if is_sel {
                                RichText::new(&ctheme.name).color(highlight_text_color).strong()
                            } else {
                                RichText::new(&ctheme.name).color(normal_text_color)
                            };

                            if ui.selectable_label(is_sel, label_text).clicked() {
                                *current_theme = ctheme.clone();
                                theme_changed = true;
                            }
                            if ui.button("❌").clicked() {
                                to_delete = Some(idx);
                            }
                        });
                    }

                    if let Some(del_idx) = to_delete {
                        custom_themes.remove(del_idx);
                    }
                });
            }

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);

            // SEÇÃO 3: SELETORES DE CORES INTERATIVOS COM POPUP DE 3 BOTÕES
            ui.heading(RichText::new(&tr.color_matrix_heading).color(normal_text_color).strong());
            ui.add_space(4.0);

            let props: [(&str, &str, &str, [u8; 4]); 8] = [
                ("accent_color", "🎨", &tr.accent_highlight_color, current_theme.accent_color),
                ("bg_dark", "⬛", &tr.main_window_bg, current_theme.bg_dark),
                ("bg_panel", "📦", &tr.panel_dock_bg, current_theme.bg_panel),
                ("bg_widget", "🔘", &tr.inactive_button_fill, current_theme.bg_widget),
                ("border_color", "🔲", &tr.panel_border, current_theme.border_color),
                ("text_light", "📝", &tr.normal_text_label_color, current_theme.text_light),
                ("text_active_dark", "⚡", &tr.selected_active_button_text, current_theme.text_active_dark),
                ("highlight_text", "✨", &tr.highlight_text_color, current_theme.highlight_text),
            ];

            egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
                egui::Grid::new("theme_color_picker_grid")
                    .num_columns(2)
                    .spacing([16.0, 10.0])
                    .show(ui, |ui| {
                        for (prop_id, icon, label, current_rgba) in props {
                            ui.label(RichText::new(format!("{} {}", icon, label)).color(normal_text_color).strong());
                            
                            let color32 = Color32::from_rgba_unmultiplied(current_rgba[0], current_rgba[1], current_rgba[2], current_rgba[3]);
                            let (rect, response) = ui.allocate_exact_size(Vec2::new(48.0, 24.0), egui::Sense::click());
                            
                            ui.painter().rect_filled(rect, Rounding::same(4.0), color32);
                            ui.painter().rect_stroke(rect, Rounding::same(4.0), Stroke::new(1.0_f32, Color32::from_rgb(100, 110, 130)));

                            if response.clicked() {
                                picker_state.active_prop = Some(prop_id.to_string());
                                picker_state.original_color = current_rgba;
                                picker_state.temp_color = current_rgba;
                                picker_state.hex_input = format!("{:02X}{:02X}{:02X}", current_rgba[0], current_rgba[1], current_rgba[2]);
                            }

                            ui.end_row();
                        }
                    });
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(8.0);

            // SEÇÃO 4: AÇÕES PRINCIPAIS DO TEMA
            ui.horizontal_wrapped(|ui| {
                if ui.button(RichText::new(format!("🔄 {}", tr.revert_to_default_btn)).strong()).clicked() {
                    current_theme.revert_to_default();
                    theme_changed = true;
                }

                ui.add_space(4.0);

                if let Some(backup) = theme_backup.clone() {
                    if ui.button(RichText::new(format!("🚫 {}", tr.cancel_changes_btn)).strong()).clicked() {
                        *current_theme = backup;
                        theme_changed = true;
                    }
                }

                ui.add_space(4.0);

                if ui.button(RichText::new(format!("➕ {}", tr.save_as_new_theme_btn)).color(accent_color).strong()).clicked() {
                    let mut new_theme = current_theme.clone();
                    new_theme.name = format!("Custom Theme {}", custom_themes.len() + 1);
                    new_theme.is_preset = false;
                    custom_themes.push(new_theme.clone());
                    *current_theme = new_theme;
                    theme_changed = true;
                }
            });
        });

    // POPUP DEDICADO DO SELETOR DE CORES ESTILO ADOBE ILLUSTRATOR
    if let Some(active_prop) = picker_state.active_prop.clone() {
        let mut is_open = true;
        let mut close_picker = false;

        egui::Window::new("🎨 Color Picker")
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(ctx.screen_rect().center())
            .default_width(580.0)
            .show(ctx, |ui| {
                ui.add_space(6.0);
                ui.label(RichText::new("Select Color:").strong());
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    // ÁREA DA ESQUERDA: ÁREA EXPANDIDA E MAIOR DO COLOR PICKER DE ALTA RESOLUÇÃO
                    ui.vertical(|ui| {
                        let mut c32 = Color32::from_rgba_unmultiplied(
                            picker_state.temp_color[0],
                            picker_state.temp_color[1],
                            picker_state.temp_color[2],
                            picker_state.temp_color[3],
                        );

                        if egui::color_picker::color_picker_color32(ui, &mut c32, egui::color_picker::Alpha::Opaque) {
                            picker_state.temp_color = [c32.r(), c32.g(), c32.b(), c32.a()];
                            picker_state.hex_input = format!("{:02X}{:02X}{:02X}", c32.r(), c32.g(), c32.b());
                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                            theme_changed = true;
                        }
                    });

                    ui.add_space(16.0);

                    // ÁREA DA DIREITA: PAINEL ESTILO ADOBE ILLUSTRATOR COM ANTES/DEPOIS + FORMATOS (RGB, HSB, CMYK, HEX) + BOTÕES OK/CANCEL/SWATCHES
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            // BLINDAGEM VISUAL: CAIXA ANTES E DEPOIS (NOVO x ANTERIOR)
                            let new_c32 = Color32::from_rgba_unmultiplied(picker_state.temp_color[0], picker_state.temp_color[1], picker_state.temp_color[2], picker_state.temp_color[3]);
                            let old_c32 = Color32::from_rgba_unmultiplied(picker_state.original_color[0], picker_state.original_color[1], picker_state.original_color[2], picker_state.original_color[3]);

                            Frame::none()
                                .stroke(Stroke::new(1.0_f32, Color32::BLACK))
                                .show(ui, |ui| {
                                    ui.set_min_size(Vec2::new(64.0, 64.0));
                                    let (r_new, _) = ui.allocate_exact_size(Vec2::new(64.0, 32.0), egui::Sense::hover());
                                    ui.painter().rect_filled(r_new, Rounding::same(0.0), new_c32);

                                    let (r_old, _) = ui.allocate_exact_size(Vec2::new(64.0, 32.0), egui::Sense::hover());
                                    ui.painter().rect_filled(r_old, Rounding::same(0.0), old_c32);
                                });

                            ui.add_space(12.0);

                            // BOTÕES OK / CANCEL / COLOR SWATCHES NA COLUNA LATERAL
                            ui.vertical(|ui| {
                                if ui.add_sized(Vec2::new(110.0, 26.0), egui::Button::new(RichText::new("OK").strong())).clicked() {
                                    update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                    theme_changed = true;
                                    close_picker = true;
                                }
                                ui.add_space(4.0);
                                if ui.add_sized(Vec2::new(110.0, 26.0), egui::Button::new(RichText::new("Cancel").strong())).clicked() {
                                    update_theme_property(current_theme, &active_prop, picker_state.original_color);
                                    theme_changed = true;
                                    close_picker = true;
                                }
                                ui.add_space(4.0);
                                let btn_label = if picker_state.active_tab == ColorPickerTab::ColorModels { "Color Swatches" } else { "Color Models" };
                                if ui.add_sized(Vec2::new(110.0, 26.0), egui::Button::new(RichText::new(btn_label).strong())).clicked() {
                                    picker_state.active_tab = if picker_state.active_tab == ColorPickerTab::ColorModels {
                                        ColorPickerTab::ColorSwatches
                                    } else {
                                        ColorPickerTab::ColorModels
                                    };
                                }
                            });
                        });

                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(6.0);

                        if picker_state.active_tab == ColorPickerTab::ColorModels {
                            // ABA 1: MODELOS DE CORES HSB, RGB, CMYK E INPUT HEX #BC8E1D
                            let (r, g, b) = (picker_state.temp_color[0], picker_state.temp_color[1], picker_state.temp_color[2]);
                            let (h, s, v) = rgb_to_hsv(r, g, b);
                            let (c, m, y, k) = rgb_to_cmyk(r, g, b);

                            egui::Grid::new("illustrator_color_inputs_grid")
                                .num_columns(4)
                                .spacing([10.0, 6.0])
                                .show(ui, |ui| {
                                    // HSB / HSV
                                    ui.label("H:");
                                    ui.label(format!("{}°", h));
                                    ui.label("C:");
                                    ui.label(format!("{}%", c));
                                    ui.end_row();

                                    ui.label("S:");
                                    ui.label(format!("{}%", s));
                                    ui.label("M:");
                                    ui.label(format!("{}%", m));
                                    ui.end_row();

                                    ui.label("B:");
                                    ui.label(format!("{}%", v));
                                    ui.label("Y:");
                                    ui.label(format!("{}%", y));
                                    ui.end_row();

                                    // RGB
                                    ui.label("R:");
                                    ui.label(format!("{}", r));
                                    ui.label("K:");
                                    ui.label(format!("{}%", k));
                                    ui.end_row();

                                    ui.label("G:");
                                    ui.label(format!("{}", g));
                                    ui.label("");
                                    ui.label("");
                                    ui.end_row();

                                    ui.label("B:");
                                    ui.label(format!("{}", b));
                                    ui.label("");
                                    ui.label("");
                                    ui.end_row();
                                });

                            ui.add_space(8.0);

                            // ENTRADA DE COR HEX (#BC8E1D / #FDC734) COM PARSER EM TEMPO REAL
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("#").strong());
                                if ui.add_sized(Vec2::new(100.0, 24.0), egui::TextEdit::singleline(&mut picker_state.hex_input)).changed() {
                                    if let Some([parsed_r, parsed_g, parsed_b]) = parse_hex_color(&picker_state.hex_input) {
                                        picker_state.temp_color = [parsed_r, parsed_g, parsed_b, 255];
                                        update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                        theme_changed = true;
                                    }
                                }
                            });
                        } else {
                            // ABA 2: COLOR SWATCHES COM TODAS AS OUTRAS CORES DO TEMA PARA COMPARAÇÃO
                            ui.label(RichText::new("Theme Color Swatches:").strong());
                            ui.add_space(4.0);

                            let swatches: [(&str, [u8; 4]); 8] = [
                                ("Accent Color", current_theme.accent_color),
                                ("Main Window BG", current_theme.bg_dark),
                                ("Panel & Dock BG", current_theme.bg_panel),
                                ("Inactive Fill", current_theme.bg_widget),
                                ("Panel Border", current_theme.border_color),
                                ("Normal Text", current_theme.text_light),
                                ("Selected Text", current_theme.text_active_dark),
                                ("Highlight Text", current_theme.highlight_text),
                            ];

                            egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                                for (s_name, s_rgba) in swatches {
                                    let swatch_c32 = Color32::from_rgba_unmultiplied(s_rgba[0], s_rgba[1], s_rgba[2], s_rgba[3]);
                                    ui.horizontal(|ui| {
                                        let (r_box, btn_res) = ui.allocate_exact_size(Vec2::new(24.0, 18.0), egui::Sense::click());
                                        ui.painter().rect_filled(r_box, Rounding::same(3.0), swatch_c32);
                                        ui.painter().rect_stroke(r_box, Rounding::same(3.0), Stroke::new(1.0_f32, Color32::GRAY));

                                        if ui.selectable_label(false, s_name).clicked() || btn_res.clicked() {
                                            picker_state.temp_color = s_rgba;
                                            picker_state.hex_input = format!("{:02X}{:02X}{:02X}", s_rgba[0], s_rgba[1], s_rgba[2]);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    });
                                }
                            });
                        }
                    });
                });
            });

        if !is_open || close_picker {
            picker_state.active_prop = None;
        }
    }

    if theme_changed {
        apply_custom_theme(ctx, current_theme);
    }
}

fn update_theme_property(theme: &mut CustomTheme, prop_id: &str, rgba: [u8; 4]) {
    match prop_id {
        "accent_color" => theme.accent_color = rgba,
        "bg_dark" => theme.bg_dark = rgba,
        "bg_panel" => theme.bg_panel = rgba,
        "bg_widget" => theme.bg_widget = rgba,
        "border_color" => theme.border_color = rgba,
        "text_light" => theme.text_light = rgba,
        "text_active_dark" => theme.text_active_dark = rgba,
        "highlight_text" => theme.highlight_text = rgba,
        _ => {}
    }
}
