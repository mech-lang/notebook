#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui::*;
use mech_core::*;
use mech_syntax::parser;
use mech_core::interpreter::*;
use mech_notebook::*;
use std::sync::Arc;

fn main() -> eframe::Result {
  env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

  let icon = icon::load_icon();


  let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]).with_icon(Arc::new(icon)),
      ..Default::default()
  };

  // Our application state:
  let mut terminal_input = String::new();
  let mut terminal_output = String::new();
  let mut text_edit_focus_id = egui::Id::new("terminal_input");
  let mut intrp = Interpreter::new();
  let mut scroll_to_bottom = false;
  terminal_output.push_str("Mech v0.2.16\n");

  eframe::run_simple_native("Mech Terminal", options, move |ctx, _frame| {

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = Color32::from_rgb(24,0,14);
    visuals.extreme_bg_color = Color32::from_rgb(24,0,14);
    ctx.set_visuals(visuals);


    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("FiraCode-Regular".to_owned(),FontData::from_static(include_bytes!("../../fonts/FiraCode-Regular.ttf")));
    fonts.families.get_mut(&FontFamily::Proportional).unwrap().insert(0, "FiraCode-Regular".to_owned());
    ctx.set_fonts(fonts);

    let screen_rect = ctx.screen_rect();
    let window_size = screen_rect.height();

    egui::CentralPanel::default().show(ctx, |ui| {
      egui::ScrollArea::vertical()
        .max_height(window_size - 50.0)
        .stick_to_bottom(true)
        .animated(false)
        .show(ui, |ui| {
          ui.label(&terminal_output);
          if scroll_to_bottom {
            ui.scroll_to_cursor(Some(Align::BOTTOM));
            scroll_to_bottom = false;
          }
        });
      ui.horizontal(|ui| {
        ui.label(">:");
        let response = ui.add(
          egui::TextEdit::singleline(&mut terminal_input)
            .id(text_edit_focus_id)
            .frame(false)
        );

        if response.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
          terminal_output.push_str(&format!(">: {}\n", terminal_input));
          match parser::parse(&terminal_input) {
            Ok(tree) => { 
              let result = intrp.interpret(&tree);
              let out_str = match result {
                Ok(r) => format!("{}\n",r.pretty_print()),
                Err(err) => format!("{:?}", err),
              };
              terminal_output.push_str(&out_str);
              scroll_to_bottom = true;
            }
            Err(err) => {
              
            }
          }
          terminal_input.clear();
        }
        response.request_focus();
      });
    });
  })
}