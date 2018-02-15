use std::any::Any;
use reader::result::ReadResult;
use document::reader::DocumentReader;


#[derive(Debug)]
struct Empty;

#[derive(Debug)]
pub struct Resources {
  ext_g_state: Option<Empty>,
  color_space: Option<Empty>,
  pattern: Option<Empty>,
  shading: Option<Empty>,
  x_object: Option<Empty>,
  font: Option<Empty>,
  proc_set: Option<Empty>
}


pub fn read_resources(reader: &mut DocumentReader) -> ReadResult<Resources> {
  unimplemented!()
}