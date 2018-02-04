use std::collections::HashMap;

use document::xref::XrefTable;
use object::Reference;
use reader::common::*;
use reader::result::ReadResult;
use reader::stream::Stream;


#[derive(Debug)]
pub struct ObjectMap {
  pub value: HashMap<Reference, usize>
}


pub fn read_object_map(stream: &mut Stream, xref: &XrefTable) -> ReadResult<ObjectMap> {
  let mut map = HashMap::new();

  for entry in &xref.entries {
    if entry.is_used {
      let reference = read_object_id(stream, entry.offset)?;
      map.insert(reference, entry.offset);
    }
  }

  Ok(ObjectMap { value: map })
}


pub fn read_object_id(stream: &mut Stream, position: usize) -> ReadResult<Reference> {
  stream.set_forward_mode();
  stream.set_position(position);

  let id = read_int(stream)?;
  skip_space(stream)?;

  let generation = read_int(stream)?;
  skip_space(stream)?;
  skip(stream, "obj")?;

  Ok(Reference { id, generation })
}