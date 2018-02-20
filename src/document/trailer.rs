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
use document::reader::DocumentReader;


#[derive(Debug)]
pub struct Trailer {
  pub size: i32,
  pub root: Reference,
  pub info: Reference,
  pub checksum: Option<String>
}


pub fn read_trailer(reader: &mut DocumentReader)  -> ReadResult<Trailer> {

  let mut trailer_map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  trailer_map.insert("Size", &read_int_boxed);
  trailer_map.insert("Prev", &read_int_boxed);
  trailer_map.insert("Root", &read_reference_boxed);
  trailer_map.insert("Info", &read_reference_boxed);
  trailer_map.insert("ID", &read_id_array);
  trailer_map.insert("DocChecksum", &read_name_boxed);

  {
    let stream = &mut reader.stream;
    stream.set_forward_mode();
    skip(stream, "trailer")?;
    skip_whitespace(stream);
  }

  let dictionary = &mut read_dictionary(reader, &trailer_map)?;

  let size = *unfold("Size", dictionary)?;
  let root = *unfold("Root", dictionary)?;
  let info = *unfold("Info", dictionary)?;
  let checksum = *unfold_optional("DocChecksum", dictionary)?;

  Ok(Trailer { size, root, info, checksum })
}

                                                                                   
fn read_id_array(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  let stream = &mut reader.stream;

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