use simple::*;
use token::*;
use xref::*;

use std::collections::HashMap;


pub struct IndirectObject(i32, i32, Object);

pub enum Object {
  Null,
  Boolean(bool),
  Real(f32),
  Integer(i32),
  String(String),
  Dictionary(HashMap<String, Object>)
}


impl Reader {

  pub fn read_from_file(file_name: &str) -> ReaderResult<Vec<IndirectObject>> {
    let mut parser = Reader::get_parser(file_name)?;
    let xref = parser.parse()?;

    let mut reader = parser.reader;

    reader.read_objects(xref)
  }


  fn get_parser(file_name: &str) -> ReaderResult<XrefParser> {
    let mut parser_or_error = XrefParser::from_file(file_name);

    match parser_or_error {
      Ok(mut parser) => Ok(parser),
      Err(err) => Err(err.to_string())
    }
  }


  pub fn read_objects(&mut self, xref: Xref) -> ReaderResult<Vec<IndirectObject>> {
    let mut buffer = Vec::new();

    for entry in xref.entries {
      if entry.is_used {
        self.set_simple(true);
        self.set_position(entry.offset);

        let object = self.read_indirect_object()?;
        buffer.push(object);
      }
    }

    Ok(buffer)
  }


  pub fn read_indirect_object(&mut self) -> ReaderResult<IndirectObject> {
    let id = self.read_int()?;
    self.skip_whitespace();

    let generation = self.read_int()?;
    self.skip_whitespace();

    self.read_exact("obj")?;
    self.skip_whitespace();

    unimplemented!();
  }

}