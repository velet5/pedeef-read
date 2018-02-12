use reader::stream::Stream;
use reader::result::ReadResult;

pub struct Rectangle {
  left: f32,
  bottom: f32,
  right: f32,
  top: f32
}


pub fn read_rectangle(stream: &mut Stream) -> ReadResult<Rectangle> {
  unimplemented!()
}