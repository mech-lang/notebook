#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};
use eframe::egui::*;
use eframe::egui::style::{Margin,WidgetVisuals};

pub mod icon;

use mech_utilities::*;
use mech_program::*;
use mech_core::*;
use mech_syntax::compiler::Compiler;

use std::fs;

pub fn load_mech_from_path(program_path: &str) -> Result<mech_core::Core,MechError> {
  match fs::read_to_string(program_path) {
    Ok(code) => {
      let mut mcore = mech_core::Core::new();
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

pub fn load_mech() -> Result<mech_core::Core,MechError> {
  let code_string = include_str!(r#"bin/notebook.mec"#);
  let mut mcore = mech_core::Core::new();
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