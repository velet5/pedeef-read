use std::any::Any;
use std::collections::HashMap;

use document::reader::DocumentReader;
use document::boxed::*;
use document::dictionary::*;
use document::map::*;

use object::Reference;
use object::date::Date;

use reader::common::*;
use reader::result::ReadResult;


#[derive(Debug)]
pub struct Info {
  title: Option<String>,
  author: Option<String>,
  subject: Option<String>,
  keywords: Option<String>,
  creator: Option<String>,
  producer: Option<String>,
  creation_date: Option<Date>,
  mod_date: Option<Date>,
  trapped: Option<String>
}


pub fn read_info(reader: &mut DocumentReader, reference: &Reference) -> ReadResult<Info> {
  let position = reader.map.get(reference).unwrap().clone();
  let _ignored = read_object_id(&mut reader.stream, position)?;
  skip_whitespace(&mut reader.stream);

  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  map.insert("Title", &read_string_boxed);
  map.insert("Author", &read_string_boxed);
  map.insert("Subject", &read_string_boxed);
  map.insert("Keywords", &read_string_boxed);
  map.insert("Creator", &read_string_boxed);
  map.insert("Producer", &read_string_boxed);
  map.insert("CreationDate", &read_date_boxed);
  map.insert("ModDate", &read_date_boxed);
  map.insert("Trapped", &read_name_boxed);

  let dictionary = &mut read_dictionary(reader, &map)?;

  let title = *unfold_optional("Title", dictionary)?;
  let author = *unfold_optional("Author", dictionary)?;
  let subject = *unfold_optional("Subject", dictionary)?;
  let keywords = *unfold_optional("Keywords", dictionary)?;
  let creator = *unfold_optional("Creator", dictionary)?;
  let producer = *unfold_optional("Producer", dictionary)?;
  let creation_date = *unfold_optional("CreationDate", dictionary)?;
  let mod_date = *unfold_optional("ModDate", dictionary)?;
  let trapped = *unfold_optional("Trapped", dictionary)?;

  Ok(Info { title, author, subject, keywords, creator, producer, creation_date, mod_date, trapped })
}

