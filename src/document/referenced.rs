use document::reader::DocumentReader;

use reader::stream::Stream;
use reader::result::ReadResult;
use reader::common::*;
use document::*;
use object::reference::*;
use document::map::*;


pub fn read_unfold_reference<T>(
  reader: &mut DocumentReader,
  reader_fn: &Fn(&mut DocumentReader) -> ReadResult<T>) -> ReadResult<T> {
  if is_reference_ahead(&mut reader.stream) {
    let reference = read_reference(&mut reader.stream)?;
    let original_position = reader.stream.position();
    let position = reader.map.get(&reference).unwrap().clone();

    let _ignored = read_object_id(&mut reader.stream, position)?;
    skip_whitespace(&mut reader.stream);
    
    let result = reader_fn(reader)?;
    reader.stream.set_position(original_position);

    Ok(result)
  } else {
    reader_fn(reader)
  }
}


pub fn is_reference_ahead(stream: &mut Stream) -> bool {
  let position = stream.position();

  skip_whitespace(stream);

  let is_ok =
    read_int(stream).is_ok() &&
    skip_space(stream).is_ok() &&
    read_int(stream).is_ok() &&
    skip(stream, " R").is_ok();

  stream.set_position(position);
  return is_ok
}

