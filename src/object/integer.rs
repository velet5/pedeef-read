use object::Object;
use reader::result::ReadResult;
use reader::stream::Stream;
use reader::common::*;


pub fn read_integer(stream: &mut Stream) -> ReadResult<Object> {
  let number = read_int(stream)?;

  Ok(Object::Integer(number))
}
