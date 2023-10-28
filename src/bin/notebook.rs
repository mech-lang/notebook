#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use mech_gui::app::*;
use mech_notebook::*;
use egui::Vec2;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let mut native_options = eframe::NativeOptions::default();
    let icon = icon::load_icon();
    let core = load_mech().unwrap();
    native_options.icon_data = Some(icon);
    native_options.min_window_size = Some(Vec2{x: 500.0, y: 400.0});
    native_options.initial_window_size = Some(Vec2{x: 1280.0, y: 720.0});
    //eframe::run_native("Mech Notebook", native_options, Box::new(|cc| 
    //  Box::new(MechApp::new(cc,core))));
    eframe::run_native(
        "Mech Notebook",
        native_options,
        Box::new(|cc| Box::new(MechApp::new(cc,core))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
