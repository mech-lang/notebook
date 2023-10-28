//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]
#![recursion_limit="256"]

use eframe::{egui};
use eframe::egui::{containers::*, *};
use egui::style::Margin;
use egui_extras::{StripBuilder, Size};

use mech_gui::button::MyButton;
use mech_gui::tabs::MyButtonTabs;
use mech_gui::{FrameStroke,Style};

use native_dialog::{FileDialog, MessageDialog, MessageType};

extern crate mech;

use mech::utilities::*;
use mech::program::*;
use crate::epaint::Shadow;
use mech::core::*;
use mech::syntax::compiler::Compiler;
use std::thread::JoinHandle;
extern crate image;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::sync::Mutex;

use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::{ImageBuffer, Pixel, Rgba};

pub fn load_mech_from_path(program_path: &str) -> Result<mech::core::Core,MechError> {
  match fs::read_to_string(program_path) {
    Ok(code) => {
      let mut mcore = mech::core::Core::new();
      let mut compiler = Compiler::new(); 
      let core = match compiler.compile_str(&code) {
        Ok(sections) => {
          let load_result = mcore.load_sections(sections);
          for (_,_,errors) in load_result {
            println!("{:?}", errors);
          }
        }
        Err(x) => {
          return Err(MechError{msg: "".to_string(), id: 87491, kind: MechErrorKind::GenericError(format!("{:?}",x))})
        }
      };
      let mut code = r#"
#time/timer = [|period<ms> ticks<u64>|]
#notebook/compiler = [|code<string>| "hi"]
#io/pointer = [|x<f32> y<f32>| 0 0]"#.to_string();
        
        code += r#"
#mech/tables = ["time/timer"
                "io/pointer"
                "mech/tables"
                "notebook/compiler""#;
      for name in mcore.table_names() {
        code += &format!("\n{:?}",name);     
      }
      code += "]";
      
      let mut compiler = Compiler::new();
      let sections = compiler.compile_str(&code).unwrap();
      let load_result = mcore.load_sections(sections);
      for (_,_,errors) in load_result {
        println!("{:?}", errors);
      }
      mcore.schedule_blocks()?;
      Ok(mcore)
    },
    Err(err) => Err(MechError{msg: "".to_string(), id: 87491, kind: MechErrorKind::GenericError(format!("{:?}",err))}),
  }
}

pub fn load_mech() -> Result<mech::core::Core,MechError> {
  let code_string = include_str!(r#"notebook.mec"#);
  let mut mcore = mech::core::Core::new();
  let mut compiler = Compiler::new(); 
  match compiler.compile_str(&code_string) {
    Ok(sections) => {
      mcore.load_sections(sections);
    }
    Err(x) => {
      
    }
  }
  
  let mut code = r#"
#time/timer = [|period<ms> ticks<u64>|]
#notebook/compiler = [|code<string>| "hi"]
#io/pointer = [|x<f32> y<f32> primary-down<bool>| 0 0 ✗]
#io/keyboard = [|space enter|
                 ✗     ✗]"#.to_string();
  code += r#"
#mech/tables = [|name<string>|
                "time/timer"
                "io/pointer"
                "io/keyboard"
                "mech/tables"
                "notebook/compiler""#;
  for (table,row,col) in &mcore.output {
    let table = match mcore.dictionary.borrow().get(table.unwrap()) {
      Some(name) => {code += &format!("\n{:?}",name.to_string());}
      None => (),
    };
  }
  code += "]";
  let mut compiler = Compiler::new();
  let sections = compiler.compile_str(&code).unwrap();
  mcore.load_sections(sections);
  mcore.schedule_blocks();
  Ok(mcore)
}

fn main() {
  //let input = std::env::args().nth(1).unwrap();
  /*let mut native_options = eframe::NativeOptions::default();
  let icon = load_icon();
  let core = load_mech().unwrap();
  //native_options.icon_data = Some(icon);
  native_options.min_window_size = Some(Vec2{x: 500.0, y: 400.0});
  native_options.initial_window_size = Some(Vec2{x: 1280.0, y: 720.0});
  eframe::run_native("Mech Notebook", native_options, Box::new(|cc| 
    Box::new(MechApp::new(cc,core))));*/
}


