use std::any::Any;

use reader::common::*;
use reader::stream::Stream;
use reader::result::ReadResult;
use object::reference::*;
use object::name::*;


fn boxed<T: 'static>(result: ReadResult<T>) -> ReadResult<Box<Any>> {
  result.map(|value| Box::new(value) as Box<Any>)
}


pub fn read_int_boxed(stream: &mut Stream) -> ReadResult<Box<Any>> {
  boxed(read_int(stream))
}


pub fn read_reference_boxed(stream: &mut Stream) -> ReadResult<Box<Any>> {
  boxed(read_reference(stream))
}


pub fn read_name_boxed(stream: &mut Stream) -> ReadResult<Box<Any>> {
  boxed(read_name_string(stream))
}