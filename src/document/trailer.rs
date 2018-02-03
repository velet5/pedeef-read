use std::collections::HashMap;

use object::Reference;
use object::Object;
use object::reference::*;
use object::name::*;
use object::integer::*;
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
  trailer_map.insert("Root", &read_reference_object);
  trailer_map.insert("Info", &read_reference_object);

  skip(stream, "trailer")?;
  skip_whitespace(stream);

  let dictionary = read_dictionary(stream, &trailer_map)?;

  println!("{:?}", dictionary);

  unimplemented!()
}
