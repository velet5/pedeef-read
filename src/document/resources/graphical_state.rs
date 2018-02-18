use std::any::Any;
use std::collections::HashMap;

use object::Reference;

use reader::result::ReadResult;

use document::reader::DocumentReader;
use document::boxed::*;
use document::dictionary::*;


#[derive(Debug)]
pub struct Function {

}


#[derive(Debug)]
pub struct GraphicalState {
  tpe: Option<String>,
  line_width: Option<f32>,
  line_cap_style: Option<i32>,
  line_join_style: Option<i32>,
  miter_limit: Option<f32>,
  dash_pattern: Option<(Vec<u32>, u32)>,
  rendering_intents: Option<String>,
  overprint: Option<bool>,
  overprint_global: Option<bool>,
  overprint_mode: Option<i32>,
  font: Option<Vec<(Reference, f32)>>,
  black_generation: Option<Function>,
  black_generation_2: Option<Function>, // actually function or name "Default"
  undercolor_removal: Option<Function>,
  undercolor_removal_2: Option<Function>, // or name
  transfer_function: Option<Function>, // or name or array
  transfer_function_2: Option<Function>, // or name or array
  halftone: Option<String>, // or dictionary or stream
  flatness_tolerance: Option<f32>,
  smoothness_tolerance: Option<f32>,
  stroke_adjustment: Option<bool>,
  blend_mode: Option<String>, // or array
  soft_mask: Option<String>, // or dictionary
  stroking_alpha: Option<f32>,
  nonstroking_alpha: Option<f32>,
  alpha_source_flag: Option<bool>,
  text_knockout_flag: Option<bool>
}


pub fn read_graphical_state(reader: &mut DocumentReader) -> ReadResult<GraphicalState> {
  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("Type", &read_name_boxed);
  map.insert("CA", &read_float_boxed);
  map.insert("ca", &read_float_boxed);
  map.insert("SA", &read_bool_boxed);
  map.insert("OP", &read_bool_boxed);
  map.insert("op", &read_bool_boxed);
  map.insert("OPM", &read_int_boxed);

  let dictionary = &mut read_dictionary(reader, &map)?;

  let tpe = *unfold_optional("Type", dictionary)?;
  let stroking_alpha = *unfold_optional("CA", dictionary)?;
  let nonstroking_alpha = *unfold_optional("ca", dictionary)?;
  let stroke_adjustment = *unfold_optional("SA", dictionary)?;
  let overprint = *unfold_optional("OP", dictionary)?;
  let overprint_global = *unfold_optional("op", dictionary)?;
  let overprint_mode = *unfold_optional("OPM", dictionary)?;

  Ok(GraphicalState {
    tpe,
    line_width: None,
    line_cap_style: None,
    line_join_style: None,
    miter_limit: None,
    dash_pattern: None,
    rendering_intents: None,
    overprint,
    overprint_global,
    overprint_mode,
    font: None,
    black_generation: None,
    black_generation_2: None,
    undercolor_removal: None,
    undercolor_removal_2: None,
    transfer_function: None,
    transfer_function_2: None,
    halftone: None,
    flatness_tolerance: None,
    smoothness_tolerance: None,
    stroke_adjustment,
    blend_mode: None,
    soft_mask: None,
    stroking_alpha,
    nonstroking_alpha,
    alpha_source_flag: None,
    text_knockout_flag: None,
  })
}


pub fn read_graphical_state_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_graphical_state(reader))
}
