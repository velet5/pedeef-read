use std::any::Any;

use reader::common::*;
use reader::stream::Stream;
use reader::result::ReadResult;
use object::reference::*;
use object::name::*;

use document::reader::DocumentReader;


pub fn boxed<T: 'static>(result: ReadResult<T>) -> ReadResult<Box<Any>> {
  result.map(|value| Box::new(value) as Box<Any>)
}


pub fn read_int_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_int(&mut reader.stream))
}


pub fn read_bool_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_bool(&mut reader.stream))
}


pub fn read_reference_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_reference(&mut reader.stream))
}


pub fn read_name_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_name_string(&mut reader.stream))
}