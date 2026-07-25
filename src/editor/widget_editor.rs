use egui::{Color32, Vec2, Pos2, Rect, RichText, Frame, Rounding, Stroke, Margin};
use serde::{Serialize, Deserialize};
use crate::editor::I18nManager;
use crate::scene::World;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EasingType {
    Linear,
    EaseOutQuad,
    EaseInOutCubic,
    SpringBounce,
}

impl Default for EasingType {
    fn default() -> Self {
        EasingType::EaseOutQuad
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WidgetElementType {
    Button,
    TextLabel,
    Image,
    ProgressBar,
    Slider,
    PanelBox,
}

fn default_true() -> bool { true }
fn default_smoothing_duration() -> f32 { 0.35 }
fn default_one() -> f32 { 1.0 }
fn default_trail_value() -> f32 { 1.0 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetElement {
    pub id: usize,
    pub name: String,
    pub element_type: WidgetElementType,
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: f32,
    pub height: f32,
    pub text: String,
    pub font_size: f32,
    pub text_color: [u8; 4],
    pub bg_color: [u8; 4],
    pub border_color: [u8; 4],
    pub onclick_action: String,

    // EFEITOS DE ANIMAÇÃO E SUAVIZAÇÃO DE VALORES (EX: BARRA DE VIDA DESCENDO SUAVEMENTE AO TOMAR DANO)
    #[serde(default = "default_true")]
    pub is_smoothed: bool,
    #[serde(default = "default_smoothing_duration")]
    pub smoothing_duration: f32, // Tempo de suavização em segundos (ex: 0.35s, 0.5s, 1.0s)
    #[serde(default = "default_one")]
    pub current_value: f32,      // Valor atual interpolado (0.0 a 1.0)
    #[serde(default = "default_one")]
    pub target_value: f32,       // Valor alvo (0.0 a 1.0) alterado por hits/dano
    #[serde(default = "default_trail_value")]
    pub trail_value: f32,        // Valor rastro de dano (efeito visual de rastro de vida estilo fighting game)
    #[serde(default)]
    pub easing_type: EasingType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetBlueprint {
    pub name: String,
    pub canvas_width: f32,
    pub canvas_height: f32,
    pub elements: Vec<WidgetElement>,
    #[serde(skip)]
    pub selected_element_idx: Option<usize>,
    #[serde(skip)]
    pub is_preview_mode: bool,
}

impl Default for WidgetBlueprint {
    fn default() -> Self {
        Self::default_main_menu()
    }
}

impl WidgetBlueprint {
    pub fn default_main_menu() -> Self {
        Self {
            name: "WBP_MainMenu".to_string(),
            canvas_width: 1920.0,
            canvas_height: 1080.0,
            selected_element_idx: Some(0),
            is_preview_mode: false,
            elements: vec![
                WidgetElement {
                    id: 1,
                    name: "TitleHeader".to_string(),
                    element_type: WidgetElementType::TextLabel,
                    pos_x: 760.0,
                    pos_y: 200.0,
                    width: 400.0,
                    height: 60.0,
                    text: "TOP DOWN EXAMPLE".to_string(),
                    font_size: 30.0,
                    text_color: [253, 199, 52, 255],
                    bg_color: [0, 0, 0, 0],
                    border_color: [0, 0, 0, 0],
                    onclick_action: "None".to_string(),
                    is_smoothed: true,
                    smoothing_duration: 0.35,
                    current_value: 1.0,
                    target_value: 1.0,
                    trail_value: 1.0,
                    easing_type: EasingType::EaseOutQuad,
                },
                WidgetElement {
                    id: 2,
                    name: "HealthBar_Player".to_string(),
                    element_type: WidgetElementType::ProgressBar,
                    pos_x: 760.0,
                    pos_y: 280.0,
                    width: 400.0,
                    height: 32.0,
                    text: "PLAYER HP".to_string(),
                    font_size: 13.0,
                    text_color: [255, 255, 255, 255],
                    bg_color: [40, 200, 100, 255],
                    border_color: [253, 199, 52, 255],
                    onclick_action: "None".to_string(),
                    is_smoothed: true,
                    smoothing_duration: 0.45,
                    current_value: 1.0,
                    target_value: 1.0,
                    trail_value: 1.0,
                    easing_type: EasingType::EaseOutQuad,
                },
                WidgetElement {
                    id: 3,
                    name: "Btn_PlayGame".to_string(),
                    element_type: WidgetElementType::Button,
                    pos_x: 810.0,
                    pos_y: 380.0,
                    width: 300.0,
                    height: 48.0,
                    text: "▶ PLAY SOLO / HOST".to_string(),
                    font_size: 15.0,
                    text_color: [20, 22, 28, 255],
                    bg_color: [253, 199, 52, 255],
                    border_color: [253, 199, 52, 255],
                    onclick_action: "OpenLevel_Map_Lobby".to_string(),
                    is_smoothed: true,
                    smoothing_duration: 0.35,
                    current_value: 1.0,
                    target_value: 1.0,
                    trail_value: 1.0,
                    easing_type: EasingType::EaseOutQuad,
                },
                WidgetElement {
                    id: 4,
                    name: "Btn_Multiplayer".to_string(),
                    element_type: WidgetElementType::Button,
                    pos_x: 810.0,
                    pos_y: 445.0,
                    width: 300.0,
                    height: 48.0,
                    text: "🌐 MULTIPLAYER LOBBY".to_string(),
                    font_size: 15.0,
                    text_color: [230, 235, 245, 255],
                    bg_color: [38, 42, 54, 255],
                    border_color: [60, 68, 85, 255],
                    onclick_action: "OpenLevel_Map_Lobby".to_string(),
                    is_smoothed: true,
                    smoothing_duration: 0.35,
                    current_value: 1.0,
                    target_value: 1.0,
                    trail_value: 1.0,
                    easing_type: EasingType::EaseOutQuad,
                },
                WidgetElement {
                    id: 5,
                    name: "Btn_Quit".to_string(),
                    element_type: WidgetElementType::Button,
                    pos_x: 810.0,
                    pos_y: 510.0,
                    width: 300.0,
                    height: 48.0,
                    text: "🚪 QUIT GAME".to_string(),
                    font_size: 15.0,
                    text_color: [230, 235, 245, 255],
                    bg_color: [38, 42, 54, 255],
                    border_color: [60, 68, 85, 255],
                    onclick_action: "QuitGame".to_string(),
                    is_smoothed: true,
                    smoothing_duration: 0.35,
                    current_value: 1.0,
                    target_value: 1.0,
                    trail_value: 1.0,
                    easing_type: EasingType::EaseOutQuad,
                },
            ],
        }
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        if let Some(parent) = std::path::Path::new(path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)
    }

    pub fn load_or_default(path: &str) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(wb) = serde_json::from_str::<Self>(&content) {
                return wb;
            }
        }
        Self::default_main_menu()
    }
}

// EDITOR COMPLETO WYSIWYG DE WIDGET BLUEPRINTS (COM EFEITOS E SUAVIZAÇÃO DE VALORES EM TEMPO REAL)
pub fn show_widget_editor(
    ui: &mut egui::Ui,
    widget: &mut WidgetBlueprint,
    world: &mut World,
    _i18n: &I18nManager,
) {
    let accent_color = ui.visuals().selection.bg_fill;
    let dt = ui.input(|i| i.stable_dt).clamp(0.001, 0.1);

    // PASSO DE ATUALIZAÇÃO E INTERPOLAÇÃO DAS ANIMAÇÕES DE SUAVIZAÇÃO (EASING) DE CADA ELEMENTO DA UI
    for elem in widget.elements.iter_mut() {
        if elem.is_smoothed {
            let dur = elem.smoothing_duration.max(0.01);
            let speed_factor = (dt * 8.0 / dur).clamp(0.001, 1.0);

            // Interpolação suave do valor principal (current_value -> target_value)
            elem.current_value += (elem.target_value - elem.current_value) * speed_factor;

            // Interpolação do rastro de dano (trail) para efeito visual fantástico estilo fighting games / Unreal Engine
            let trail_speed = (dt * 3.0 / dur).clamp(0.001, 1.0);
            if elem.trail_value > elem.current_value {
                elem.trail_value += (elem.current_value - elem.trail_value) * trail_speed;
            } else {
                elem.trail_value = elem.current_value;
            }

            if (elem.current_value - elem.target_value).abs() > 0.0001 || (elem.trail_value - elem.current_value).abs() > 0.0001 {
                ui.ctx().request_repaint(); // Requisita repintura contínua durante a transição suave
            }
        } else {
            elem.current_value = elem.target_value;
            elem.trail_value = elem.target_value;
        }
    }

    // BARRA SUPERIOR DO EDITOR DE WIDGET (SAVE | ADD ELEMENTOS | PREVIEW)
    Frame::none()
        .fill(Color32::from_rgb(22, 25, 34))
        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(45, 52, 68)))
        .inner_margin(Margin::symmetric(12.0, 6.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(RichText::new(format!("🖼️ Widget Blueprint Editor: {}", widget.name)).color(accent_color).strong());
                ui.separator();

                if ui.button("💾 Save Widget").clicked() {
                    let file_path = format!("projects/TopDownExample/Content/Actors/{}.uasset", widget.name);
                    let _ = widget.save_to_file(&file_path);
                }

                ui.separator();

                let prev_btn = if widget.is_preview_mode { "⏹ Exit Preview" } else { "▶ Test UI Preview" };
                if ui.selectable_label(widget.is_preview_mode, prev_btn).clicked() {
                    widget.is_preview_mode = !widget.is_preview_mode;
                }

                ui.separator();
                ui.label(RichText::new("Palette Add:").strong());

                if ui.button("➕ Button").clicked() {
                    let new_id = widget.elements.len() + 1;
                    widget.elements.push(WidgetElement {
                        id: new_id,
                        name: format!("Button_{}", new_id),
                        element_type: WidgetElementType::Button,
                        pos_x: 800.0,
                        pos_y: 400.0,
                        width: 240.0,
                        height: 44.0,
                        text: "New Button".to_string(),
                        font_size: 14.0,
                        text_color: [255, 255, 255, 255],
                        bg_color: [38, 42, 54, 255],
                        border_color: [70, 78, 95, 255],
                        onclick_action: "None".to_string(),
                        is_smoothed: true,
                        smoothing_duration: 0.35,
                        current_value: 1.0,
                        target_value: 1.0,
                        trail_value: 1.0,
                        easing_type: EasingType::EaseOutQuad,
                    });
                    widget.selected_element_idx = Some(widget.elements.len() - 1);
                }

                if ui.button("➕ Text").clicked() {
                    let new_id = widget.elements.len() + 1;
                    widget.elements.push(WidgetElement {
                        id: new_id,
                        name: format!("Text_{}", new_id),
                        element_type: WidgetElementType::TextLabel,
                        pos_x: 800.0,
                        pos_y: 350.0,
                        width: 300.0,
                        height: 40.0,
                        text: "New Text Label".to_string(),
                        font_size: 20.0,
                        text_color: [230, 235, 245, 255],
                        bg_color: [0, 0, 0, 0],
                        border_color: [0, 0, 0, 0],
                        onclick_action: "None".to_string(),
                        is_smoothed: true,
                        smoothing_duration: 0.35,
                        current_value: 1.0,
                        target_value: 1.0,
                        trail_value: 1.0,
                        easing_type: EasingType::EaseOutQuad,
                    });
                    widget.selected_element_idx = Some(widget.elements.len() - 1);
                }

                if ui.button("➕ Progress Bar (Health/Mana)").clicked() {
                    let new_id = widget.elements.len() + 1;
                    widget.elements.push(WidgetElement {
                        id: new_id,
                        name: format!("HealthBar_{}", new_id),
                        element_type: WidgetElementType::ProgressBar,
                        pos_x: 760.0,
                        pos_y: 300.0,
                        width: 350.0,
                        height: 30.0,
                        text: "PLAYER HP".to_string(),
                        font_size: 13.0,
                        text_color: [255, 255, 255, 255],
                        bg_color: [40, 200, 100, 255],
                        border_color: [253, 199, 52, 255],
                        onclick_action: "None".to_string(),
                        is_smoothed: true,
                        smoothing_duration: 0.45,
                        current_value: 1.0,
                        target_value: 1.0,
                        trail_value: 1.0,
                        easing_type: EasingType::EaseOutQuad,
                    });
                    widget.selected_element_idx = Some(widget.elements.len() - 1);
                }
            });
        });

    ui.add_space(4.0);

    // LAYOUT PRINCIPAL DE 3 COLUNAS DO EDITOR DE UI:
    ui.columns(3, |cols| {
        // COLUNA 1: HIERARQUIA
        cols[0].group(|ui| {
            ui.set_width(220.0);
            ui.heading(RichText::new("🌲 Component Hierarchy").strong());
            ui.separator();

            let mut to_remove: Option<usize> = None;

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (idx, elem) in widget.elements.iter().enumerate() {
                    let is_sel = widget.selected_element_idx == Some(idx);
                    let (icon, type_str) = match elem.element_type {
                        WidgetElementType::Button => ("🔘", "Button"),
                        WidgetElementType::TextLabel => ("📝", "TextLabel"),
                        WidgetElementType::Image => ("🖼️", "Image"),
                        WidgetElementType::ProgressBar => ("📊", "ProgressBar"),
                        WidgetElementType::Slider => ("🎚️", "Slider"),
                        WidgetElementType::PanelBox => ("📦", "PanelBox"),
                    };

                    ui.horizontal(|ui| {
                        if ui.selectable_label(is_sel, format!("{} {} ({})", icon, elem.name, type_str)).clicked() {
                            widget.selected_element_idx = Some(idx);
                        }
                        if ui.small_button("❌").clicked() {
                            to_remove = Some(idx);
                        }
                    });
                }
            });

            if let Some(rem_idx) = to_remove {
                widget.elements.remove(rem_idx);
                widget.selected_element_idx = None;
            }
        });

        // COLUNA 2: CANVAS INTERATIVO DE DESIGN (CENTRO)
        cols[1].vertical(|ui| {
            ui.heading(RichText::new("📐 Visual UI Canvas (1920x1080 Viewport)").strong());
            ui.separator();

            let canvas_area = ui.available_rect_before_wrap();
            let canvas_w = canvas_area.width().max(400.0);
            let canvas_h = (canvas_w * (1080.0 / 1920.0)).clamp(250.0, 600.0);
            let (rect_canvas, _resp_canvas) = ui.allocate_exact_size(Vec2::new(canvas_w, canvas_h), egui::Sense::click());

            let scale_x = canvas_w / 1920.0;
            let scale_y = canvas_h / 1080.0;

            // Fundo do Canvas do Menu
            ui.painter().rect_filled(rect_canvas, Rounding::same(6.0), Color32::from_rgb(16, 18, 24));
            ui.painter().rect_stroke(rect_canvas, Rounding::same(6.0), Stroke::new(1.5_f32, Color32::from_rgb(50, 55, 70)));

            // RENDERIZAÇÃO E INTERAÇÃO COM CADA ELEMENTO DA INTERFACE NO CANVAS
            let mut elem_clicked: Option<usize> = None;

            for (idx, elem) in widget.elements.iter_mut().enumerate() {
                let is_sel = widget.selected_element_idx == Some(idx);

                let render_x = rect_canvas.min.x + elem.pos_x * scale_x;
                let render_y = rect_canvas.min.y + elem.pos_y * scale_y;
                let render_w = elem.width * scale_x;
                let render_h = elem.height * scale_y;

                let elem_rect = Rect::from_min_size(Pos2::new(render_x, render_y), Vec2::new(render_w, render_h));

                let bg_c32 = Color32::from_rgba_unmultiplied(elem.bg_color[0], elem.bg_color[1], elem.bg_color[2], elem.bg_color[3]);
                let text_c32 = Color32::from_rgba_unmultiplied(elem.text_color[0], elem.text_color[1], elem.text_color[2], elem.text_color[3]);
                let border_c32 = if is_sel { accent_color } else { Color32::from_rgba_unmultiplied(elem.border_color[0], elem.border_color[1], elem.border_color[2], elem.border_color[3]) };

                let stroke_width = if is_sel { 2.0_f32 } else { 1.0_f32 };

                match elem.element_type {
                    WidgetElementType::ProgressBar => {
                        // 1. Fundo do trilho da barra de vida
                        ui.painter().rect_filled(elem_rect, Rounding::same(4.0), Color32::from_rgb(25, 28, 38));
                        ui.painter().rect_stroke(elem_rect, Rounding::same(4.0), Stroke::new(stroke_width, border_c32));

                        // 2. Rastro de dano (efeito fantasma amarelo/laranja ao tomar hit)
                        if elem.trail_value > elem.current_value {
                            let trail_w = render_w * elem.trail_value.clamp(0.0, 1.0);
                            let trail_rect = Rect::from_min_size(elem_rect.min, Vec2::new(trail_w, render_h));
                            ui.painter().rect_filled(trail_rect, Rounding::same(4.0), Color32::from_rgb(245, 158, 11));
                        }

                        // 3. Preenchimento principal suave da barra (ex: Verde para HP)
                        let fill_w = render_w * elem.current_value.clamp(0.0, 1.0);
                        if fill_w > 1.0 {
                            let fill_rect = Rect::from_min_size(elem_rect.min, Vec2::new(fill_w, render_h));
                            ui.painter().rect_filled(fill_rect, Rounding::same(4.0), bg_c32);
                        }

                        // 4. Porcentagem / Texto sobre a barra (ex: "PLAYER HP (75%)")
                        let font_scaled = (elem.font_size * scale_y).max(9.0);
                        let hp_pct_str = format!("{} ({:.0}%)", elem.text, elem.current_value * 100.0);
                        ui.painter().text(
                            elem_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &hp_pct_str,
                            egui::FontId::proportional(font_scaled),
                            text_c32,
                        );
                    }
                    _ => {
                        // Desenha fundo e borda padrão do elemento
                        ui.painter().rect_filled(elem_rect, Rounding::same(4.0), bg_c32);
                        ui.painter().rect_stroke(elem_rect, Rounding::same(4.0), Stroke::new(stroke_width, border_c32));

                        // Desenha o texto do elemento
                        let font_scaled = (elem.font_size * scale_y).max(9.0);
                        ui.painter().text(
                            elem_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            &elem.text,
                            egui::FontId::proportional(font_scaled),
                            text_c32,
                        );
                    }
                }

                // INTERAÇÃO E SELEÇÃO POR CLIQUE / ARRASTE NO CANVAS
                if !widget.is_preview_mode {
                    let elem_resp = ui.interact(elem_rect, ui.id().with(elem.id), egui::Sense::click_and_drag());
                    if elem_resp.clicked() {
                        elem_clicked = Some(idx);
                    }
                    if elem_resp.dragged() {
                        elem_clicked = Some(idx);
                        if let Some(pos) = elem_resp.interact_pointer_pos() {
                            let new_canvas_x = (pos.x - rect_canvas.min.x) / scale_x;
                            let new_canvas_y = (pos.y - rect_canvas.min.y) / scale_y;
                            elem.pos_x = (new_canvas_x - elem.width * 0.5).clamp(0.0, 1920.0 - elem.width);
                            elem.pos_y = (new_canvas_y - elem.height * 0.5).clamp(0.0, 1080.0 - elem.height);
                        }
                    }
                } else {
                    // MODO PREVIEW INTERATIVO (TESTA EVENTOS E HITS)
                    let elem_resp = ui.interact(elem_rect, ui.id().with(elem.id), egui::Sense::click());
                    if elem_resp.clicked() {
                        match elem.onclick_action.as_str() {
                            "OpenLevel_Map_Lobby" => {
                                world.is_playing = true;
                                log::info!("▶ WBP Menu Event: Launching Map_Lobby Game Simulation Mode!");
                            }
                            "QuitGame" => {
                                std::process::exit(0);
                            }
                            _ => {}
                        }
                    }
                }
            }

            if let Some(sel_idx) = elem_clicked {
                widget.selected_element_idx = Some(sel_idx);
            }
        });

        // COLUNA 3: INSPECTOR DE PROPRIEDADES DO COMPONENTE SELECIONADO (DIREITA)
        cols[2].group(|ui| {
            ui.set_width(280.0);
            ui.heading(RichText::new("⚙️ Component Details & Effects").strong());
            ui.separator();

            if let Some(sel_idx) = widget.selected_element_idx {
                if sel_idx < widget.elements.len() {
                    let elem = &mut widget.elements[sel_idx];

                    ui.label(RichText::new("Element Identity:").strong());
                    ui.horizontal(|ui| {
                        ui.label("Name:");
                        ui.add_sized(Vec2::new(180.0, 22.0), egui::TextEdit::singleline(&mut elem.name));
                    });

                    ui.add_space(6.0);
                    ui.separator();
                    ui.label(RichText::new("Canvas Transform:").strong());

                    egui::Grid::new("widget_transform_grid").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
                        ui.label("Pos X:");
                        ui.add(egui::DragValue::new(&mut elem.pos_x).speed(1.0).range(0.0..=1920.0));
                        ui.end_row();

                        ui.label("Pos Y:");
                        ui.add(egui::DragValue::new(&mut elem.pos_y).speed(1.0).range(0.0..=1080.0));
                        ui.end_row();

                        ui.label("Width:");
                        ui.add(egui::DragValue::new(&mut elem.width).speed(1.0).range(10.0..=1920.0));
                        ui.end_row();

                        ui.label("Height:");
                        ui.add(egui::DragValue::new(&mut elem.height).speed(1.0).range(10.0..=1080.0));
                        ui.end_row();
                    });

                    ui.add_space(6.0);
                    ui.separator();
                    ui.label(RichText::new("Text & Appearance:").strong());

                    ui.horizontal(|ui| {
                        ui.label("Text:");
                        ui.add_sized(Vec2::new(180.0, 22.0), egui::TextEdit::singleline(&mut elem.text));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Font Size:");
                        ui.add(egui::Slider::new(&mut elem.font_size, 8.0..=60.0).suffix("pt"));
                    });

                    ui.add_space(4.0);

                    // CORES DO ELEMENTO (TEXTO / FUNDO / BORDA)
                    let mut text_c32 = Color32::from_rgba_unmultiplied(elem.text_color[0], elem.text_color[1], elem.text_color[2], elem.text_color[3]);
                    let mut bg_c32 = Color32::from_rgba_unmultiplied(elem.bg_color[0], elem.bg_color[1], elem.bg_color[2], elem.bg_color[3]);
                    let mut border_c32 = Color32::from_rgba_unmultiplied(elem.border_color[0], elem.border_color[1], elem.border_color[2], elem.border_color[3]);

                    ui.horizontal(|ui| {
                        ui.label("Text Color:");
                        if ui.color_edit_button_srgba(&mut text_c32).changed() {
                            elem.text_color = [text_c32.r(), text_c32.g(), text_c32.b(), text_c32.a()];
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Fill Color:");
                        if ui.color_edit_button_srgba(&mut bg_c32).changed() {
                            elem.bg_color = [bg_c32.r(), bg_c32.g(), bg_c32.b(), bg_c32.a()];
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Border Color:");
                        if ui.color_edit_button_srgba(&mut border_c32).changed() {
                            elem.border_color = [border_c32.r(), border_c32.g(), border_c32.b(), border_c32.a()];
                        }
                    });

                    // SEÇÃO DEDICADA DE ANIMAÇÕES E SUAVIZAÇÃO DE VALORES (EASING)
                    ui.add_space(8.0);
                    ui.separator();
                    ui.label(RichText::new("✨ Smooth Value Effects & Animations:").color(accent_color).strong());

                    ui.checkbox(&mut elem.is_smoothed, "Enable Value Smoothing");

                    if elem.is_smoothed {
                        ui.horizontal(|ui| {
                            ui.label("⏱️ Smoothing Time:");
                            ui.add(egui::Slider::new(&mut elem.smoothing_duration, 0.05..=3.00).suffix("s"));
                        });

                        ui.horizontal(|ui| {
                            ui.label("📉 Easing Function:");
                            egui::ComboBox::from_id_salt("elem_easing_type")
                                .selected_text(match elem.easing_type {
                                    EasingType::Linear => "Linear",
                                    EasingType::EaseOutQuad => "EaseOutQuad (Smooth)",
                                    EasingType::EaseInOutCubic => "EaseInOutCubic",
                                    EasingType::SpringBounce => "SpringBounce",
                                })
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut elem.easing_type, EasingType::Linear, "Linear");
                                    ui.selectable_value(&mut elem.easing_type, EasingType::EaseOutQuad, "EaseOutQuad (Smooth)");
                                    ui.selectable_value(&mut elem.easing_type, EasingType::EaseInOutCubic, "EaseInOutCubic");
                                    ui.selectable_value(&mut elem.easing_type, EasingType::SpringBounce, "SpringBounce");
                                });
                        });
                    }

                    ui.add_space(4.0);
                    ui.label(RichText::new("💥 Simular Hit / Alteração de Valor:").strong());

                    ui.horizontal(|ui| {
                        if ui.button("💥 Take Hit (-25%)").clicked() {
                            elem.target_value = (elem.target_value - 0.25).max(0.0);
                        }
                        if ui.button("💚 Heal (+25%)").clicked() {
                            elem.target_value = (elem.target_value + 0.25).min(1.0);
                        }
                        if ui.button("🔄 Reset (100%)").clicked() {
                            elem.target_value = 1.0;
                            elem.current_value = 1.0;
                            elem.trail_value = 1.0;
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.label("Target Value %:");
                        if ui.add(egui::Slider::new(&mut elem.target_value, 0.0..=1.0)).changed() {
                            if !elem.is_smoothed {
                                elem.current_value = elem.target_value;
                                elem.trail_value = elem.target_value;
                            }
                        }
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.label(RichText::new("⚡ OnClicked Event Binding:").color(accent_color).strong());

                    ui.horizontal(|ui| {
                        ui.label("Action:");
                        egui::ComboBox::from_id_salt("elem_event_action")
                            .selected_text(&elem.onclick_action)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut elem.onclick_action, "None".to_string(), "None");
                                ui.selectable_value(&mut elem.onclick_action, "OpenLevel_Map_Lobby".to_string(), "🚀 OpenLevel (Map_Lobby)");
                                ui.selectable_value(&mut elem.onclick_action, "QuitGame".to_string(), "🚪 Quit Game");
                            });
                    });
                }
            } else {
                ui.label(RichText::new("Select a UI Component to edit details.").italics().color(Color32::GRAY));
            }
        });
    });
}
