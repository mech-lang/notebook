use eframe::egui::*;
use eframe::egui::style::{Margin,WidgetVisuals};

use crate::FrameStroke;

pub struct MyButton {
    pub text: WidgetText,
    pub color: Color32,
    pub hovered_color: Color32,
    pub clicked_color: Color32,
    pub wrap: Option<bool>,
    pub sense: Sense,
    pub min_size: Vec2,
    pub frame: Frame,
    pub hovered_frame: Frame,
    pub clicked_frame: Frame,
    pub frame_stroke: FrameStroke,
  }
  
  impl MyButton {
    pub fn new(text: impl Into<WidgetText>) -> Self {
      Self {
        text: text.into(),
        wrap: None,
        sense: Sense::click(),
        min_size: Vec2::ZERO,
        frame: Frame::default().fill(Color32::GRAY),
        color: Color32::DARK_GRAY,
        frame_stroke: FrameStroke::new(1.0,Color32::TRANSPARENT),
        clicked_color: Color32::GRAY,
        clicked_frame: Frame::default().fill(Color32::DARK_GRAY),
        hovered_color: Color32::DARK_GRAY,
        hovered_frame: Frame::default().fill(Color32::LIGHT_GRAY),
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
        frame,
        hovered_frame,
        color,
        hovered_color,
        clicked_color,
        clicked_frame,
        frame_stroke,
      } = self;
  
      let mut text_wrap_width = ui.available_width() - (frame.inner_margin.left + frame.inner_margin.right);
      let text = text.into_galley(ui, wrap, text_wrap_width, TextStyle::Button);
      let mut desired_size = text.size();
  
      let button_padding = Vec2::new(frame.inner_margin.left + frame.inner_margin.right, frame.inner_margin.top + frame.inner_margin.bottom);
      desired_size += button_padding;
      let button_stroke = Vec2::new(frame_stroke.left.width + frame_stroke.right.width, frame_stroke.top.width + frame_stroke.bottom.width);
      //desired_size += button_stroke;
      desired_size = desired_size.at_least(min_size);
  
      let (rect, response) = ui.allocate_at_least(desired_size, sense);
      response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));
  
      let (frame,text_color) = if response.is_pointer_button_down_on() {
        (clicked_frame, clicked_color)
      } else if response.hovered() {
        (hovered_frame, hovered_color)
      } else {
        (frame,color)
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