use object::Object;
use reader::result::ReadResult;
use reader::stream::Stream;
use reader::common::*;


pub fn read_byte_string(stream: &mut Stream) -> ReadResult<String> {
  skip(stream, "<");
  let value = read_regular_string(stream);
  skip(stream, ">");

  Ok(value)
}


pub fn read_byte_string_object(stream: &mut Stream) -> ReadResult<Object> {
  let string = read_byte_string(stream)?;

  Ok(Object::String(string))
}