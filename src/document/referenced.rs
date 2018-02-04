use reader::stream::Stream;
use reader::result::ReadResult;
use reader::common::*;


pub fn read_unfold_reference<T>(
  stream: &mut Stream,
  reader: &Fn(&mut Stream) -> ReadResult<T>) -> ReadResult<T> {
  if is_reference_ahead(stream) {
    unimplemented!()
  } else {
    reader(stream)
  }
}


fn is_reference_ahead(stream: &mut Stream) -> bool {
  let position = stream.position();

  skip_whitespace(stream);

  if read_int(stream).is_err() {
    stream.set_position(position);
    return false
  }

  if skip_space(stream).is_err() {
    stream.set_position(position);
    return false
  }

  if read_int(stream).is_err() {
    stream.set_position(position);
    return false
  }

  if skip(stream, " R").is_err() {
    stream.set_position(position);
    return false
  }

  stream.set_position(position);
  return true
}

