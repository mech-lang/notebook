#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
use eframe::egui::*;
use eframe::egui::style::{Margin};

pub struct MyButton {
  text: WidgetText,
  wrap: Option<bool>,
  sense: Sense,
  min_size: Vec2,
  frame: Option<Frame>,
}

impl MyButton {
  pub fn new(text: impl Into<WidgetText>) -> Self {
    Self {
      text: text.into(),
      wrap: None,
      sense: Sense::click(),
      min_size: Vec2::ZERO,
      frame: None,
    }
  }

  #[inline]
  pub fn wrap(mut self, wrap: bool) -> Self {
    self.wrap = Some(wrap);
    self
  }

  pub fn frame(mut self, frame: Frame) -> Self {
    self.frame = Some(frame);
    self
  }

  pub fn sense(mut self, sense: Sense) -> Self {
    self.sense = sense;
    self
  }

  pub fn min_size(mut self, min_size: Vec2) -> Self {
    self.min_size = min_size;
    self
  }

}

impl Widget for MyButton {
  fn ui(self, ui: &mut Ui) -> Response {
    let MyButton {
      text,
      wrap,
      sense,
      min_size,
      frame
    } = self;

    let (mut inner_margin, outer_margin, rounding, fill, stroke) = if let Some(frame) = frame {
      (frame.inner_margin,
      frame.outer_margin,
      frame.rounding,
      frame.fill,
      frame.stroke)
    } else {
      (Margin::same(0.0),
      Margin::same(0.0),
      Rounding::same(0.0),
      Color32::TRANSPARENT,
      Stroke::new(0.0, Color32::TRANSPARENT))
    };

    let mut text_wrap_width = ui.available_width() - (inner_margin.left + inner_margin.right);

    let text = text.into_galley(ui, wrap, text_wrap_width, TextStyle::Button);

    let mut desired_size = text.size();

    let button_padding = Vec2::new(inner_margin.left + inner_margin.right, inner_margin.top + inner_margin.bottom);
    desired_size += button_padding;
    desired_size = desired_size.at_least(min_size);

    let (rect, response) = ui.allocate_at_least(desired_size, sense);
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

    if ui.is_rect_visible(rect) {
      if response.hovered() {
        let visuals = ui.style().interact(&response);
        ui.painter().rect(
          rect.expand(visuals.expansion),
          rounding,
          Color32::RED,
          stroke,
        );
        let text_pos = ui.layout()
            .align_size_within_rect(text.size(), rect.shrink2(button_padding / 2.0))
            .min;
        text.paint_with_visuals(ui.painter(), text_pos, visuals);
  
      } else {
        let visuals = ui.style().interact(&response);
        ui.painter().rect(
          rect.expand(visuals.expansion),
          rounding,
          fill,
          stroke,
        );
        let text_pos = ui.layout()
            .align_size_within_rect(text.size(), rect.shrink2(button_padding / 2.0))
            .min;
        text.paint_with_visuals(ui.painter(), text_pos, visuals);
      }
    }
    response
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
