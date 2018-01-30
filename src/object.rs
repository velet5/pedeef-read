use simple::*;
use token::*;
use xref::*;

use std::collections::HashMap;


const LESS_THAN: u8 = '<' as u8;
const F: u8 = 'f' as u8;
const N: u8 = 'n' as u8;
const T: u8 = 't' as u8;


#[derive(Debug)]
pub struct IndirectObject(pub i32, pub i32, pub Object);


#[derive(Debug)]
pub enum Object {
  Null,
  Boolean(bool),
  Real(f32),
  Integer(i32),
  String(String),
  Dictionary(HashMap<String, Object>),
  Reference(i32, i32)
}


pub trait ToObject {
  fn to_object(&self) -> Object;
}


impl Reader {

  pub fn read_from_file(file_name: &str) -> ReaderResult<Vec<IndirectObject>> {
    let mut parser = Reader::get_parser(file_name)?;
    let xref = parser.parse()?;

    let mut reader = parser.reader;

    let trailer = reader.read_trailer()?;

    println!("trailer: {:?}", trailer);

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

    let object = self.read_object()?;

    Ok(IndirectObject(id, generation, object))
  }


  fn read_object(&mut self) -> ReaderResult<Object> {
    let peek = self.peek();

    match peek {
      F => self.read_false(),
      N => self.read_null(),
      T => self.read_true(),
      other if ZERO <= other && other <= NINE => self.read_integer(),
      _ => self.default(),
    }
  }


  fn read_integer(&mut self) -> ReaderResult<Object> {
    let value = self.read_int()?;

    Ok(Object::Integer(value))
  }


  fn read_null(&mut self) -> ReaderResult<Object> {
    self.read_exact("null")?;
    self.skip_whitespace();

    Ok(Object::Null)
  }


  fn read_true(&mut self) -> ReaderResult<Object> {
    self.read_exact("true")?;
    self.skip_whitespace();

    Ok(Object::Boolean(true))
  }


  fn read_false(&mut self) -> ReaderResult<Object> {
    self.read_exact("false")?;
    self.skip_whitespace();

    Ok(Object::Boolean(false))
  }


  fn read_reference_object(&mut self) -> ReaderResult<Object> {
    let reference = self.read_reference()?;

    Ok(reference.to_object())
  }
  

  fn default(&mut self) -> ReaderResult<Object> {
    Ok(Object::Integer(777))
  }

}