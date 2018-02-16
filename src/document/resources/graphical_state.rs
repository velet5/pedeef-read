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
}


pub fn read_graphical_state(reader: &mut DocumentReader) -> ReadResult<GraphicalState> {
  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  let dictionary = &mut read_dictionary(reader, &map)?;

  unimplemented!()
}


pub fn read_graphical_state_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_graphical_state(reader))
}