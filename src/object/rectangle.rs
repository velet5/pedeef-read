use reader::common::*;
use reader::stream::Stream;
use reader::result::ReadResult;

#[derive(Debug)]
pub struct Rectangle {
  left: f32,
  bottom: f32,
  right: f32,
  top: f32
}


pub fn read_rectangle(stream: &mut Stream) -> ReadResult<Rectangle> {
  skip_whitespace(stream);
  skip(stream, "[")?;
  skip_whitespace(stream);

  let left = read_float(stream)?;
  let bottom = read_float(stream)?;
  let right = read_float(stream)?;
  let top = read_float(stream)?;

  skip_whitespace(stream);
  skip(stream, "]")?;
  skip_whitespace(stream);

  Ok(Rectangle {left, bottom, right, top})
}