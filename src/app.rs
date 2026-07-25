use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::renderer::Renderer;
use crate::scene::World;
use crate::project::ProjectConfig;
use crate::editor::{
    show_top_bars, show_outliner_panel, show_details_panel, show_content_browser_and_log,
    show_launcher_gui, show_main_menu_widget, MainMenuAction, LauncherState, I18nManager, LayoutSettings, BottomTab, ConsoleState,
};

#[derive(PartialEq)]
pub enum AppMode {
    Launcher,
    Editor,
}

pub struct App {
    renderer: Option<Renderer>,
    egui_winit_state: Option<egui_winit::State>,
    mode: AppMode,
    launcher_state: LauncherState,
    layout_settings: LayoutSettings,
    console_state: ConsoleState,
    active_project: Option<ProjectConfig>,
    world: World,
    i18n: I18nManager,
    last_frame: Instant,
}

fn load_window_icon() -> Option<winit::window::Icon> {
    if let Ok(img) = image::open("logo.jpg") {
        let rgba = img.to_rgba8();
        let (w, h) = (img.width(), img.height());
        winit::window::Icon::from_rgba(rgba.into_raw(), w, h).ok()
    } else {
        None
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            renderer: None,
            egui_winit_state: None,
            mode: AppMode::Launcher,
            launcher_state: LauncherState::new(),
            layout_settings: LayoutSettings::load(),
            console_state: ConsoleState::new(),
            active_project: None,
            world: World::new_main_menu_scene(),
            i18n: I18nManager::new(),
            last_frame: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_none() {
            log::info!("Inicializando Oxyd Engine v0.0.1...");
            
            // JANELA MAXIMIZADA POR PADRÃO NO MONITOR ATUAL
            let mut window_attributes = Window::default_attributes()
                .with_title("Oxyd Engine Hub")
                .with_inner_size(winit::dpi::PhysicalSize::new(1280, 800))
                .with_maximized(true)
                .with_visible(true);

            if let Some(icon) = load_window_icon() {
                window_attributes = window_attributes.with_window_icon(Some(icon));
            }

            let window = Arc::new(event_loop.create_window(window_attributes).expect("Falha ao criar janela."));
            window.set_visible(true);

            let renderer = pollster::block_on(Renderer::new(window.clone()));
            
            let egui_winit_state = egui_winit::State::new(
                renderer.egui_ctx.clone(),
                egui::ViewportId::ROOT,
                window.as_ref(),
                Some(window.scale_factor() as f32),
                None,
                None,
            );

            renderer.window.request_redraw();
            
            self.renderer = Some(renderer);
            self.egui_winit_state = Some(egui_winit_state);
            self.last_frame = Instant::now();
            log::info!("Oxyd Engine inicializado com sucesso!");
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let renderer = match self.renderer.as_mut() {
            Some(r) => r,
            None => return,
        };

        if let Some(egui_state) = self.egui_winit_state.as_mut() {
            let _ = egui_state.on_window_event(renderer.window.as_ref(), &event);
        }

        match event {
            WindowEvent::CloseRequested => {
                log::info!("Encerrando Oxyd Engine...");
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                renderer.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = now.duration_since(self.last_frame).as_secs_f32();
                self.last_frame = now;

                // NAVEGAÇÃO DE VÔO UNREAL ENGINE NO VIEWPORT (Botão Direito + WASD / QE)
                if self.mode == AppMode::Editor {
                    renderer.camera_controller.process_input(&renderer.egui_ctx, dt);
                    
                    if renderer.egui_ctx.input(|i| i.key_pressed(egui::Key::F)) {
                        if let Some(actor) = self.world.get_selected_actor_mut() {
                            renderer.camera_controller.focus_target(actor.transform.position);
                        }
                    }
                }

                let egui_state = self.egui_winit_state.as_mut().unwrap();
                let raw_input = egui_state.take_egui_input(renderer.window.as_ref());

                let mode = &self.mode;
                let launcher_state = &mut self.launcher_state;
                let layout_settings = &mut self.layout_settings;
                let console_state = &mut self.console_state;
                let active_project = &self.active_project;
                let world = &mut self.world;
                let i18n = &mut self.i18n;

                let mut project_to_open: Option<ProjectConfig> = None;
                let mut switch_to_launcher = false;

                let proj_name = active_project.as_ref().map(|p| p.name.as_str()).unwrap_or("AlchemySurvival57old");

                let full_output = renderer.egui_ctx.run(raw_input, |ctx| {
                    if ctx.input(|i| i.key_pressed(egui::Key::Space) && i.modifiers.ctrl) {
                        layout_settings.active_bottom_tab = if layout_settings.active_bottom_tab == BottomTab::ContentDrawer { BottomTab::None } else { BottomTab::ContentDrawer };
                        layout_settings.save();
                    }

                    if *mode == AppMode::Launcher {
                        project_to_open = show_launcher_gui(ctx, launcher_state, i18n);
                    } else {
                        if let Some(true) = show_top_bars(ctx, world, i18n, layout_settings, proj_name) {
                            switch_to_launcher = true;
                        }
                        
                        // DUPLO CLIQUE NO OUTLINER FOCA A CÂMERA AUTOMATICAMENTE
                        if let Some(target_pos) = show_outliner_panel(ctx, world, i18n, layout_settings) {
                            renderer.camera_controller.focus_target(target_pos);
                        }

                        show_details_panel(ctx, world, i18n, layout_settings);
                        show_content_browser_and_log(ctx, i18n, layout_settings, world, console_state);

                        // O JOGO FICA RESTRITO À ÁREA CENTRAL DO VIEWPORT DO EDITOR (CentralPanel)
                        egui::CentralPanel::default()
                            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
                            .show(ctx, |ui| {
                                if !world.is_playing {
                                    match show_main_menu_widget(ui, world) {
                                        MainMenuAction::HostSoloGame => {
                                            log::info!("Iniciando partida solo no AlchemySurvival57old...");
                                            world.is_playing = true;
                                        }
                                        MainMenuAction::JoinLobby => {
                                            log::info!("Carregando Map_Lobby...");
                                            *world = World::new_third_person_level();
                                        }
                                        MainMenuAction::QuitGame => {
                                            std::process::exit(0);
                                        }
                                        _ => {}
                                    }
                                }
                            });
                    }
                });

                egui_state.handle_platform_output(renderer.window.as_ref(), full_output.platform_output);
                let clipped_primitives = renderer.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

                if switch_to_launcher {
                    self.mode = AppMode::Launcher;
                    renderer.window.set_title("Oxyd Engine Hub");
                }

                if let Some(proj) = project_to_open {
                    log::info!("Abrindo projeto: {} ({})", proj.name, proj.path);
                    renderer.window.set_title(&format!("Oxyd Engine - {}", proj.name));
                    self.active_project = Some(proj);
                    
                    self.world = World::new_main_menu_scene();
                    self.mode = AppMode::Editor;
                }

                renderer.update(dt, &mut self.world);

                match renderer.render(&mut self.world, &clipped_primitives, &full_output.textures_delta) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = renderer.size;
                        renderer.resize(size);
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        log::error!("Memória da GPU esgotada!");
                        event_loop.exit();
                    }
                    Err(e) => {
                        log::error!("Erro de renderização Surface: {:?}", e);
                    }
                }

                renderer.window.request_redraw();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.as_ref() {
            renderer.window.request_redraw();
        }
    }
}
