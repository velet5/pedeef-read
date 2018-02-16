use std::any::Any;
use std::collections::HashMap;

use reader::result::ReadResult;

use document::reader::DocumentReader;
use document::boxed::*;
use document::dictionary::*;

pub struct Group {
  tpe: String,
  subtype: String,
  colour_space: Option<String>,
  isolated: Option<bool>
}


pub fn read_group(reader: &mut DocumentReader) -> ReadResult<Group> {
  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("Type", &read_name_boxed);
  map.insert("S", &read_name_boxed);
  map.insert("CS", &read_name_boxed);
  map.insert("I", &read_bool_boxed);

  let dictionary = &mut read_dictionary(reader, &map)?;

  let tpe = *unfold("Type", dictionary)?;
  let subtype = *unfold("S", dictionary)?;
  let colour_space = *unfold_optional("CS", dictionary)?;
  let isolated = *unfold_optional("I", dictionary)?;

  Ok(Group { tpe, subtype, colour_space, isolated })
}


pub fn read_group_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_group(reader))
}