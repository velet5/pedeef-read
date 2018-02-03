use reader::common::*;
use reader::stream::Stream;
use reader::result::ReadResult;


pub fn read_name_string(stream: &mut Stream) -> ReadResult<String> {
  skip(stream, "/")?;
  Ok(read_regular_string(stream))
}