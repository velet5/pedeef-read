use std::any::Any;
use std::collections::HashMap;

use document::boxed::*;
use document::reader::DocumentReader;
use document::dictionary::*;
use document::referenced::*;

use object::name::*;
use object::integer::*;
use object::reference::*;

use reader::common::*;
use reader::characters::*;
use reader::error::ReadError;
use reader::stream::Stream;
use reader::result::ReadResult;

use object::Object;

#[derive(Debug)]
pub struct NumberTree {
  kids: Option<Vec<NumberTree>>,
  numbers: Option<HashMap<i32, Object>>,
  limits: Option<[i32; 2]>
}

                         
pub fn read_number_tree(reader: &mut DocumentReader) -> ReadResult<NumberTree> {
  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("Kids", &read_kids_boxed);
  map.insert("Nums", &read_numbers_boxed);
  map.insert("Limits", &read_limits_boxed);

  let dictionary = &mut read_dictionary(reader, &map)?;

  let limits = *unfold_optional("Limits", dictionary)?;
  let kids = *unfold_optional("Kids", dictionary)?;
  let numbers = *unfold_optional("Nums", dictionary)?;

  Ok(NumberTree { kids, numbers, limits })
}

fn read_kids(reader: &mut DocumentReader) -> ReadResult<Vec<NumberTree>> {
  unimplemented!()
}

fn read_kids_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_kids(reader))
}


fn read_numbers(reader: &mut DocumentReader) -> ReadResult<HashMap<i32, Object>> {
  skip(&mut reader.stream, "[")?;

  let mut buffer = HashMap::new();

  while is_digit(reader.stream.peek()) {
    let (number, object) = read_number(reader)?;
    buffer.insert(number, object);
  }

  skip(&mut reader.stream, "]")?;

  Ok(buffer)
}


fn read_number(reader: &mut DocumentReader) -> ReadResult<(i32, Object)> {
  let number = read_int(&mut reader.stream)?;
  skip_whitespace(&mut reader.stream);

  let object: Object =
    match reader.stream.peek() {
      SOLIDUS =>
        read_name(&mut reader.stream)?,
      other if is_digit(other) =>
        if is_reference_ahead(&mut reader.stream) {
          read_reference_object(&mut reader.stream)?
        } else {
          read_integer(&mut reader.stream)?
        },
      other =>
        Err(ReadError {message:
          format!(
            "Unknown number tree value starting: {}. Position: {}",
            reader.stream.peek(), reader.stream.position())})?
    };

  skip_whitespace(&mut reader.stream);

  Ok((number, object))
}


fn read_numbers_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_numbers(reader))
}


fn read_limits(reader: &mut DocumentReader) -> ReadResult<[i32; 2]> {
  unimplemented!()
}


fn read_limits_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_limits(reader))
}