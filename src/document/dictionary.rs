use std::any::Any;
use std::collections::HashMap;

use reader::characters::*;
use reader::common::*;
use reader::stream::Stream;
use reader::error::ReadError;
use reader::result::ReadResult;
use object::Object;
use object::name::*;

use document::reader::DocumentReader;


pub fn read_dictionary(
  reader: &mut DocumentReader,
  map: &HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>>) -> ReadResult<HashMap<String, Box<Any>>> {

  let mut buffer = HashMap::new();

  skip(&mut reader.stream, "<<")?;

  loop {
    match reader.stream.peek() {
      SOLIDUS => {
        let name = read_name_string(&mut reader.stream)?;

        println!("{}", name);

        skip_whitespace(&mut reader.stream);
        match map.get(name.as_str()) {
          Some(parser) => {
            let object = parser(reader)?;
            buffer.insert(name, object);
          },
          None =>
            return Err(ReadError {
              message: format!("Unknown name in dictionary: {}. Position: {}", name, &reader.stream.position())
            })
        }
      },
      GREATER_THAN => break,
      other if is_whitespace(other) => skip_whitespace(&mut reader.stream),
      unknown => return Err(ReadError {
        message: format!("Unknown character in trailer: {}. Position: {}", unknown, &reader.stream.position())
      })
    }
  }

  Ok(buffer)
}


pub fn unfold<T: 'static>(name: &str, map: &mut HashMap<String, Box<Any>>) -> ReadResult<Box<T>> {
  let maybe_value = map.remove(name);

  match maybe_value {
    None =>
      return Err(ReadError {
        message: format!("Not found dictionary key {} while reading trailer.", name)
      }),
    Some(value) => {
      value
        .downcast::<T>()
        .map_err(|_| ReadError {
          message: format!("Wrong type for {}.", name)
        })
    }
  }
}


pub fn unfold_optional<T: 'static>(
  name: &str,
  map: &mut HashMap<String, Box<Any>>) -> ReadResult<Box<Option<T>>> {

  if map.contains_key(name) {
    Ok(Box::new(Some(*unfold(name, map)?)))
  } else {
    Ok(Box::new(None))
  }

}