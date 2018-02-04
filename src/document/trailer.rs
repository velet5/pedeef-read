use std::any::Any;
use std::collections::HashMap;

use object::Reference;
use object::Object;
use object::reference::*;
use object::integer::*;
use object::string::*;
use reader::stream::Stream;
use reader::result::ReadResult;
use reader::common::*;
use document::dictionary::*;
use document::boxed::*;


#[derive(Debug)]
pub struct Trailer {
  pub size: i32,
  pub root: Reference,
  pub info: Reference
}


pub fn read_trailer(stream: &mut Stream)  -> ReadResult<Trailer> {
  let mut trailer_map: HashMap<&str, &Fn(&mut Stream) -> ReadResult<Box<Any>>> = HashMap::new();

  trailer_map.insert("Size", &read_int_boxed);
  trailer_map.insert("Prev", &read_int_boxed);
  trailer_map.insert("Root", &read_reference_boxed);
  trailer_map.insert("Info", &read_reference_boxed);
  trailer_map.insert("ID", &read_id_array);

  skip(stream, "trailer")?;
  skip_whitespace(stream);

  let dictionary = &mut read_dictionary(stream, &trailer_map)?;

  let size = unfold::<i32>("Size", dictionary)?;
  let root = unfold::<Reference>("Root", dictionary)?;
  let info = unfold::<Reference>("Info", dictionary)?;

  Ok(Trailer {
    size: *size,
    root: *root,
    info: *info
  })
}

                                                                                   
fn read_id_array(stream: &mut Stream) -> ReadResult<Box<Any>> {
  skip(stream, "[")?;
  skip_whitespace(stream);

  let first = read_byte_string_object(stream)?;
  skip_whitespace(stream);

  let second = read_byte_string_object(stream)?;
  skip_whitespace(stream);

  skip(stream, "]")?;
  skip_whitespace(stream);

  Ok(Box::new([first, second]))
}