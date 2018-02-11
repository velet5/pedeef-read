use std::any::Any;
use std::collections::HashMap;

use reader::common::*;
use reader::characters::*;
use reader::result::ReadResult;

use document::reader::DocumentReader;
use document::boxed::*;
use document::dictionary::*;


#[derive(Debug)]
pub struct ViewerPreferences {
  hide_toolbar: Option<bool>,
  hide_menubar: Option<bool>,
  hide_window_ui: Option<bool>,
  fit_window: Option<bool>,
  center_window: Option<bool>,
  display_doc_title: Option<bool>,
  non_fullscreen_page_mode: Option<String>,
  direction: Option<String>,
  view_area: Option<String>,
  view_clip: Option<String>,
  print_area: Option<String>,
  print_clip: Option<String>,
  print_scaling: Option<String>,
  duplex: Option<String>,
  pick_tray_by_pdf_size: Option<bool>,
  print_page_range: Option<Vec<[i32; 2]>>,
  num_copies: Option<i32>
}


pub fn read_viewer_preferences(reader: &mut DocumentReader) -> ReadResult<ViewerPreferences> {
  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("HideToolbar", &read_bool_boxed);
  map.insert("HideMenuBar", &read_bool_boxed);
  map.insert("HideWindowUI", &read_bool_boxed);
  map.insert("FitWindow", &read_bool_boxed);
  map.insert("CenterWindow", &read_bool_boxed);
  map.insert("DisplayDocTitle", &read_bool_boxed);
  map.insert("NonFullScreenPageMode", &read_name_boxed);
  map.insert("Direction", &read_name_boxed);
  map.insert("ViewArea", &read_name_boxed);
  map.insert("ViewClip", &read_name_boxed);
  map.insert("PrintArea", &read_name_boxed);
  map.insert("PrintClip", &read_name_boxed);
  map.insert("PrintScaling", &read_name_boxed);
  map.insert("Duplex", &read_name_boxed);
  map.insert("PickTrayByPDFSize", &read_bool_boxed);
  map.insert("PrintPageRange", &read_int_array_boxed);
  map.insert("NumCopies", &read_int_boxed);

  let dictionary = &mut read_dictionary(reader, &map)?;

  let hide_toolbar = *unfold_optional("HideToolbar", dictionary)?;
  let hide_menubar = *unfold_optional("HideMenuBar", dictionary)?;
  let hide_window_ui = *unfold_optional("HideWindowUI", dictionary)?;
  let fit_window = *unfold_optional("FitWindow", dictionary)?;
  let center_window = *unfold_optional("CenterWindow", dictionary)?;
  let display_doc_title = *unfold_optional("DisplayDocTitle", dictionary)?;
  let non_fullscreen_page_mode = *unfold_optional("NonFullScreenPageMode", dictionary)?;
  let direction = *unfold_optional("Direction", dictionary)?;
  let view_area = *unfold_optional("ViewArea", dictionary)?;
  let view_clip = *unfold_optional("ViewClip", dictionary)?;
  let print_area = *unfold_optional("PrintArea", dictionary)?;
  let print_clip = *unfold_optional("PrintClip", dictionary)?;
  let print_scaling = *unfold_optional("PrintScaling", dictionary)?;
  let duplex = *unfold_optional("Duplex", dictionary)?;
  let pick_tray_by_pdf_size = *unfold_optional("PickTrayByPDFSize", dictionary)?;
  let print_page_range = *unfold_optional("PrintPageRange", dictionary)?;
  let num_copies = *unfold_optional("NumCopies", dictionary)?;

  Ok(ViewerPreferences {
    hide_toolbar,
    hide_menubar,
    hide_window_ui,
    fit_window,
    center_window,
    display_doc_title,
    non_fullscreen_page_mode,
    direction,
    view_area,
    view_clip,
    print_area,
    print_clip,
    print_scaling,
    duplex,
    pick_tray_by_pdf_size,
    print_page_range,
    num_copies
  })
}


pub fn read_viewer_preferences_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_viewer_preferences(reader))
}


pub fn read_int_array(reader: &mut DocumentReader) -> ReadResult<Vec<i32>> {
  let mut buffer = Vec::new();

  skip(&mut reader.stream, "[")?;
  skip_whitespace(&mut reader.stream);

  while is_digit(reader.stream.peek()) {
    let number = read_int(&mut reader.stream)?;
    skip_whitespace(&mut reader.stream);
    buffer.push(number);
  }

  skip(&mut reader.stream, "]")?;
  Ok(buffer)
}


pub fn read_int_array_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_int_array(reader))
}