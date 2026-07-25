use egui::{Color32, Visuals, Rounding, Stroke, Shadow, Vec2, RichText, Margin, Frame, Pos2, Rect};
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
        Self::warm_amber()
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

    // REGISTRO DE FONTES INTERNACIONAIS DE FALLBACK (CJK CHINÊS, JAPONÊS, COREANO, CIRÍLICO, ÁRABE, HINDI, EMOJIS)
    let system_fallback_fonts = [
        ("C:\\Windows\\Fonts\\msyh.ttc", "MSYaHei"),       // Chinês Simplificado / Mandarim
        ("C:\\Windows\\Fonts\\simsun.ttc", "SimSun"),        // Chinês
        ("C:\\Windows\\Fonts\\msgothic.ttc", "MSGothic"),   // Japonês
        ("C:\\Windows\\Fonts\\malgun.ttf", "MalgunGothic"), // Coreano
        ("C:\\Windows\\Fonts\\Nirmala.ttf", "NirmalaUI"),   // Hindi / Devanagari
        ("C:\\Windows\\Fonts\\arial.ttf", "ArialSystem"),   // Cirílico / Árabe / Hebraico
        ("C:\\Windows\\Fonts\\seguiemj.ttf", "SegoeEmoji"), // Emojis
    ];

    for (f_path, f_name) in system_fallback_fonts {
        if let Ok(data) = std::fs::read(f_path) {
            font_defs.font_data.insert(f_name.to_string(), egui::FontData::from_owned(data));
            font_defs.families.get_mut(&egui::FontFamily::Proportional).unwrap().push(f_name.to_string());
            font_defs.families.get_mut(&egui::FontFamily::Monospace).unwrap().push(f_name.to_string());
        }
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

// ESTADO COMPLETO DO SELETOR DE CORES POPUP - CLONE 100% EXATO AO ADOBE ILLUSTRATOR
#[derive(Debug, Clone)]
pub struct ColorPickerPopupState {
    pub active_prop: Option<String>,
    pub original_color: [u8; 4],
    pub temp_color: [u8; 4],
    pub r_input: String,
    pub g_input: String,
    pub b_input: String,
    pub h_input: String,
    pub s_input: String,
    pub v_input: String,
    pub hex_input: String,
    pub c_input: String,
    pub m_input: String,
    pub y_input: String,
    pub k_input: String,
    pub selected_radio: String, // "H", "S", "B_val", "R", "G", "B"
    pub active_tab: ColorPickerTab,
}

impl Default for ColorPickerPopupState {
    fn default() -> Self {
        Self {
            active_prop: None,
            original_color: [0, 0, 0, 255],
            temp_color: [0, 0, 0, 255],
            r_input: "0".to_string(),
            g_input: "0".to_string(),
            b_input: "0".to_string(),
            h_input: "0".to_string(),
            s_input: "0".to_string(),
            v_input: "0".to_string(),
            hex_input: "000000".to_string(),
            c_input: "0".to_string(),
            m_input: "0".to_string(),
            y_input: "0".to_string(),
            k_input: "100".to_string(),
            selected_radio: "H".to_string(),
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

pub fn hsv_to_rgb(h: u16, s: u8, v: u8) -> [u8; 3] {
    let h_f = (h % 360) as f32;
    let s_f = s as f32 / 100.0;
    let v_f = v as f32 / 100.0;

    let c = v_f * s_f;
    let x = c * (1.0 - ((h_f / 60.0) % 2.0 - 1.0).abs());
    let m = v_f - c;

    let (r_f, g_f, b_f) = match (h_f / 60.0) as u8 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    [
        ((r_f + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g_f + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b_f + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
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

pub fn cmyk_to_rgb(c: u8, m: u8, y: u8, k: u8) -> [u8; 3] {
    let c_f = c as f32 / 100.0;
    let m_f = m as f32 / 100.0;
    let y_f = y as f32 / 100.0;
    let k_f = k as f32 / 100.0;

    let r = 255.0 * (1.0 - c_f) * (1.0 - k_f);
    let g = 255.0 * (1.0 - m_f) * (1.0 - k_f);
    let b = 255.0 * (1.0 - y_f) * (1.0 - k_f);

    [
        r.round().clamp(0.0, 255.0) as u8,
        g.round().clamp(0.0, 255.0) as u8,
        b.round().clamp(0.0, 255.0) as u8,
    ]
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

pub fn sync_color_picker_inputs(state: &mut ColorPickerPopupState, rgba: [u8; 4]) {
    state.temp_color = rgba;
    let (r, g, b) = (rgba[0], rgba[1], rgba[2]);
    let (h, s, v) = rgb_to_hsv(r, g, b);
    let (c, m, y, k) = rgb_to_cmyk(r, g, b);

    state.r_input = format!("{}", r);
    state.g_input = format!("{}", g);
    state.b_input = format!("{}", b);

    state.h_input = format!("{}", h);
    state.s_input = format!("{}", s);
    state.v_input = format!("{}", v);

    state.c_input = format!("{}", c);
    state.m_input = format!("{}", m);
    state.y_input = format!("{}", y);
    state.k_input = format!("{}", k);

    state.hex_input = format!("{:02X}{:02X}{:02X}", r, g, b);
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

    // Título da janela principal em HIGHLIGHT TEXT COLOR para alto contraste no header selecionado
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
                                sync_color_picker_inputs(picker_state, current_rgba);
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

    // POPUP DEDICADO DO SELETOR DE CORES - CLONE 100% FIEL E IDÊNTICO AO ADOBE ILLUSTRATOR (IMAGENS 1 A 5)
    if let Some(active_prop) = picker_state.active_prop.clone() {
        let mut is_open = true;
        let mut close_picker = false;

        // Estilo escuro idêntico ao Photoshop / Illustrator (#383838)
        let ill_frame = Frame::none()
            .fill(Color32::from_rgb(56, 56, 56))
            .rounding(Rounding::same(4.0))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(20, 20, 20)))
            .inner_margin(Margin::same(12.0));

        egui::Window::new("Color Picker")
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(ctx.screen_rect().center())
            .frame(ill_frame)
            .show(ctx, |ui| {
                ui.label(RichText::new("Select Color:").color(Color32::from_rgb(220, 220, 220)).strong());
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    let (r, g, b) = (picker_state.temp_color[0], picker_state.temp_color[1], picker_state.temp_color[2]);
                    let (h, current_s, current_v) = rgb_to_hsv(r, g, b);
                    let mode_str = picker_state.selected_radio.clone();

                    // 1. ÁREA DA ESQUERDA: QUADRADO DE ESPECTRO 2D (250x250px) DINÂMICO + SLIDER VERTICAL
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            // QUADRADO DE ESPECTRO 2D (250x250px)
                            let (rect_2d, resp_2d) = ui.allocate_exact_size(Vec2::new(250.0, 250.0), egui::Sense::click_and_drag());

                            // RENDERIZAÇÃO EM GRID DA MATRIZ 2D DE CORES DO ILLUSTRATOR COM BASE NO MODO SELECIONADO (H, S, B, R, G, B)
                            let steps = 100;
                            let cell_w = 250.0 / steps as f32;
                            let cell_h = 250.0 / steps as f32;

                            for y_i in 0..steps {
                                let norm_y = y_i as f32 / steps as f32;
                                let y_val_inv = 1.0 - norm_y;
                                for x_i in 0..steps {
                                    let norm_x = x_i as f32 / steps as f32;

                                    let cell_rgb = match mode_str.as_str() {
                                        "H" => {
                                            // X = Saturation (0..100%), Y = Brightness (100..0%)
                                            hsv_to_rgb(h, (norm_x * 100.0) as u8, (y_val_inv * 100.0) as u8)
                                        }
                                        "S" => {
                                            // X = Hue (0..360°), Y = Brightness (100..0%)
                                            hsv_to_rgb((norm_x * 360.0) as u16, current_s, (y_val_inv * 100.0) as u8)
                                        }
                                        "B_val" => {
                                            // X = Hue (0..360°), Y = Saturation (100..0%)
                                            hsv_to_rgb((norm_x * 360.0) as u16, (y_val_inv * 100.0) as u8, current_v)
                                        }
                                        "R" => {
                                            // X = Blue (0..255), Y = Green (255..0)
                                            [r, (y_val_inv * 255.0) as u8, (norm_x * 255.0) as u8]
                                        }
                                        "G" => {
                                            // X = Blue (0..255), Y = Red (255..0)
                                            [(y_val_inv * 255.0) as u8, g, (norm_x * 255.0) as u8]
                                        }
                                        "B" => {
                                            // X = Red (0..255), Y = Green (255..0)
                                            [(norm_x * 255.0) as u8, (y_val_inv * 255.0) as u8, b]
                                        }
                                        _ => hsv_to_rgb(h, (norm_x * 100.0) as u8, (y_val_inv * 100.0) as u8),
                                    };

                                    let cell_rect = Rect::from_min_size(
                                        Pos2::new(rect_2d.min.x + x_i as f32 * cell_w, rect_2d.min.y + y_i as f32 * cell_h),
                                        Vec2::new(cell_w + 0.5, cell_h + 0.5),
                                    );
                                    ui.painter().rect_filled(cell_rect, Rounding::ZERO, Color32::from_rgb(cell_rgb[0], cell_rgb[1], cell_rgb[2]));
                                }
                            }

                            // POSIÇÃO E DESENHO DO CURSOR CIRCULAR `○` DO SELETOR 2D DO ILLUSTRATOR
                            let (point_x, point_y) = match mode_str.as_str() {
                                "H" => (
                                    rect_2d.min.x + (current_s as f32 / 100.0) * rect_2d.width(),
                                    rect_2d.min.y + (1.0 - (current_v as f32 / 100.0)) * rect_2d.height(),
                                ),
                                "S" => (
                                    rect_2d.min.x + (h as f32 / 360.0) * rect_2d.width(),
                                    rect_2d.min.y + (1.0 - (current_v as f32 / 100.0)) * rect_2d.height(),
                                ),
                                "B_val" => (
                                    rect_2d.min.x + (h as f32 / 360.0) * rect_2d.width(),
                                    rect_2d.min.y + (1.0 - (current_s as f32 / 100.0)) * rect_2d.height(),
                                ),
                                "R" => (
                                    rect_2d.min.x + (b as f32 / 255.0) * rect_2d.width(),
                                    rect_2d.min.y + (1.0 - (g as f32 / 255.0)) * rect_2d.height(),
                                ),
                                "G" => (
                                    rect_2d.min.x + (b as f32 / 255.0) * rect_2d.width(),
                                    rect_2d.min.y + (1.0 - (r as f32 / 255.0)) * rect_2d.height(),
                                ),
                                "B" => (
                                    rect_2d.min.x + (r as f32 / 255.0) * rect_2d.width(),
                                    rect_2d.min.y + (1.0 - (g as f32 / 255.0)) * rect_2d.height(),
                                ),
                                _ => (rect_2d.center().x, rect_2d.center().y),
                            };

                            if resp_2d.dragged() || resp_2d.clicked() {
                                if let Some(pos) = resp_2d.interact_pointer_pos() {
                                    let rel_x = ((pos.x - rect_2d.min.x) / rect_2d.width()).clamp(0.0, 1.0);
                                    let rel_y = ((pos.y - rect_2d.min.y) / rect_2d.height()).clamp(0.0, 1.0);
                                    let inv_y = 1.0 - rel_y;

                                    let new_rgb = match mode_str.as_str() {
                                        "H" => hsv_to_rgb(h, (rel_x * 100.0) as u8, (inv_y * 100.0) as u8),
                                        "S" => hsv_to_rgb((rel_x * 360.0) as u16, current_s, (inv_y * 100.0) as u8),
                                        "B_val" => hsv_to_rgb((rel_x * 360.0) as u16, (inv_y * 100.0) as u8, current_v),
                                        "R" => [r, (inv_y * 255.0) as u8, (rel_x * 255.0) as u8],
                                        "G" => [(inv_y * 255.0) as u8, g, (rel_x * 255.0) as u8],
                                        "B" => [(rel_x * 255.0) as u8, (inv_y * 255.0) as u8, b],
                                        _ => hsv_to_rgb(h, (rel_x * 100.0) as u8, (inv_y * 100.0) as u8),
                                    };

                                    sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                    update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                    theme_changed = true;
                                }
                            }

                            // BORDA 1PX PRETA E CURSOR CIRCULAR `○`
                            ui.painter().rect_stroke(rect_2d, Rounding::ZERO, Stroke::new(1.0_f32, Color32::BLACK));
                            ui.painter().circle_stroke(Pos2::new(point_x, point_y), 5.0, Stroke::new(1.5_f32, Color32::BLACK));
                            ui.painter().circle_stroke(Pos2::new(point_x, point_y), 4.0, Stroke::new(1.5_f32, Color32::WHITE));

                            ui.add_space(8.0);

                            // SLIDER VERTICAL BARRA (20x250px) DINÂMICO CONFORME O MODO (H, S, B, R, G, B)
                            let (rect_hue, resp_hue) = ui.allocate_exact_size(Vec2::new(20.0, 250.0), egui::Sense::click_and_drag());

                            for y_i in 0..250 {
                                let norm_y = 1.0 - (y_i as f32 / 250.0);
                                let line_rgb = match mode_str.as_str() {
                                    "H" => hsv_to_rgb((norm_y * 360.0) as u16, 100, 100),
                                    "S" => hsv_to_rgb(h, (norm_y * 100.0) as u8, current_v),
                                    "B_val" => hsv_to_rgb(h, current_s, (norm_y * 100.0) as u8),
                                    "R" => [(norm_y * 255.0) as u8, g, b],
                                    "G" => [r, (norm_y * 255.0) as u8, b],
                                    "B" => [r, g, (norm_y * 255.0) as u8],
                                    _ => hsv_to_rgb((norm_y * 360.0) as u16, 100, 100),
                                };

                                let line_rect = Rect::from_min_size(Pos2::new(rect_hue.min.x, rect_hue.min.y + y_i as f32), Vec2::new(20.0, 1.0));
                                ui.painter().rect_filled(line_rect, Rounding::ZERO, Color32::from_rgb(line_rgb[0], line_rgb[1], line_rgb[2]));
                            }
                            ui.painter().rect_stroke(rect_hue, Rounding::ZERO, Stroke::new(1.0_f32, Color32::BLACK));

                            // EVENTO DE CLIQUE / ARRASTE NO SLIDER VERTICAL
                            if resp_hue.dragged() || resp_hue.clicked() {
                                if let Some(pos) = resp_hue.interact_pointer_pos() {
                                    let rel_y = ((pos.y - rect_hue.min.y) / rect_hue.height()).clamp(0.0, 1.0);
                                    let inv_y = 1.0 - rel_y;

                                    let new_rgb = match mode_str.as_str() {
                                        "H" => hsv_to_rgb((inv_y * 360.0) as u16, current_s, current_v),
                                        "S" => hsv_to_rgb(h, (inv_y * 100.0) as u8, current_v),
                                        "B_val" => hsv_to_rgb(h, current_s, (inv_y * 100.0) as u8),
                                        "R" => [(inv_y * 255.0) as u8, g, b],
                                        "G" => [r, (inv_y * 255.0) as u8, b],
                                        "B" => [r, g, (inv_y * 255.0) as u8],
                                        _ => hsv_to_rgb((inv_y * 360.0) as u16, current_s, current_v),
                                    };

                                    sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                    update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                    theme_changed = true;
                                }
                            }

                            // RENDERIZAÇÃO GEOMÉTRICA PERFEITA DAS SETAS ◀ ▶ DO SLIDER (SEM QUADRADOS ☒ OU ERROS DE FONTE)
                            let slider_ratio = match mode_str.as_str() {
                                "H" => 1.0 - (h as f32 / 360.0),
                                "S" => 1.0 - (current_s as f32 / 100.0),
                                "B_val" => 1.0 - (current_v as f32 / 100.0),
                                "R" => 1.0 - (r as f32 / 255.0),
                                "G" => 1.0 - (g as f32 / 255.0),
                                "B" => 1.0 - (b as f32 / 255.0),
                                _ => 1.0 - (h as f32 / 360.0),
                            };

                            let arrow_y = rect_hue.min.y + slider_ratio.clamp(0.0, 1.0) * rect_hue.height();

                            // Seta Esquerda apontando para a direita ▶
                            let left_arrow_poly = vec![
                                Pos2::new(rect_hue.min.x - 7.0, arrow_y - 5.0),
                                Pos2::new(rect_hue.min.x - 1.0, arrow_y),
                                Pos2::new(rect_hue.min.x - 7.0, arrow_y + 5.0),
                            ];
                            ui.painter().add(egui::Shape::convex_polygon(left_arrow_poly, Color32::from_rgb(220, 220, 220), Stroke::new(1.0_f32, Color32::BLACK)));

                            // Seta Direita apontando para a esquerda ◀
                            let right_arrow_poly = vec![
                                Pos2::new(rect_hue.max.x + 7.0, arrow_y - 5.0),
                                Pos2::new(rect_hue.max.x + 1.0, arrow_y),
                                Pos2::new(rect_hue.max.x + 7.0, arrow_y + 5.0),
                            ];
                            ui.painter().add(egui::Shape::convex_polygon(right_arrow_poly, Color32::from_rgb(220, 220, 220), Stroke::new(1.0_f32, Color32::BLACK)));
                        });
                    });

                    ui.add_space(16.0);

                    // 2. COLUNA DIREITA: BLINDAGEM ANTES/DEPOIS + PAINEL DE BOTÕES + CAMPOS EDITÁVEIS RECALCULADOS (H, S, B, R, G, B, CMYK, HEX)
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            // CAIXA DE PREVIA ANTES / DEPOIS (NOVO x ANTERIOR DO ILLUSTRATOR)
                            let new_c32 = Color32::from_rgba_unmultiplied(picker_state.temp_color[0], picker_state.temp_color[1], picker_state.temp_color[2], picker_state.temp_color[3]);
                            let old_c32 = Color32::from_rgba_unmultiplied(picker_state.original_color[0], picker_state.original_color[1], picker_state.original_color[2], picker_state.original_color[3]);

                            let (prev_rect, _) = ui.allocate_exact_size(Vec2::new(54.0, 70.0), egui::Sense::hover());
                            let top_half = Rect::from_min_size(prev_rect.min, Vec2::new(54.0, 35.0));
                            let bot_half = Rect::from_min_size(Pos2::new(prev_rect.min.x, prev_rect.min.y + 35.0), Vec2::new(54.0, 35.0));

                            ui.painter().rect_filled(top_half, Rounding::ZERO, new_c32);
                            ui.painter().rect_filled(bot_half, Rounding::ZERO, old_c32);
                            ui.painter().rect_stroke(prev_rect, Rounding::ZERO, Stroke::new(1.0_f32, Color32::BLACK));

                            ui.add_space(16.0);

                            // BOTÕES PILL DO ADOBE ILLUSTRATOR: [ OK ], [ Cancel ], [ Color Swatches ]
                            ui.vertical(|ui| {
                                let pill_style = egui::Button::new(RichText::new("OK").color(Color32::WHITE).strong())
                                    .rounding(Rounding::same(12.0))
                                    .stroke(Stroke::new(1.0_f32, Color32::WHITE))
                                    .fill(Color32::from_rgb(50, 50, 50));

                                if ui.add_sized(Vec2::new(115.0, 26.0), pill_style).clicked() {
                                    update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                    theme_changed = true;
                                    close_picker = true;
                                }

                                ui.add_space(4.0);

                                let cancel_pill = egui::Button::new(RichText::new("Cancel").color(Color32::WHITE).strong())
                                    .rounding(Rounding::same(12.0))
                                    .stroke(Stroke::new(1.0_f32, Color32::WHITE))
                                    .fill(Color32::from_rgb(50, 50, 50));

                                if ui.add_sized(Vec2::new(115.0, 26.0), cancel_pill).clicked() {
                                    update_theme_property(current_theme, &active_prop, picker_state.original_color);
                                    theme_changed = true;
                                    close_picker = true;
                                }

                                ui.add_space(4.0);

                                let tab_name = if picker_state.active_tab == ColorPickerTab::ColorModels { "Color Swatches" } else { "Color Models" };
                                let swatches_pill = egui::Button::new(RichText::new(tab_name).color(Color32::WHITE).strong())
                                    .rounding(Rounding::same(12.0))
                                    .stroke(Stroke::new(1.0_f32, Color32::WHITE))
                                    .fill(Color32::from_rgb(50, 50, 50));

                                if ui.add_sized(Vec2::new(115.0, 26.0), swatches_pill).clicked() {
                                    picker_state.active_tab = if picker_state.active_tab == ColorPickerTab::ColorModels {
                                        ColorPickerTab::ColorSwatches
                                    } else {
                                        ColorPickerTab::ColorModels
                                    };
                                }
                            });
                        });

                        ui.add_space(10.0);

                        if picker_state.active_tab == ColorPickerTab::ColorModels {
                            // CAMPOS EDITÁVEIS COM RECÁLCULO AUTOMÁTICO EM TEMPO REAL DE TODOS OS CAMPOS DO ILLUSTRATOR
                            egui::Grid::new("illustrator_inputs_grid")
                                .num_columns(4)
                                .spacing([8.0, 6.0])
                                .show(ui, |ui| {
                                    // 1. HSB: Hue H:
                                    ui.horizontal(|ui| {
                                        if ui.radio_value(&mut picker_state.selected_radio, "H".to_string(), "").clicked() {
                                            // Ao clicar no radio button, seleciona o modo sem alterar a cor
                                        }
                                        ui.label(RichText::new("H:").color(Color32::from_rgb(220, 220, 220)));
                                    });
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.h_input)).changed() {
                                        if let Ok(val) = picker_state.h_input.trim_end_matches('°').trim().parse::<u16>() {
                                            let new_rgb = hsv_to_rgb(val, current_s, current_v);
                                            sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }

                                    // CMYK: C:
                                    ui.label(RichText::new("C:").color(Color32::from_rgb(220, 220, 220)));
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.c_input)).changed() {
                                        if let Ok(c_val) = picker_state.c_input.trim_end_matches('%').trim().parse::<u8>() {
                                            let (_, m, y, k) = rgb_to_cmyk(r, g, b);
                                            let new_rgb = cmyk_to_rgb(c_val, m, y, k);
                                            sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }
                                    ui.end_row();

                                    // 2. HSB: Saturation S:
                                    ui.horizontal(|ui| {
                                        if ui.radio_value(&mut picker_state.selected_radio, "S".to_string(), "").clicked() {}
                                        ui.label(RichText::new("S:").color(Color32::from_rgb(220, 220, 220)));
                                    });
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.s_input)).changed() {
                                        if let Ok(s_val) = picker_state.s_input.trim_end_matches('%').trim().parse::<u8>() {
                                            let new_rgb = hsv_to_rgb(h, s_val, current_v);
                                            sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }

                                    // CMYK: M:
                                    ui.label(RichText::new("M:").color(Color32::from_rgb(220, 220, 220)));
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.m_input)).changed() {
                                        if let Ok(m_val) = picker_state.m_input.trim_end_matches('%').trim().parse::<u8>() {
                                            let (c, _, y, k) = rgb_to_cmyk(r, g, b);
                                            let new_rgb = cmyk_to_rgb(c, m_val, y, k);
                                            sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }
                                    ui.end_row();

                                    // 3. HSB: Brightness B:
                                    ui.horizontal(|ui| {
                                        if ui.radio_value(&mut picker_state.selected_radio, "B_val".to_string(), "").clicked() {}
                                        ui.label(RichText::new("B:").color(Color32::from_rgb(220, 220, 220)));
                                    });
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.v_input)).changed() {
                                        if let Ok(v_val) = picker_state.v_input.trim_end_matches('%').trim().parse::<u8>() {
                                            let new_rgb = hsv_to_rgb(h, current_s, v_val);
                                            sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }

                                    // CMYK: Y:
                                    ui.label(RichText::new("Y:").color(Color32::from_rgb(220, 220, 220)));
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.y_input)).changed() {
                                        if let Ok(y_val) = picker_state.y_input.trim_end_matches('%').trim().parse::<u8>() {
                                            let (c, m, _, k) = rgb_to_cmyk(r, g, b);
                                            let new_rgb = cmyk_to_rgb(c, m, y_val, k);
                                            sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }
                                    ui.end_row();

                                    // 4. RGB: Red R:
                                    ui.horizontal(|ui| {
                                        if ui.radio_value(&mut picker_state.selected_radio, "R".to_string(), "").clicked() {}
                                        ui.label(RichText::new("R:").color(Color32::from_rgb(220, 220, 220)));
                                    });
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.r_input)).changed() {
                                        if let Ok(r_val) = picker_state.r_input.trim().parse::<u8>() {
                                            let new_rgba = [r_val, g, b, 255];
                                            sync_color_picker_inputs(picker_state, new_rgba);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }

                                    // CMYK: K:
                                    ui.label(RichText::new("K:").color(Color32::from_rgb(220, 220, 220)));
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.k_input)).changed() {
                                        if let Ok(k_val) = picker_state.k_input.trim_end_matches('%').trim().parse::<u8>() {
                                            let (c, m, y, _) = rgb_to_cmyk(r, g, b);
                                            let new_rgb = cmyk_to_rgb(c, m, y, k_val);
                                            sync_color_picker_inputs(picker_state, [new_rgb[0], new_rgb[1], new_rgb[2], 255]);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }
                                    ui.end_row();

                                    // 5. RGB: Green G:
                                    ui.horizontal(|ui| {
                                        if ui.radio_value(&mut picker_state.selected_radio, "G".to_string(), "").clicked() {}
                                        ui.label(RichText::new("G:").color(Color32::from_rgb(220, 220, 220)));
                                    });
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.g_input)).changed() {
                                        if let Ok(g_val) = picker_state.g_input.trim().parse::<u8>() {
                                            let new_rgba = [r, g_val, b, 255];
                                            sync_color_picker_inputs(picker_state, new_rgba);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }
                                    ui.label("");
                                    ui.label("");
                                    ui.end_row();

                                    // 6. RGB: Blue B:
                                    ui.horizontal(|ui| {
                                        if ui.radio_value(&mut picker_state.selected_radio, "B".to_string(), "").clicked() {}
                                        ui.label(RichText::new("B:").color(Color32::from_rgb(220, 220, 220)));
                                    });
                                    if ui.add_sized(Vec2::new(55.0, 22.0), egui::TextEdit::singleline(&mut picker_state.b_input)).changed() {
                                        if let Ok(b_val) = picker_state.b_input.trim().parse::<u8>() {
                                            let new_rgba = [r, g, b_val, 255];
                                            sync_color_picker_inputs(picker_state, new_rgba);
                                            update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                            theme_changed = true;
                                        }
                                    }
                                    ui.label("");
                                    ui.label("");
                                    ui.end_row();
                                });

                            ui.add_space(8.0);

                            // CAMPO # HEX (#BC8E1D / #FDC734) COM PARSER E RECÁLCULO INSTANTÂNEO DE TODOS OS CAMPOS
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("#").color(Color32::from_rgb(220, 220, 220)).strong());
                                if ui.add_sized(Vec2::new(100.0, 22.0), egui::TextEdit::singleline(&mut picker_state.hex_input)).changed() {
                                    if let Some([parsed_r, parsed_g, parsed_b]) = parse_hex_color(&picker_state.hex_input) {
                                        sync_color_picker_inputs(picker_state, [parsed_r, parsed_g, parsed_b, 255]);
                                        update_theme_property(current_theme, &active_prop, picker_state.temp_color);
                                        theme_changed = true;
                                    }
                                }
                            });
                        } else {
                            // ABA COLOR SWATCHES
                            ui.label(RichText::new("Color Swatches:").color(Color32::from_rgb(220, 220, 220)).strong());
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

                            egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                                for (s_name, s_rgba) in swatches {
                                    let swatch_c32 = Color32::from_rgba_unmultiplied(s_rgba[0], s_rgba[1], s_rgba[2], s_rgba[3]);
                                    ui.horizontal(|ui| {
                                        let (r_box, btn_res) = ui.allocate_exact_size(Vec2::new(24.0, 18.0), egui::Sense::click());
                                        ui.painter().rect_filled(r_box, Rounding::same(3.0), swatch_c32);
                                        ui.painter().rect_stroke(r_box, Rounding::same(3.0), Stroke::new(1.0_f32, Color32::GRAY));

                                        if ui.selectable_label(false, RichText::new(s_name).color(Color32::WHITE)).clicked() || btn_res.clicked() {
                                            sync_color_picker_inputs(picker_state, s_rgba);
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
