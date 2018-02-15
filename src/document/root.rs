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
use document::reader::*;
use document::referenced::*;
use document::number_tree::*;
use document::viewer_preferences::*;

use object::Reference;
use object::Object;
use object::name::*;
use object::reference::*;


#[derive(Debug)]
pub struct DocumentCatalog {
  tpe: String,
  version: Option<String>,
  page_labels: Option<NumberTree>,
  page_layout: Option<String>,
  page_mode: Option<String>,
  pub pages: Reference,
  viewer_preferences: Option<ViewerPreferences>
}


pub fn read_root(reader: &mut DocumentReader, reference: &Reference) -> ReadResult<DocumentCatalog> {
  {
    let stream = &mut reader.stream;
    stream.set_forward_mode();
    let offset = reader.map.value.get(reference).unwrap();
    let _ignored = read_object_id(stream, *offset)?;
    skip_whitespace(stream);
  }

  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("Type", &read_name_boxed);
  map.insert("Version", &read_name_boxed);
  map.insert("PageLabels", &read_page_labels);
  map.insert("PageLayout", &read_name_boxed);
  map.insert("PageMode", &read_name_boxed);
  map.insert("Pages", &read_reference_boxed);
  map.insert("ViewerPreferences", &read_viewer_preferences_boxed);

  let dictionary = &mut read_dictionary(reader, &map)?;

  let tpe = *unfold("Type", dictionary)?;
  let version = *unfold_optional("Version", dictionary)?;
  let page_labels = *unfold_optional("PageLabels", dictionary)?;
  let page_layout = *unfold_optional("PageLayout", dictionary)?;
  let page_mode = *unfold_optional("PageMode", dictionary)?;
  let pages = *unfold("Pages", dictionary)?;
  let viewer_preferences = *unfold_optional("ViewerPreferences", dictionary)?;

  Ok(DocumentCatalog {
    tpe,
    version,
    page_labels,
    page_layout,
    page_mode,
    pages,
    viewer_preferences
  })
}


fn read_page_labels(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  skip_whitespace(&mut reader.stream);

  let number_tree = read_unfold_reference(reader, &read_number_tree)?;

  Ok(Box::new(number_tree))
}