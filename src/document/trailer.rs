use std::collections::HashMap;

use object::Reference;
use object::Object;
use object::reference::*;
use object::name::*;
use object::integer::*;
use object::string::*;
use reader::stream::Stream;
use reader::result::ReadResult;
use reader::error::ReadError;
use reader::common::*;
use reader::characters::*;
use document::dictionary::*;


#[derive(Debug)]
pub struct Trailer {
  pub size: i32,
  pub root: Reference,
  pub info: Reference
}


pub fn read_trailer(stream: &mut Stream)  -> ReadResult<Trailer> {
  let mut trailer_map: HashMap<&str, &Fn(&mut Stream) -> ReadResult<Object>> = HashMap::new();

  trailer_map.insert("Size", &read_integer);
  trailer_map.insert("Prev", &read_integer);
  trailer_map.insert("Root", &read_reference_object);
  trailer_map.insert("Info", &read_reference_object);
  trailer_map.insert("ID", &read_id_array);

  skip(stream, "trailer")?;
  skip_whitespace(stream);

  let dictionary = &mut read_dictionary(stream, &trailer_map)?;
  
  Ok(Trailer {
    size: unfold("Size", dictionary, &to_int)?,
    root: unfold("Root", dictionary, &to_reference)?,
    info: unfold("Info", dictionary, &to_reference)?
  })
}


fn read_id_array(stream: &mut Stream) -> ReadResult<Object> {
  skip(stream, "[")?;
  skip_whitespace(stream);

  let first = read_byte_string_object(stream)?;
  skip_whitespace(stream);

  let second = read_byte_string_object(stream)?;
  skip_whitespace(stream);

  skip(stream, "]");
  skip_whitespace(stream);

  Ok(Object::Array(vec!(first, second)))
}