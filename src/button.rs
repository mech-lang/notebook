use eframe::egui::*;
use eframe::egui::style::{Margin,WidgetVisuals};

use crate::Style;

pub struct MyButton {
    pub text: WidgetText,
    pub wrap: Option<bool>,
    pub sense: Sense,
    pub min_size: Vec2,
    pub style: Style,
  }
  
  impl MyButton {
    pub fn new(text: impl Into<WidgetText>) -> Self {
      Self {
        text: text.into(),
        wrap: None,
        sense: Sense::click(),
        min_size: Vec2::ZERO,
        style: Style::default(),
      }
    }
    
  }
  
  impl Widget for MyButton {
    fn ui(self, ui: &mut Ui) -> Response {
      let MyButton {
        text,
        wrap,
        sense,
        min_size,
        style,
      } = self;

      let (frame,_,_) = style.default;
  
      let mut text_wrap_width = ui.available_width() - (frame.inner_margin.left + frame.inner_margin.right);
      let text = text.into_galley(ui, wrap, text_wrap_width, TextStyle::Button);
      let mut desired_size = text.size();
  
      let button_padding = Vec2::new(frame.inner_margin.left + frame.inner_margin.right, frame.inner_margin.top + frame.inner_margin.bottom);
      desired_size += button_padding;
      desired_size = desired_size.at_least(min_size);
  
      let (rect, response) = ui.allocate_at_least(desired_size, sense);
      response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));
  
      let (frame,text_color,frame_stroke) = if response.is_pointer_button_down_on() {
        style.clicked
      } else if response.hovered() {
        style.hovered
      } else {
        style.default
      };
  
      let mut frame = frame;

      if ui.is_rect_visible(rect) {
        let mut visuals: WidgetVisuals = ui.style().interact(&response).clone();
        visuals.fg_stroke.color = text_color;
        let frame_rect = Rect{min: Pos2{x: rect.min.x - frame_stroke.left.width, y: rect.min.y - frame_stroke.top.width}, max: Pos2{x: rect.max.x + frame_stroke.right.width, y: rect.max.y + frame_stroke.bottom.width}};
        ui.painter().rect(
          frame_rect.expand(0.0),
          frame.rounding,
          frame_stroke.color,
          frame.stroke,
        );
        ui.painter().rect(
          rect.expand(0.0),
          frame.rounding,
          frame.fill,         
          frame.stroke,
        );
        let text_pos = ui.layout()
            .align_size_within_rect(text.size(), rect.shrink2(button_padding / 2.0))
            .min;
        text.paint_with_visuals(ui.painter(), text_pos, &visuals);
      }
      response
    }
  }