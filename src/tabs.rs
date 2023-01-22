use eframe::egui::*;
use eframe::egui::style::{Margin,WidgetVisuals};
use std::rc::Rc;
use std::cell::RefCell;
use mech_core::*;

pub struct MyButtonTabs {
    labels: Vec<WidgetText>,
    color: Color32,
    hovered_color: Color32,
    clicked_color: Color32,
    wrap: Option<bool>,
    sense: Sense,
    min_size: Vec2,
    frame: Frame,
    hovered_frame: Frame,
    clicked_frame: Frame,
    active_frame: Frame,
    active_clicked: Frame,
    active_hovered: Frame,
    active_color: Color32,
    active_hovered_color: Color32,
    active_clicked_color: Color32,
    active_ix: Rc<RefCell<Value>>,
  }
  
  impl MyButtonTabs {
    pub fn new(active_ix: Rc<RefCell<Value>>, labels: Vec<String>) -> Self {
      Self {
        labels: labels.iter().map(|t| WidgetText::RichText(RichText::new(t))).collect(),
        wrap: None,
        sense: Sense::click(),
        min_size: Vec2{x: 120.0, y: 15.0},
        frame: Frame::default().fill(Color32::GRAY),
        color: Color32::DARK_GRAY,
        clicked_color: Color32::GRAY,
        clicked_frame: Frame::default().fill(Color32::DARK_GRAY),
        hovered_color: Color32::DARK_GRAY,
        hovered_frame: Frame::default().fill(Color32::LIGHT_GRAY),
        active_frame: Frame::default().fill(Color32::from_rgb(46,42,60)),
        active_color: Color32::from_rgb(227,221,235),
        active_clicked_color: Color32::LIGHT_BLUE,
        active_clicked: Frame::default().fill(Color32::DARK_BLUE),
        active_hovered_color: Color32::DARK_BLUE,
        active_hovered: Frame::default().fill(Color32::LIGHT_BLUE),
        active_ix,
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
  
  impl Widget for MyButtonTabs {
    fn ui(mut self, ui: &mut Ui) -> Response {
      let MyButtonTabs {
        labels,
        wrap,
        sense,
        min_size,
        frame,
        hovered_frame,
        color,
        hovered_color,
        clicked_color,
        clicked_frame,
        active_frame,
        active_clicked,
        active_hovered,
        active_color,
        active_hovered_color,
        active_clicked_color,
        active_ix,
      } = self;
  
      let mut desired_size: Vec2 = Vec2{x: 0.0, y: 0.0};
      let button_padding = Vec2::new(frame.inner_margin.left + frame.inner_margin.right, frame.inner_margin.top + frame.inner_margin.bottom);
      
      let mut label_galley = vec![];
      let mut desired_tab_sizes: Vec<Vec2> = vec![];
      for l in labels {
        let mut text_wrap_width = ui.available_width() - (frame.inner_margin.left + frame.inner_margin.right);
        let label = l.into_galley(ui, wrap, text_wrap_width, TextStyle::Button);
        let mut button_size = label.size();
        button_size += button_padding;
        button_size = button_size.at_least(min_size);

        desired_size = Vec2{x: desired_size.x + button_size.x, y: desired_size.y};
        label_galley.push(label);
        desired_tab_sizes.push(button_size);
      }

      let mut tab_rect = Rect::from_min_size(ui.clip_rect().min, Vec2{x: 0.0, y: 0.0});
      let (rect, mut response) = ui.allocate_at_least(desired_size, sense);

      let mut ix = 1;
      let mut active_ixx = active_ix.borrow_mut();
      for l in label_galley {
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, l.text()));
        let tab_size = desired_tab_sizes[ix - 1];
        tab_rect = Rect::from_min_size(Pos2{x: tab_rect.max.x, y: tab_rect.min.y}, tab_size);
        let hovered = if let Some(hovered_pos) = ui.input().pointer.hover_pos() { tab_rect.contains(hovered_pos) } else { false };
        let primary_down = ui.input().pointer.primary_down();
        let vix = Value::U8(U8::new(ix as u8));
        let (frame,text_color) = if hovered & primary_down {
          if *active_ixx == vix {
            (active_clicked, active_clicked_color)
          } else {
            *active_ixx = vix;
            response.mark_changed();
            (clicked_frame, clicked_color)
          }
        } else if hovered {
          if *active_ixx == vix {(active_hovered, active_hovered_color)} else {(hovered_frame, hovered_color)}
        } else {
          if *active_ixx == vix {(active_frame, active_color)} else {(frame,color)}
        };

        let mut text_pos = Pos2::new(0.0,0.0);
        if ui.is_rect_visible(rect) {
          let mut visuals: WidgetVisuals = ui.style().interact(&response).clone();
          visuals.fg_stroke.color = text_color;
          ui.painter_at(tab_rect).rect(
            tab_rect,
            frame.rounding,
            frame.fill,
            frame.stroke,
          );
          let pos: Pos2 = ui.layout()
              .align_size_within_rect(l.size(), tab_rect.shrink2(button_padding / 2.0))
              .min;
          text_pos += Vec2::new(pos.x,pos.y);
          l.paint_with_visuals(ui.painter(), text_pos, &visuals);
        }
        ix += 1;
      }
      response
    }
  }