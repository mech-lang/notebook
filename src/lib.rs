#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
use eframe::egui::*;
use eframe::egui::style::{Margin};

pub struct MyButton {
  text: WidgetText,
  shortcut_text: WidgetText,
  wrap: Option<bool>,
  fill: Option<Color32>,
  stroke: Option<Stroke>,
  sense: Sense,
  small: bool,
  frame: Option<bool>,
  min_size: Vec2,
  inner_margin: Option<Margin>,
  outer_margin: Option<Margin>,
  rounding: Option<Rounding>,
}

impl MyButton {
  pub fn new(text: impl Into<WidgetText>) -> Self {
    Self {
      text: text.into(),
      shortcut_text: Default::default(),
      wrap: None,
      fill: None,
      stroke: None,
      sense: Sense::click(),
      small: false,
      frame: None,
      min_size: Vec2::ZERO,
      inner_margin: None,
      outer_margin: None,
      rounding: None
    }
  }

  /// If `true`, the text will wrap to stay within the max width of the [`Ui`].
  ///
  /// By default [`Self::wrap`] will be true in vertical layouts
  /// and horizontal layouts with wrapping,
  /// and false on non-wrapping horizontal layouts.
  ///
  /// Note that any `\n` in the text will always produce a new line.
  #[inline]
  pub fn wrap(mut self, wrap: bool) -> Self {
    self.wrap = Some(wrap);
    self
  }

  /// Override background fill color. Note that this will override any on-hover effects.
  /// Calling this will also turn on the frame.
  pub fn outer_margin(mut self, margin: impl Into<Margin>) -> Self {
    self.outer_margin = Some(margin.into());
    self
  }

  /// Override background fill color. Note that this will override any on-hover effects.
  /// Calling this will also turn on the frame.
  pub fn fill(mut self, fill: impl Into<Color32>) -> Self {
    self.fill = Some(fill.into());
    self.frame = Some(true);
    self
  }

  pub fn rounding(mut self, rounding: impl Into<Rounding>) -> Self {
    self.rounding = Some(rounding.into());
    self.frame = Some(true);
    self
  }

  /// Override button stroke. Note that this will override any on-hover effects.
  /// Calling this will also turn on the frame.
  pub fn stroke(mut self, stroke: impl Into<Stroke>) -> Self {
    self.stroke = Some(stroke.into());
    self.frame = Some(true);
    self
  }

  /// Make this a small button, suitable for embedding into text.
  pub fn small(mut self) -> Self {
    self.text = self.text.text_style(TextStyle::Body);
    self.small = true;
    self
  }

  /// Turn off the frame
  pub fn frame(mut self, frame: bool) -> Self {
    self.frame = Some(frame);
    self
  }

  /// By default, buttons senses clicks.
  /// Change this to a drag-button with `Sense::drag()`.
  pub fn sense(mut self, sense: Sense) -> Self {
    self.sense = sense;
    self
  }

  /// Set the minimum size of the button.
  pub fn min_size(mut self, min_size: Vec2) -> Self {
    self.min_size = min_size;
    self
  }

  /// Show some text on the right side of the button, in weak color.
  ///
  /// Designed for menu buttons, for setting a keyboard shortcut text (e.g. `Ctrl+S`).
  ///
  /// The text can be created with [`Context::format_shortcut`].
  pub fn shortcut_text(mut self, shortcut_text: impl Into<WidgetText>) -> Self {
    self.shortcut_text = shortcut_text.into();
    self
  }
}

impl Widget for MyButton {
  fn ui(self, ui: &mut Ui) -> Response {
    let MyButton {
      text,
      shortcut_text,
      wrap,
      fill,
      stroke,
      sense,
      small,
      frame,
      min_size,
      inner_margin,
      outer_margin,
      rounding
    } = self;

    let frame = frame.unwrap_or_else(|| ui.visuals().button_frame);

    let mut inner_margin = if let Some(inner_margin) = inner_margin {
      inner_margin
    } else {
      Margin::same(0.0)
    };

    let rounding = if let Some(rounding) = rounding {
        rounding
      } else {
        Rounding::same(0.0)
      };

    if small {
      inner_margin.top = 0.0;
      inner_margin.bottom = 0.0;
    }

    let mut text_wrap_width = ui.available_width() - (inner_margin.left + inner_margin.right);

    if !shortcut_text.is_empty() {
      text_wrap_width -= 60.0; // Some space for the shortcut text (which we never wrap).
    }

    let text = text.into_galley(ui, wrap, text_wrap_width, TextStyle::Button);
    let shortcut_text = (!shortcut_text.is_empty())
      .then(|| shortcut_text.into_galley(ui, Some(false), f32::INFINITY, TextStyle::Button));

    let mut desired_size = text.size();
    if let Some(shortcut_text) = &shortcut_text {
      desired_size.x += ui.spacing().item_spacing.x + shortcut_text.size().x;
      desired_size.y = desired_size.y.max(shortcut_text.size().y);
    }
    if !small {
      desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
    }
    let button_padding = Vec2::new(inner_margin.left + inner_margin.right, inner_margin.top + inner_margin.bottom);
    desired_size += button_padding;
    desired_size = desired_size.at_least(min_size);

    let (rect, response) = ui.allocate_at_least(desired_size, sense);
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

    if ui.is_rect_visible(rect) {
      let visuals = ui.style().interact(&response);

      if frame {
        let fill = fill.unwrap_or(visuals.bg_fill);
        let stroke = stroke.unwrap_or(visuals.bg_stroke);
        ui.painter().rect(
          rect.expand(visuals.expansion),
          rounding,
          fill,
          stroke,
        );
      }

      let text_pos = ui.layout()
          .align_size_within_rect(text.size(), rect.shrink2(button_padding / 2.0))
          .min;
      text.paint_with_visuals(ui.painter(), text_pos, visuals);

      if let Some(shortcut_text) = shortcut_text {
        let shortcut_text_pos = pos2(
          rect.max.x - button_padding.x / 2.0 - shortcut_text.size().x,
          rect.center().y - 0.5 * shortcut_text.size().y,
        );
        shortcut_text.paint_with_fallback_color(
          ui.painter(),
          shortcut_text_pos,
          ui.visuals().weak_text_color(),
        );
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
