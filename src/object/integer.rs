use object::Object;
use reader::result::ReadResult;
use reader::stream::Stream;
use reader::common::*;


pub fn read_integer(stream: &mut Stream) -> ReadResult<Object> {
  let number = read_int(stream)?;

  Ok(Object::Integer(number))
}


pub fn to_int(object: Object) -> Option<i32> {
  match object {
    Object::Integer(value) => Some(value),
    _ => None
  }
}