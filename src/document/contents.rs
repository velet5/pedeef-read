use std::fmt::Debug;
use std::any::Any;
use std::collections::HashMap;

use reader::common::*;
use reader::result::ReadResult;

use object::Reference;
use object::name::*;

use document::reader::DocumentReader;
use document::map::*;
use document::dictionary::*;
use document::boxed::*;
use document::referenced::*;


#[derive(Debug)]
pub struct Contents {
  filter: Filter,
  bytes: Vec<u8>
}


struct ContentInfo {
  filter: String,
  length: i32
}

#[derive(Debug)]
pub enum Filter {
  FlateDecode
}



pub fn read_content(reader: &mut DocumentReader, reference: &Reference) -> ReadResult<Contents> {
  let position = reader.map.get(reference).unwrap().clone();
  let _ignored = read_object_id(&mut reader.stream, position)?;

  skip_whitespace(&mut reader.stream);

  let info = read_content_info(reader)?;

  let filter =
    if info.filter == "FlateDecode" {
      Filter::FlateDecode
    } else {
      unimplemented!()
    };

  let bytes = read_bytes(reader, info.length)?;

  Ok(Contents {filter, bytes})
}



fn read_content_info(reader: &mut DocumentReader) -> ReadResult<ContentInfo> {
  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("Length", &read_length_boxed);
  map.insert("Filter", &read_name_boxed);

  let dictionary = &mut read_dictionary(reader, &map)?;

  let length: i32 = *unfold("Length", dictionary)?;
  let filter_name: String = *unfold("Filter", dictionary)?;

  Ok(ContentInfo { length, filter: filter_name })
}


fn read_bytes(reader: &mut DocumentReader, length: i32) -> ReadResult<Vec<u8>> {
  let stream = &mut reader.stream;
  skip_whitespace(stream);
  skip(stream, "stream")?;
  let _ignored = stream.next();

  let mut buffer = Vec::with_capacity(length as usize);

  for _ in 0 .. length {
    buffer.push(stream.next());
  }

  Ok(buffer)
}


fn read_length_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_length(reader))
}


fn read_length(reader: &mut DocumentReader) -> ReadResult<i32> {
  read_unfold_reference(reader, &|reader| read_int(&mut reader.stream))
}
