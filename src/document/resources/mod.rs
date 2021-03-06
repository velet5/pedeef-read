pub mod graphical_state;

use std::any::Any;
use std::collections::HashMap;

use reader::common::*;
use reader::characters::*;
use reader::result::ReadResult;

use object::Reference;
use object::name::*;
use object::reference::*;

use document::boxed::*;
use document::dictionary::*;
use document::referenced::*;
use document::reader::DocumentReader;

use self::graphical_state::*;


#[derive(Debug)]
struct Empty;



#[derive(Debug)]
pub struct Resources {
  ext_g_state: Option<HashMap<String, GraphicalState>>,
  color_space: Option<Empty>,
  pattern: Option<HashMap<String, Reference>>,
  shading: Option<Empty>,
  x_object: Option<Empty>,
  font: Option<HashMap<String, Reference>>,
  proc_set: Option<Vec<String>>,
  properties: Option<Empty>
}



pub fn read_resources(reader: &mut DocumentReader) -> ReadResult<Resources> {
  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("Font", &read_name_reference_map_boxed);
  map.insert("ProcSet", &read_name_array_boxed);
  map.insert("ExtGState", &read_name_graphical_state_map_boxed);
  map.insert("Pattern", &read_name_reference_map_boxed);
  map.insert("Type", &read_name_boxed);

  let dictionary = &mut read_dictionary(reader, &map)?;

  let font = *unfold_optional("Font", dictionary)?;
  let proc_set = *unfold_optional("ProcSet", dictionary)?;
  let ext_g_state = *unfold_optional("ExtGState", dictionary)?;
  let pattern = *unfold_optional("Pattern", dictionary)?;

  Ok(Resources {
    ext_g_state,
    color_space: None,
    pattern,
    shading: None,
    x_object: None,
    font,
    proc_set,
    properties: None,
  })
}

pub fn read_resources_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_unfold_reference(reader, &read_resources))
}


fn read_name_reference_map_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_name_reference_map(reader))
}


fn read_name_graphical_state_map(reader: &mut DocumentReader) -> ReadResult<HashMap<String, GraphicalState>> {
  skip_whitespace(&mut reader.stream);
  skip(&mut reader.stream, "<<")?;
  skip_whitespace(&mut reader.stream);

  let mut buffer = HashMap::new();

  while reader.stream.peek() != GREATER_THAN {
    let name = read_name_string(&mut reader.stream)?;
    skip_whitespace(&mut reader.stream);

    let graphical_state = read_unfold_reference(reader, &read_graphical_state)?;
    skip_whitespace(&mut reader.stream);

    buffer.insert(name, graphical_state);
  }

  skip(&mut reader.stream, ">>")?;
  skip_whitespace(&mut reader.stream);

  Ok(buffer)
}


fn read_name_graphical_state_map_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_name_graphical_state_map(reader))
}


fn read_name_array(reader: &mut DocumentReader) -> ReadResult<Vec<String>> {
  let mut buffer = Vec::new();

  let stream = &mut reader.stream;

  skip_whitespace(stream);
  skip(stream, "[")?;
  skip_whitespace(stream);

  while stream.peek() == SOLIDUS {
    let name = read_name_string(stream)?;
    buffer.push(name);
    skip_whitespace(stream);
  }

  skip(stream, "]")?;
  skip_whitespace(stream);

  Ok(buffer)
}


fn read_name_array_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_name_array(reader))
}


fn read_name_reference_map(reader: &mut DocumentReader) -> ReadResult<HashMap<String, Reference>> {
  let mut buffer = HashMap::new();
  let stream = &mut reader.stream;

  skip_whitespace(stream);
  skip(stream, "<<")?;
  skip_whitespace(stream);

  while stream.peek() == SOLIDUS {
    let name = read_name_string(stream)?;
    skip_whitespace(stream);
    let reference = read_reference(stream)?;
    skip_whitespace(stream);

    buffer.insert(name, reference);
  }

  skip_whitespace(stream);
  skip(stream, ">>")?;
  skip_whitespace(stream);

  Ok(buffer)
}
