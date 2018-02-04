use std::collections::HashMap;

use reader::characters::*;
use reader::common::*;
use reader::stream::Stream;
use reader::error::ReadError;
use reader::result::ReadResult;
use object::Object;
use object::name::*;


pub fn read_dictionary(
  stream: &mut Stream,
  map: &HashMap<&str, &Fn(&mut Stream) -> ReadResult<Object>>) -> ReadResult<HashMap<String, Object>> {

  let mut buffer = HashMap::new();

  skip(stream, "<<")?;

  loop {
    match stream.peek() {
      SOLIDUS => {
        let name = read_name_string(stream)?;
        skip_whitespace(stream);
        match map.get(name.as_str()) {
          Some(parser) => {
            let object = parser(stream)?;
            buffer.insert(name, object);
          },
          None =>
            return Err(ReadError {
              message: format!("Unknown name in dictionary: {}. Position: {}", name, stream.position())
            })
        }
      },
      GREATER_THAN => break,
      other if is_whitespace(other) => skip_whitespace(stream),
      unknown => return Err(ReadError {
        message: format!("Unknown character in trailer: {}. Position: {}", unknown, stream.position())
      })
    }
  }

  Ok(buffer)
}


pub fn unfold<T>(
  name: &str,
  map: &mut HashMap<String, Object>,
  extractor: &Fn(Object) -> Option<T>) -> ReadResult<T> {

  let maybe_value = map.remove(name);

  match maybe_value {
    None =>
      return Err(ReadError {
        message: format!("Not found dictionary key {} while reading trailer.", name)
      }),
    _ => ()
  }

  let maybe_extracted = maybe_value.and_then(extractor);

  match maybe_extracted {
    Some(value) => Ok(value),
    None => return Err(ReadError {
      message: format!("Not found dictionary key {} while reading trailer.", name)
    })
  }
}
