use reader::stream::Stream;
use reader::result::ReadResult;
use object::Reference;
use reader::common::*;
use object::Object;


pub fn read_reference(stream: &mut Stream) -> ReadResult<Reference> {
  let id = read_int(stream)?;
  skip(stream, " ")?;
  let generation = read_int(stream)?;
  skip(stream, " R")?;

  Ok(Reference { id, generation })
}


pub fn read_reference_object(stream: &mut Stream) -> ReadResult<Object> {
  let reference = read_reference(stream)?;

  Ok(Object::Reference(reference))
}


pub fn to_reference(object: Object) -> Option<Reference> {
  match object {
    Object::Reference(value) => Some(value),
    _ => None
  }
}