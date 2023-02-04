#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
use eframe::egui::*;
use eframe::egui::style::{Margin,WidgetVisuals};

pub mod button;
pub mod tabs;
//pub mod code_box;

#[derive(Copy, Clone, Debug)]
pub struct Style{
  pub default: (Frame,Color32,FrameStroke),
  pub hovered: (Frame,Color32,FrameStroke),
  pub clicked: (Frame,Color32,FrameStroke),
}

impl Style{

  pub fn default() -> Self {
    Self {
      default: (Frame::default(), Color32::GRAY, FrameStroke::default()),
      hovered: (Frame::default(), Color32::LIGHT_GRAY, FrameStroke::default()),
      clicked: (Frame::default(), Color32::DARK_GRAY, FrameStroke::default()),
    }
  }

}

#[derive(Copy, Clone, Debug)]
pub struct FrameStroke{pub left: Stroke, pub right: Stroke, pub top: Stroke, pub bottom: Stroke, pub color: Color32}

impl FrameStroke{

  pub fn new(width: f32, color: Color32) -> Self {
    Self {
      left: Stroke::new(width, color),
      right: Stroke::new(width, color),
      top: Stroke::new(width, color),
      bottom: Stroke::new(width, color),
      color,
    }
  }

  pub fn default() -> Self {
    Self::new(0.0, Color32::TRANSPARENT)
  }

}

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), wasm_bindgen::JsValue> {
  // Make sure panics are logged using `console.error`.
  console_error_panic_hook::set_once();

  // Redirect tracing to console.log and friends:
  tracing_wasm::set_as_global_default();

  eframe::start_web(
    canvas_id,
    Box::new(|cc| Box::new(egui_demo_lib::WrapApp::new(cc))),
  )
}
