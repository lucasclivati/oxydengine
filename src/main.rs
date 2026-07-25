#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use winit::event_loop::EventLoop;
use oxydengine::App;

fn main() {
    env_logger::init();
    log::info!("Iniciando Oxyd Engine v0.0.1...");

    let event_loop = EventLoop::new().expect("Falha ao criar o EventLoop do Winit 0.30.");
    let mut app = App::new();

    let _ = event_loop.run_app(&mut app);
}
