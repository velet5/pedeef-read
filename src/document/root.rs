use std::any::Any;
use std::collections::HashMap;

use reader::characters::*;
use reader::common::*;
use reader::error::ReadError;
use reader::stream::Stream;
use reader::result::ReadResult;

use document::map::*;
use document::boxed::*;
use document::dictionary::*;

use object::Reference;
use object::Object;
use object::name::*;
use object::reference::*;


#[derive(Debug)]
pub struct DocumentCatalog;


pub struct PageLabels;


pub fn read_root(
  stream: &mut Stream,
  reference: &Reference,
  object_map: &ObjectMap) -> ReadResult<DocumentCatalog> {
  stream.set_forward_mode();

  let offset = object_map.value.get(reference).unwrap();
  let _ignored = read_object_id(stream, *offset)?;
  
  skip_whitespace(stream);

  let mut map: HashMap<&str, &Fn(&mut Stream) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("Type", &read_name_boxed);
  map.insert("Version", &read_name_boxed);
  map.insert("PageLabels", &read_page_labels);
  map.insert("PageLayout", &read_name_boxed);
  map.insert("PageMode", &read_name_boxed);
  map.insert("Pages", &read_reference_boxed);

 let dictionary = read_dictionary(stream, &map)?;
  
  unimplemented!()
}


fn read_page_labels(stream: &mut Stream) -> ReadResult<Box<Any>> {
  skip_whitespace(stream);

  unimplemented!()
}