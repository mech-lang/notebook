use eframe::egui::*;
use eframe::egui::style::{Margin,WidgetVisuals};


pub struct MyButton {
    text: WidgetText,
    color: Color32,
    hovered_color: Color32,
    clicked_color: Color32,
    wrap: Option<bool>,
    sense: Sense,
    min_size: Vec2,
    frame: Frame,
    hovered_frame: Frame,
    clicked_frame: Frame,
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
        clicked_color: Color32::GRAY,
        clicked_frame: Frame::default().fill(Color32::DARK_GRAY),
        hovered_color: Color32::DARK_GRAY,
        hovered_frame: Frame::default().fill(Color32::LIGHT_GRAY),
      }
    }
  
    #[inline]
    pub fn wrap(mut self, wrap: bool) -> Self {
      self.wrap = Some(wrap);
      self
    }
  
    pub fn frame(mut self, frame: Frame) -> Self {
      self.frame = frame;
      self
    }
  
    pub fn hovered_frame(mut self, frame: Frame) -> Self {
      self.frame = frame;
      self
    }
  
    pub fn hovered_color(mut self, color: Color32) -> Self {
      self.hovered_color = color;
      self
    }
  
    pub fn color(mut self, color: Color32) -> Self {
      self.color = color;
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
        frame,
        hovered_frame,
        color,
        hovered_color,
        clicked_color,
        clicked_frame,
      } = self;
  
      let mut text_wrap_width = ui.available_width() - (frame.inner_margin.left + frame.inner_margin.right);
      let text = text.into_galley(ui, wrap, text_wrap_width, TextStyle::Button);
      let mut desired_size = text.size();
  
      let button_padding = Vec2::new(frame.inner_margin.left + frame.inner_margin.right, frame.inner_margin.top + frame.inner_margin.bottom);
      desired_size += button_padding;
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
  
      if ui.is_rect_visible(rect) {
        let mut visuals: WidgetVisuals = ui.style().interact(&response).clone();
        visuals.fg_stroke.color = text_color;
        ui.painter().rect(
          rect.expand(visuals.expansion),
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