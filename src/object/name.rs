use reader::common::*;
use reader::stream::Stream;
use reader::result::ReadResult;

use object::Object;


pub fn read_name_string(stream: &mut Stream) -> ReadResult<String> {
  skip(stream, "/")?;
  Ok(read_regular_string(stream))
}


pub fn read_name(stream: &mut Stream) -> ReadResult<Object> {
  let string = read_name_string(stream)?;

  Ok(Object::Name(string))
}