#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]
#![recursion_limit="256"]

use eframe::{egui};
use eframe::egui::{containers::*, *};
use egui::style::Margin;
use egui_extras::{StripBuilder, Size};

extern crate mech_utilities;
extern crate mech_syntax;
extern crate mech_core;

use mech_utilities::*;
use crate::epaint::Shadow;
use mech_core::*;
use mech_syntax::compiler::Compiler;
use std::thread::JoinHandle;
extern crate image;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, Pixel, Rgba};

#[macro_use]
extern crate lazy_static;

lazy_static! {
  static ref LINK: u64 = hash_str("link");
  static ref IMG: u64 = hash_str("img");
  static ref SRC: u64 = hash_str("src");
  static ref CONTAINS: u64 = hash_str("contains");
  static ref KIND: u64 = hash_str("kind");
  static ref BUTTON: u64 = hash_str("button");
  static ref SLIDER: u64 = hash_str("slider");
  static ref MIN: u64 = hash_str("min");
  static ref MAX__HEIGHT: u64 = hash_str("max-width");
  static ref MAX__WIDTH: u64 = hash_str("max-width");
  static ref MIN__WIDTH: u64 = hash_str("min-width");
  static ref MIN__HEIGHT: u64 = hash_str("min-height");
  static ref MAX: u64 = hash_str("max");
  static ref VALUE: u64 = hash_str("value");
  static ref CANVAS: u64 = hash_str("canvas");
  static ref PARAMETERS: u64 = hash_str("parameters");
  static ref HEIGHT: u64 = hash_str("height");
  static ref WIDTH: u64 = hash_str("width");
  static ref SHAPE: u64 = hash_str("shape");
  static ref CIRCLE: u64 = hash_str("circle");
  static ref RECTANGLE: u64 = hash_str("rectangle");
  static ref LINE: u64 = hash_str("line");
  static ref PATH: u64 = hash_str("path");
  static ref START__POINT: u64 = hash_str("start-point");
  static ref LINE__WIDTH: u64 = hash_str("line-width");
  static ref START__ANGLE: u64 = hash_str("start-angle");
  static ref END__ANGLE: u64 = hash_str("end-angle");
  static ref QUADRATIC: u64 = hash_str("quadratic");
  static ref CONTROL__POINT: u64 = hash_str("control-point");
  static ref CONTROL__POINTS: u64 = hash_str("control-points");
  static ref END__POINT: u64 = hash_str("end-point");
  static ref X1: u64 = hash_str("x1");
  static ref X2: u64 = hash_str("x2");
  static ref Y1: u64 = hash_str("y1");
  static ref Y2: u64 = hash_str("y2");
  static ref RADIUS: u64 = hash_str("radius");
  static ref STROKE: u64 = hash_str("stroke");
  static ref FILL: u64 = hash_str("fill");
  static ref CENTER__X: u64 = hash_str("center-x");
  static ref CENTER__Y: u64 = hash_str("center-y");
  static ref IMAGE: u64 = hash_str("image");
  static ref X: u64 = hash_str("x");
  static ref Y: u64 = hash_str("y");
  static ref ROTATE: u64 = hash_str("rotate");
  static ref TRANSLATE: u64 = hash_str("translate");
  static ref SOURCE: u64 = hash_str("source");
  static ref TIME_TIMER: u64 = hash_str("time/timer");
  static ref PERIOD: u64 = hash_str("period");
  static ref TICKS: u64 = hash_str("ticks");
  static ref TARGET: u64 = hash_str("target");
  static ref KEY: u64 = hash_str("key");
  static ref EVENT__ID: u64 = hash_str("event-id");
  static ref ARC: u64 = hash_str("arc");
  static ref ELLIPSE: u64 = hash_str("ellipse");
  static ref MAJOR__AXIS: u64 = hash_str("major-axis");
  static ref MINOR__AXIS: u64 = hash_str("minor-axis");
  static ref STARTING__ANGLE: u64 = hash_str("starting-angle");
  static ref ENDING__ANGLE: u64 = hash_str("ending-angle");
  static ref FONT: u64 = hash_str("font");
  static ref SIZE: u64 = hash_str("size");
  static ref FACE: u64 = hash_str("face");
  static ref STYLE: u64 = hash_str("style");
  static ref WEIGHT: u64 = hash_str("weight");
  static ref BOLD: u64 = hash_str("bold");
  static ref NORMAL: u64 = hash_str("normal");
  static ref ITALIC: u64 = hash_str("italic");
  static ref FAMILY: u64 = hash_str("family");
  static ref DIRECTION: u64 = hash_str("direction");
  static ref ALIGNMENT: u64 = hash_str("alignment");
  static ref START: u64 = hash_str("start");
  static ref END: u64 = hash_str("end");
  static ref LEFT: u64 = hash_str("left");
  static ref RIGHT: u64 = hash_str("right");
  static ref CENTER: u64 = hash_str("center");
  static ref BEZIER: u64 = hash_str("bezier");
  static ref TEXT: u64 = hash_str("text");
  static ref URL: u64 = hash_str("url");
  static ref CODE: u64 = hash_str("code");
  static ref PANEL__TOP: u64 = hash_str("panel-top");
  static ref PANEL__BOTTOM: u64 = hash_str("panel-bottom");
  static ref PANEL__LEFT: u64 = hash_str("panel-left");
  static ref PANEL__CENTER: u64 = hash_str("panel-center");
  static ref PANEL__RIGHT: u64 = hash_str("panel-right");
  static ref DEBUG: u64 = hash_str("debug");
  static ref CLICKED: u64 = hash_str("clicked");
  static ref TABLE__WINDOW: u64 = hash_str("table-window");
  static ref LABEL: u64 = hash_str("label");
  static ref COLOR: u64 = hash_str("color");
  static ref MARGIN: u64 = hash_str("margin");
  static ref PADDING: u64 = hash_str("padding");
  static ref FRAME: u64 = hash_str("frame");
  static ref ROUNDING: u64 = hash_str("rounding");
  static ref SHADOW: u64 = hash_str("shadow");
  static ref DISTANCE: u64 = hash_str("distance");
  static ref TOP: u64 = hash_str("top");
  static ref BOTTOM: u64 = hash_str("bottom");
  static ref STRIP: u64 = hash_str("strip");
  static ref HORIZONTAL: u64 = hash_str("horizontal");
  static ref VERTICAL: u64 = hash_str("vertical");
}

pub struct MechApp {
  //mech_client: RunLoop,
  ticks: f32,
  frame: usize,
  code: String,
  core: mech_core::Core,
  maestro_thread: Option<JoinHandle<()>>,
  shapes: Vec<epaint::Shape>,
  value_store: HashMap<usize,Value>,
  changes: Vec<Change>,
  windows: HashSet<String>,
}

//static LONG_STRING: &'static str = include_str!(concat!(env!("OUT_DIR"), "/hello.rs"));

impl MechApp {
  pub fn new(cc: &eframe::CreationContext<'_>, core: mech_core::Core) -> Self {
    //let code = LONG_STRING;
    //let code = include_str!("notebook.mec");

    let mut shapes = vec![epaint::Shape::Noop; 100000];

    Self {
      frame: 0,
      ticks: 0.0,
      code: "".to_string(),
      //mech_client,
      core,
      maestro_thread: None,
      shapes,
      windows: HashSet::new(),
      value_store: HashMap::new(),
      changes: vec![],
    }
  }
  
  pub fn render_app(&mut self, ui: &mut egui::Ui) -> Result<(), MechError> {
    match self.core.get_table("app") {
      Ok(app_table) => { 
        let app_table_brrw = app_table.borrow();
        ui.columns(app_table_brrw.cols, |cols| {
          for (col, col_ui) in cols.iter_mut().enumerate() {
            for row in 1..=app_table_brrw.rows as usize {
              match app_table_brrw.get(&TableIndex::Index(row), &TableIndex::Index(col+1)) {
                Ok(contents) => {
                  self.render_value(contents, col_ui);
                }
                x => {return Err(MechError{msg: "".to_string(), id: 6486, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
              }
            }
          }
          Ok(())
        });
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6487, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  fn render_value(&mut self, value: Value, ui: &mut egui::Ui) -> Result<(), MechError> {
    match value {
      Value::String(chars) => {
        let contents_string = chars.to_string();
        ui.label(&contents_string);
      },
      Value::Bool(x) => {ui.label(&format!("{}", x));},
      Value::F32(x) => {ui.label(&format!("{:.2?}", x));},
      Value::F64(x) => {ui.label(&format!("{:?}", x));},
      Value::U128(x) => {ui.label(&format!("{:?}", x));},
      Value::U64(x) => {ui.label(&format!("{:?}", x));},
      Value::U32(x) => {ui.label(&format!("{:?}", x));},
      Value::U16(x) => {ui.label(&format!("{:?}", x));},
      Value::U8(x) => {ui.label(&format!("{:?}", x));},
      Value::I128(x) => {ui.label(&format!("{:?}", x));},
      Value::I64(x) => {ui.label(&format!("{:?}", x));},
      Value::I32(x) => {ui.label(&format!("{:?}", x));},
      Value::I16(x) => {ui.label(&format!("{:?}", x));},
      Value::I8(x) => {ui.label(&format!("{:?}", x));},
      Value::Reference(TableId::Global(table_id)) => {
        let table = self.core.get_table_by_id(table_id).unwrap();
        self.make_element(&table.borrow(), ui);  
        //div.append_child(&rendered_ref)?;
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6488, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  fn make_element(&mut self, table: &Table, ui: &mut egui::Ui) -> Result<(), MechError> {
    match table.col_map.get_index(&*KIND) {
      Ok(_) => {
        for row in 1..=table.rows {
          match table.get(&TableIndex::Index(row), &TableIndex::Alias(*KIND))  {
            Ok(Value::String(kind)) => {
              let raw_kind = kind.hash();
              // Render an element
              if raw_kind == *LINK { self.render_link(table,row,ui)?; }
              else if raw_kind == *SLIDER { self.render_slider(table,row,ui)?; }
              else if raw_kind == *CODE { self.render_code(table,row,ui)?; }
              else if raw_kind == *PANEL__TOP { self.render_panel_top(table,row,ui)?; }
              else if raw_kind == *PANEL__BOTTOM { self.render_panel_bottom(table,row,ui)?; }
              else if raw_kind == *PANEL__RIGHT { self.render_panel_right(table,row,ui)?; }
              else if raw_kind == *PANEL__LEFT { self.render_panel_left(table,row,ui)?; }
              else if raw_kind == *PANEL__CENTER { self.render_panel_center(table,row,ui)?; }
              else if raw_kind == *BUTTON { self.render_button(table,row,ui)?; }
              else if raw_kind == *TABLE__WINDOW { self.render_table__window(table,row,ui)?; }
              else if raw_kind == *CANVAS { self.render_canvas(table,row,ui)?; }
              else if raw_kind == *DEBUG { self.render_debug(table,row,ui)?; }
              else if raw_kind == *LABEL { self.render_label(table,ui)?; }
              else if raw_kind == *FRAME { self.render_frame(table,row,ui)?; }
              else if raw_kind == *STRIP { self.render_strip(table,row,ui)?; }
              //else if raw_kind == *IMAGE { render_iamge(table,ui)?; }
              else {
                return Err(MechError{msg: "".to_string(), id: 6489, kind: MechErrorKind::GenericError(format!("{:?}", raw_kind))});
              }
            }
            x => {return Err(MechError{msg: "".to_string(), id: 6488, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
            Err(x) => {return Err(MechError{msg: "".to_string(), id: 6488, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
          }
        }
      }
      // There's no Type column, so we are going to treat the table as a generic thing and just turn it into divs
      Err(_) => {
        ui.columns(table.cols, |cols| {
          for (col, col_ui) in cols.iter_mut().enumerate() {
            for row in 1..=table.rows as usize {
              match table.get(&TableIndex::Index(row), &TableIndex::Index(col+1)) {
                Ok(contents) => {
                  self.render_value(contents, col_ui);
                }
                x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
              }
            }
          }
          Ok(())
        });
      }
    }
    Ok(())
  }

  pub fn render_code(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*TEXT))) {
      Ok(Value::Reference(code_table_id)) => {
        let code_table = self.core.get_table_by_id(*code_table_id.unwrap()).unwrap();
        let code_table_brrw = code_table.borrow();
        match code_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
          Ok(Value::String(code)) => {
            self.code = code.to_string();
            let response = container.add_sized(container.available_size(), egui::TextEdit::multiline(&mut self.code)
              .code_editor()
              .frame(false)
            );
            if response.changed() {
              self.changes.push(Change::Set((code_table_brrw.id,vec![(TableIndex::Index(1),TableIndex::Index(1),Value::String(MechString::from_string(self.code.clone())))])));
            }
          }
          x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
        }
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  pub fn render_panel_bottom(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*CONTAINS)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS))) {
      (contained,parameters_table) => {
        let mut frame = Frame::default();
        let mut min_height = 100.0;
        let mut max_height = 100.0;
        if let Ok(Value::Reference(parameters_table_id)) = parameters_table {
          match self.core.get_table_by_id(*parameters_table_id.unwrap()) {
            Ok(parameters_table) => {
              let parameters_table_brrw = parameters_table.borrow();
              if let Ok(Value::U128(color)) = parameters_table_brrw.get(&TableIndex::Index(1),&TableIndex::Alias(*FILL)) { 
                frame.fill = get_color(color);
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MIN__HEIGHT)) {
                min_height = value.into();
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MAX__HEIGHT)) {
                max_height = value.into();
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                frame.outer_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                frame.inner_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
                      frame.outer_margin = Margin{left: value.unwrap(), right: value.unwrap(), top: value.unwrap(), bottom: value.unwrap()};
                    }
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
                      frame.inner_margin = Margin{left: value.unwrap(), right: value.unwrap(), top: value.unwrap(), bottom: value.unwrap()};
                    }
                  }
                  _ => (),
                }
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*HEIGHT)) {
                max_height = value.into();
                min_height = value.into();
              }
            }
            _ => (),
          }
        }
        egui::TopBottomPanel::bottom(humanize(&table.id))
          .resizable(false)
          .min_height(min_height)
          .max_height(max_height)
          .frame(frame)
        .show_inside(container, |ui| {
          if let Ok(contained) = contained {
            self.render_value(contained, ui);
          }
        });
      }
    }
    Ok(())
  }

  pub fn render_panel_top(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*CONTAINS)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS))) {
      (contained,parameters_table) => {
        let mut frame = Frame::none();
        frame.stroke = Stroke::new(0.0, Color32::BLACK);
        let mut panel = egui::TopBottomPanel::top(humanize(&table.id)).resizable(false).show_separator_line(false);
        if let Ok(Value::Reference(parameters_table_id)) = parameters_table {
          match self.core.get_table_by_id(*parameters_table_id.unwrap()) {
            Ok(parameters_table) => {
              let parameters_table_brrw = parameters_table.borrow();
              if let Ok(Value::U128(color)) = parameters_table_brrw.get(&TableIndex::Index(1),&TableIndex::Alias(*FILL)) { 
                frame.fill = get_color(color);
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MIN__HEIGHT)) {
                panel = panel.min_height(value.into());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MAX__HEIGHT)) {
                panel = panel.min_height(value.into());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*HEIGHT)) {
                panel = panel.exact_height(value.into());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                frame.outer_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                frame.inner_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
                      frame.outer_margin = Margin{left: value.unwrap(), right: value.unwrap(), top: value.unwrap(), bottom: value.unwrap()};
                    }
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    let left = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*LEFT)) { value.unwrap()} else { 0.0 };
                    let right = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*RIGHT)) { value.unwrap()} else { 0.0 };
                    let top = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*TOP)) { value.unwrap()} else { 0.0 };
                    let bottom = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*BOTTOM)) { value.unwrap()} else { 0.0 };
                    frame.inner_margin = Margin{left, right, top, bottom};
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*STROKE)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    let width: f32 = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*WIDTH)) {
                      value.unwrap()
                    } else { 1.0 };
                    let color: Color32 = if let Ok(Value::U128(color)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*COLOR)) {
                      get_color(color)
                    } else { Color32::WHITE };
                    frame.stroke = Stroke::new(width, color);
                  }
                  _ => (),
                }
              }
            }
            _ => (),
          }
        }
        panel.frame(frame).show_inside(container, |ui| {
          if let Ok(contained) = contained {
            self.render_value(contained, ui);
          }
        });
      }
    }
    Ok(())
  }

  pub fn render_panel_left(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*CONTAINS)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS))) {
      (contained,parameters_table) => {
        let mut frame = Frame::none();
        frame.stroke = Stroke::new(0.0, Color32::BLACK);
        let mut panel = egui::SidePanel::left(humanize(&table.id)).resizable(false).show_separator_line(false);
        if let Ok(Value::Reference(parameters_table_id)) = parameters_table {
          match self.core.get_table_by_id(*parameters_table_id.unwrap()) {
            Ok(parameters_table) => {
              let parameters_table_brrw = parameters_table.borrow();
              if let Ok(Value::U128(color)) = parameters_table_brrw.get(&TableIndex::Index(1),&TableIndex::Alias(*FILL)) { 
                frame.fill = get_color(color);
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MIN__WIDTH)) {
                panel = panel.min_width(value.into());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MAX__WIDTH)) {
                panel = panel.min_width(value.into());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*WIDTH)) {
                panel = panel.exact_width(value.into());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                frame.outer_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                frame.inner_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
                      frame.outer_margin = Margin{left: value.unwrap(), right: value.unwrap(), top: value.unwrap(), bottom: value.unwrap()};
                    }
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    let left = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*LEFT)) { value.unwrap()} else { 0.0 };
                    let right = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*RIGHT)) { value.unwrap()} else { 0.0 };
                    let top = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*TOP)) { value.unwrap()} else { 0.0 };
                    let bottom = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*BOTTOM)) { value.unwrap()} else { 0.0 };
                    frame.inner_margin = Margin{left, right, top, bottom};
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*STROKE)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    let width: f32 = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*WIDTH)) {
                      value.unwrap()
                    } else { 1.0 };
                    let color: Color32 = if let Ok(Value::U128(color)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*COLOR)) {
                      get_color(color)
                    } else { Color32::WHITE };
                    frame.stroke = Stroke::new(width, color);
                  }
                  _ => (),
                }
              }
            }
            _ => (),
          }
        }
        panel.frame(frame).show_inside(container, |ui| {
          if let Ok(contained) = contained {
            self.render_value(contained, ui);
          }
        });
      }
    }
    Ok(())
  }

  pub fn render_panel_right(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*CONTAINS)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS))) {
      (contained,parameters_table) => {
        let mut frame = Frame::default();
        let mut min_width = 10.0;
        let mut max_width = 100.0;
        if let Ok(Value::Reference(parameters_table_id)) = parameters_table {
          match self.core.get_table_by_id(*parameters_table_id.unwrap()) {
            Ok(parameters_table) => {
              let parameters_table_brrw = parameters_table.borrow();
              if let Ok(Value::U128(color)) = parameters_table_brrw.get(&TableIndex::Index(1),&TableIndex::Alias(*FILL)) { 
                frame.fill = get_color(color);
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MIN__WIDTH)) {
                min_width = value.into();
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MAX__WIDTH)) {
                max_width = value.into();
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*WIDTH)) {
                max_width = value.into();
                min_width = value.into();
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                frame.outer_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                frame.inner_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
                      frame.outer_margin = Margin{left: value.unwrap(), right: value.unwrap(), top: value.unwrap(), bottom: value.unwrap()};
                    }
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
                      frame.inner_margin = Margin{left: value.unwrap(), right: value.unwrap(), top: value.unwrap(), bottom: value.unwrap()};
                    }
                  }
                  _ => (),
                }
              }
            }
            _ => (),
          }
        }
        egui::SidePanel::right(humanize(&table.id))
          .resizable(false)
          .min_width(min_width)
          .max_width(max_width)
          .frame(frame)
          .show_inside(container, |ui| {
            if let Ok(contained) = contained {
              self.render_value(contained, ui);
            }
          });
      }
    }
    Ok(())
  }

  pub fn render_panel_center(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*CONTAINS)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS))) {
      (contained,parameters_table) => {
        let mut frame = Frame::default();
        if let Ok(Value::Reference(parameters_table_id)) = parameters_table {
          match self.core.get_table_by_id(*parameters_table_id.unwrap()) {
            Ok(parameters_table) => {
              let parameters_table_brrw = parameters_table.borrow();
              if let Ok(Value::U128(color)) = parameters_table_brrw.get(&TableIndex::Index(1),&TableIndex::Alias(*FILL)) { 
                frame.fill = get_color(color);
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                frame.outer_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                frame.inner_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
                      frame.outer_margin = Margin{left: value.unwrap(), right: value.unwrap(), top: value.unwrap(), bottom: value.unwrap()};
                    }
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
                      frame.inner_margin = Margin{left: value.unwrap(), right: value.unwrap(), top: value.unwrap(), bottom: value.unwrap()};
                    }
                  }
                  _ => (),
                }
              }
            }
            _ => (),
          }
        }
        egui::CentralPanel::default()
          .frame(frame)
        .show_inside(container, |ui| {
          if let Ok(contained) = contained {
            self.render_value(contained, ui);
          }
        });
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  pub fn render_link(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*TEXT)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*URL))) {
      (Ok(Value::String(text)), Ok(Value::String(url))) => {
        container.hyperlink_to(text.to_string(),url.to_string());
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  pub fn render_label(&mut self, table: &Table, container: &mut egui::Ui) -> Result<(),MechError> {
    for row in 1..=table.rows {
      match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*TEXT)),
             table.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS))) {
        (Ok(Value::String(text)), Ok(Value::Reference(parameters_table_id))) => {
          let parameters_table = self.core.get_table_by_id(*parameters_table_id.unwrap())?;
          let parameters_table_brrw = parameters_table.borrow();

          let color = if let Ok(Value::U128(color)) = parameters_table_brrw.get(&TableIndex::Index(1),&TableIndex::Alias(*COLOR)) { color }
          else { U128::new(0) };

          let size = if let Ok(Value::F32(size)) = parameters_table_brrw.get(&TableIndex::Index(1),&TableIndex::Alias(*SIZE)) { size }
          else { F32::new(12.0) };

          let label = egui::RichText::new(text.to_string())
            .color(get_color(color))
            .size(size.into());

          container.label(label);
        }
        x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
      }
    }
    Ok(())
  }

  pub fn render_frame(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*CONTAINS)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS))) {
      (contained,parameters_table) => {
        let mut frame = Frame::default();
        let mut margin = Margin {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0};
        let mut padding = Margin {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0};

        if let Ok(Value::Reference(parameters_table_id)) = parameters_table {
          match self.core.get_table_by_id(*parameters_table_id.unwrap()) {
            Ok(parameters_table) => {
              let parameters_table_brrw = parameters_table.borrow();
              if let Ok(Value::U128(color)) = parameters_table_brrw.get(&TableIndex::Index(1),&TableIndex::Alias(*FILL)) { 
                frame.fill = get_color(color);
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*ROUNDING)) {
                frame.rounding = Rounding::same(value.unwrap());
              }
              if let Ok(Value::Reference(ref_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*ROUNDING)) {
                let mut rounding = Rounding::same(0.0);
                match self.core.get_table_by_id(*ref_table_id.unwrap()) {
                  Ok(ref_table) => {
                    let ref_table_brrw = ref_table.borrow();
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*LEFT)) {
                      rounding.nw = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*TOP)) {
                      rounding.ne = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*RIGHT)) {
                      rounding.se = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*BOTTOM)) {
                      rounding.sw = value.unwrap();
                    };
                    frame.rounding = rounding;
                  }
                  _ => (),
                }
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                frame.outer_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::Reference(ref_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*MARGIN)) {
                match self.core.get_table_by_id(*ref_table_id.unwrap()) {
                  Ok(ref_table) => {
                    let ref_table_brrw = ref_table.borrow();
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*LEFT)) {
                      padding.left = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*TOP)) {
                      padding.top = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*RIGHT)) {
                      padding.right = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*BOTTOM)) {
                      padding.bottom = value.unwrap();
                    };
                    frame.outer_margin = padding;
                  }
                  _ => (),
                }
              }
              if let Ok(Value::F32(value)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                frame.inner_margin = Margin::same(value.unwrap());
              }
              if let Ok(Value::Reference(ref_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*PADDING)) {
                match self.core.get_table_by_id(*ref_table_id.unwrap()) {
                  Ok(ref_table) => {
                    let ref_table_brrw = ref_table.borrow();
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*LEFT)) {
                      padding.left = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*TOP)) {
                      padding.top = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*RIGHT)) {
                      padding.right = value.unwrap();
                    };
                    if let Ok(Value::F32(value)) = ref_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*BOTTOM)) {
                      padding.bottom = value.unwrap();
                    };
                    frame.inner_margin = padding;
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*STROKE)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    let width: f32 = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*WIDTH)) {
                      value.unwrap()
                    } else { 1.0 };
                    let color: Color32 = if let Ok(Value::U128(color)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*COLOR)) {
                      get_color(color)
                    } else { Color32::WHITE };
                    frame.stroke = Stroke::new(width, color);
                  }
                  _ => (),
                }
              }
              if let Ok(Value::Reference(margin_table_id)) = parameters_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*SHADOW)) {
                match self.core.get_table_by_id(*margin_table_id.unwrap()) {
                  Ok(margin_table) => {
                    let margin_table_brrw = margin_table.borrow();
                    let dist: f32 = if let Ok(Value::F32(value)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*DISTANCE)) {
                      value.unwrap()
                    } else { 1.0 };
                    let color: Color32 = if let Ok(Value::U128(color)) = margin_table_brrw.get(&TableIndex::Index(1), &TableIndex::Alias(*COLOR)) {
                      get_color(color)
                    } else { Color32::BLACK };
                    frame.shadow = Shadow{extrusion: dist, color};
                  }
                  _ => (),
                }
              }
            }
            _ => (),
          }
        }
        egui::SidePanel::left(humanize(&table.id))
          .frame(frame)
          .show_separator_line(false)
          .show_inside(container, |ui| {
            if let Ok(contained) = contained {
              self.render_value(contained, ui);
            }
        });
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  pub fn render_strip(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*CONTAINS)),
           table.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS))) {
      (contained,parameters_table) => {
        let mut frame = Frame::default();
        let mut margin = Margin {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0};
        let mut padding = Margin {left: 0.0, right: 0.0, top: 0.0, bottom: 0.0};
    
        if let Ok(Value::Reference(contained_table_id)) = contained {
          match self.core.get_table_by_id(*contained_table_id.unwrap()) {
            Ok(contained_table) => {
              let contained_table_brrw = contained_table.borrow();
              let cells = contained_table_brrw.get_col_raw(0)?;

              let cells = match cells {
                Column::Ref(c) => c,
                _ => {return Err(MechError{msg: "".to_string(), id: 6498, kind: MechErrorKind::None});}
              };

              StripBuilder::new(container)
                .size(Size::initial(250.0 * cells.len() as f32)) 
                .size(Size::remainder())
                .horizontal(|mut strip| {
                  strip.strip(|builder| {
                    builder.sizes(Size::initial(250.0), cells.len()).horizontal(|mut strip| {
                      for i in 0..cells.len() {
                        strip.cell(|ui| {
                          let mut r = ui.available_rect_before_wrap();
                          if i == 1 {r.set_height(50.0);}
                          ui.painter().rect_filled(
                            r,
                            0.0,
                            Color32::BLUE,
                          );
                          self.render_value(Value::Reference(cells.get_unchecked(i)), ui);
                        });
                      }
                    });
                  });
                  strip.cell(|ui| {
         
                  });
                });
            }
            _ => (),
          }
        }
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }
  
  pub fn render_table__window(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*TEXT)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*CLICKED))) {
        (Ok(Value::String(text)), Ok(Value::Reference(value_table_id))) => {
        let value_table = self.core.get_table_by_id(*value_table_id.unwrap())?;
        let value_table_brrw = value_table.borrow();
        match value_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
          Ok(Value::Bool(value)) => {
            if container.add(egui::Button::new(text.to_string())).clicked() {
              let new_value = !value;
              if new_value {
                self.windows.insert(text.to_string());
              } else {
                self.windows.remove(&text.to_string());
              }
              self.changes.push(Change::Set((value_table_brrw.id,vec![(TableIndex::Index(1),TableIndex::Index(1),Value::Bool(!value))])));
            }
          }
          x => {return Err(MechError{msg: "".to_string(), id: 6497, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
        }
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6497, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  pub fn render_button(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*TEXT)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*CLICKED))) {
        (Ok(Value::String(text)), Ok(Value::Reference(value_table_id))) => {
        let value_table = self.core.get_table_by_id(*value_table_id.unwrap())?;
        let value_table_brrw = value_table.borrow();
        match value_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
          Ok(Value::Bool(value)) => {
            if container.add(egui::Button::new(text.to_string())).clicked() {
              self.changes.push(Change::Set((value_table_brrw.id,vec![(TableIndex::Index(1),TableIndex::Index(1),Value::Bool(!value))])));
            }
          }
          x => {return Err(MechError{msg: "".to_string(), id: 6497, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
        }
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6497, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  pub fn render_slider(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match (table.get(&TableIndex::Index(row), &TableIndex::Alias(*MIN)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*MAX)),
            table.get(&TableIndex::Index(row), &TableIndex::Alias(*VALUE))) {
        (Ok(Value::F32(min)), Ok(Value::F32(max)), Ok(Value::Reference(value_table_id))) => {
        let value_table = self.core.get_table_by_id(*value_table_id.unwrap())?;
        let value_table_brrw = value_table.borrow();
        match value_table_brrw.get(&TableIndex::Index(1), &TableIndex::Index(1)) {
          Ok(Value::F32(value)) => {
            self.ticks = value.into();
            let response = container.add(egui::Slider::new(&mut self.ticks, min.into()..=max.into()));
            if response.changed() {
              self.changes.push(Change::Set((value_table_brrw.id,vec![(TableIndex::Index(1),TableIndex::Index(1),Value::F32(F32::new(self.ticks)))])));
            }
          }
          x => {return Err(MechError{msg: "".to_string(), id: 6497, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
        }
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6497, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }  

  pub fn render_debug(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    egui::ScrollArea::vertical().show(container, |ui| {
      ui.label(format!("{:?}", self.core));
    });
    Ok(())
  }  

  pub fn render_canvas(&mut self, table: &Table, row: usize, container: &mut egui::Ui) -> Result<(),MechError> {
    match table.get(&TableIndex::Index(row), &TableIndex::Alias(*CONTAINS)) {
      Ok(Value::Reference(contains_table_id)) => {
        Frame::none().show(container, |ui| {
          let table = self.core.get_table_by_id(*contains_table_id.unwrap()).unwrap();
          let table_brrw = table.borrow();
          match (table_brrw.get(&TableIndex::Index(row), &TableIndex::Alias(*SHAPE)),
                  table_brrw.get(&TableIndex::Index(row), &TableIndex::Alias(*PARAMETERS)))  {
            (Ok(Value::String(kind)),Ok(Value::Reference(contains_table_id))) => {
              let table = self.core.get_table_by_id(*contains_table_id.unwrap()).unwrap();
              let table_brrw = table.borrow();
              let raw_kind = kind.hash();
              // Render an element
              if raw_kind == *CIRCLE { 
                let shapes = self.render_circle(&table_brrw,ui)?;
                ui.painter().extend(shapes);
              } else {
                return Err(MechError{msg: "".to_string(), id: 6489, kind: MechErrorKind::GenericError(format!("{:?}", raw_kind))});
              }
            }
            x => {
              return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});
            },
          }
          Ok(())
        });
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6496, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(())
  }

  pub fn render_circle(&mut self, table: &Table, container: &mut egui::Ui) -> Result<Vec<epaint::Shape>,MechError> {
    let mut shapes = vec![];
    match (table.get_column(&TableIndex::Alias(*CENTER__X)),
           table.get_column(&TableIndex::Alias(*CENTER__Y))) {
      (Ok(Column::F32(x)), Ok(Column::F32(y))) => {
  
        let radius = if let Ok(Column::F32(radius)) = table.get_column(&TableIndex::Alias(*RADIUS)) { radius }
        else { ColumnV::new(vec![F32::new(1.0); table.rows]) };

        let line_width = if let Ok(Column::F32(line_width)) = table.get_column(&TableIndex::Alias(*LINE__WIDTH)) { line_width }
        else { ColumnV::new(vec![F32::new(0.0); table.rows]) };

        let fill = if let Ok(Column::U128(fill)) = table.get_column(&TableIndex::Alias(*FILL)) { fill }
        else { ColumnV::new(vec![U128::new(0); table.rows]) };

        let stroke = if let Ok(Column::U128(color)) = table.get_column(&TableIndex::Alias(*STROKE)) { color }
        else { ColumnV::new(vec![U128::new(0); table.rows]) };

        let radius_brrw = radius.borrow();
        let line_width_brrw = line_width.borrow();
        let stroke_brrw = stroke.borrow();
        let fill_brrw = fill.borrow();

        let x_brrw = x.borrow();
        let y_brrw = y.borrow();

        for i in 0..table.rows {
          let line_width: f32 = line_width_brrw[i].into();
          shapes.push(epaint::Shape::Circle(epaint::CircleShape{
            center: Pos2{x: x_brrw[i].into(), y: y_brrw[i].into()},
            radius: radius_brrw[i].into(),
            fill: get_color(fill_brrw[i]),
            stroke: epaint::Stroke::new(line_width, get_color(stroke_brrw[i])),
          }));
        }
      }
      x => {return Err(MechError{msg: "".to_string(), id: 6497, kind: MechErrorKind::GenericError(format!("{:?}", x))});},
    }
    Ok(shapes)
  }

}

pub fn get_color(color_value: U128) -> Color32 {
  let color: u32 = color_value.into();
  let r = (color >> 16) as u8;
  let g = (color >> 8) as u8;
  let b = color as u8;
  Color32::from_rgb(r,g,b)
}

impl eframe::App for MechApp {

  fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    let Self { ticks, core, .. } = self;

    let windows = self.windows.clone();
    
    for table_id in windows {
      egui::Window::new(table_id.clone()).show(ctx, |ui| {
        let table = self.core.get_table(&table_id).unwrap();
        let table_brrw = table.borrow();
        self.make_element(&table_brrw,ui);
      });
    }

    // Set font
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("FiraCode-Regular".to_owned(),FontData::from_static(include_bytes!("../../../assets/fonts/FiraCode-Regular.ttf")));
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "FiraCode-Regular".to_owned());
    ctx.set_fonts(fonts);

    // Draw frame
    let mut frame = Frame::default();
    frame.fill = Color32::from_rgb(0x13,0x12,0x18);
    egui::CentralPanel::default()
      .frame(frame)
      .show(ctx, |ui| {
      // Compile new code...
      {
        let code_table = self.core.get_table("mech/compiler").unwrap();
        let code_table_brrw = code_table.borrow();
        if let Value::String(code_string) = code_table_brrw.get(&TableIndex::Index(1),&TableIndex::Index(1)).unwrap() {
          if code_string.to_string() != "" {
            let mut compiler = Compiler::new();
            match compiler.compile_str(&code_string.to_string()) {
              Ok(sections) => {
                self.core.load_sections(sections);
                self.core.schedule_blocks();    
                self.changes.push(Change::Set((hash_str("mech/compiler"),vec![
                  (TableIndex::Index(1),TableIndex::Index(1),Value::String(MechString::from_string("".to_string())))
                ])));
              }
              Err(_) => (), // No blocks compiled
            }
          }
        }
      }

      //ui.ctx().request_repaint();
      self.render_app(ui);

      // Update IO
      let time = ui.input().time;
      self.frame += 1;
      self.changes.push(Change::Set((hash_str("time/timer"),vec![(TableIndex::Index(1),TableIndex::Index(2),Value::U64(U64::new(self.frame as u64)))])));
      match ui.input().pointer.hover_pos() {
        Some(pos) => {
          self.changes.push(Change::Set((hash_str("io/pointer"),vec![
            (TableIndex::Index(1),TableIndex::Index(1),Value::F32(F32::new(pos.x))),
            (TableIndex::Index(1),TableIndex::Index(2),Value::F32(F32::new(pos.y)))
          ])));
        }
        _ => (),
      }
      if ui.input().keys_down.contains(&egui::Key::Escape) {
        //let core = load_mech();
        //self.core = core;
      }
      self.core.process_transaction(&self.changes);
      self.changes.clear();
    });
  }

  fn warm_up_enabled(&self) -> bool {
    true
  }

}

//pub fn load_mech(program_path: &str) -> Result<mech_core::Core,MechError> {
  /*let code_string = include_str!(program_path);
  let code_string = match fs::read_to_string(program_path) {
    Ok(code) => code,
    Err(err) => {return Err(MechError{msg: "".to_string(), id: 87491, kind: MechErrorKind::GenericError(format!("{:?}",err))});}
  };*/
pub fn load_mech() -> Result<mech_core::Core,MechError> {
  let code_string = include_str!(r#"notebook.mec"#);
  let mut mech_core = mech_core::Core::new();
  let mut compiler = Compiler::new(); 
  match compiler.compile_str(&code_string) {
    Ok(sections) => {
      mech_core.load_sections(sections);
    }
    Err(x) => {
      
    }
  }
  
  let mut code = r#"
#time/timer = [|period<ms> ticks<u64>|]
#mech/compiler = [|code<string>| "hi"]
#io/pointer = [|x<f32> y<f32>| 0 0]"#.to_string();
  
  code += r#"
#mech/tables = [|name<string>|
                "time/timer"
                "io/pointer"
                "mech/tables"
                "mech/compiler""#;
  for name in mech_core.table_names() {
  code += &format!("\n{:?}",name);     
  }
  code += "]";
  
  let mut compiler = Compiler::new();
  let sections = compiler.compile_str(&code).unwrap();
  mech_core.load_sections(sections);
  mech_core.schedule_blocks();
  Ok(mech_core)
}

pub fn load_icon() -> eframe::IconData {
  let bytes = vec![153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 246, 192, 78, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 248, 208, 122, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255, 153, 105, 0, 255];
  eframe::IconData{rgba: bytes.to_vec(), width: 64, height: 64}
}

fn main() {
  //let input = std::env::args().nth(1).unwrap();
  let mut native_options = eframe::NativeOptions::default();
  let icon = load_icon();
  let core = load_mech().unwrap();
  native_options.icon_data = Some(icon);
  native_options.min_window_size = Some(Vec2{x: 500.0, y: 400.0});
  eframe::run_native("Mech Notebook", native_options, Box::new(|cc| 
    Box::new(MechApp::new(cc,core))));
}


