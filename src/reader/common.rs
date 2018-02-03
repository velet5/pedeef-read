use reader::characters::*;
use reader::stream::Stream;
use reader::result::ReadResult;
use std::num::ParseIntError;
use reader::error::ReadError;


pub fn read_with_predicate(stream: &mut Stream, predicate: &Fn(u8) -> bool) -> String {
  let mut buffer = String::new();

  while predicate(stream.peek()) {
    let ch = read_char(stream);
    buffer.push(ch);
  }

  buffer
}


pub fn read_char(stream: &mut Stream) -> char {
  let byte = stream.next();
  char::from(byte)
}


pub fn read_regular_string(stream: &mut Stream) -> String {
  read_with_predicate(stream, &is_regular)
}


pub fn read_int(stream: &mut Stream) -> ReadResult<i32> {
  let string = read_with_predicate(stream, &is_digit);

  string
    .parse()
    .map_err(|err: ParseIntError|
      ReadError {
        message: format!("Error: {}. Position: {}", err.to_string(), stream.position())
      })
}


pub fn skip(stream: &mut Stream, what: &str) -> ReadResult<()> {
  for ch in what.chars() {
    let read = read_char(stream);
    if read != ch {
      return Err(ReadError {
        message: format!("Expected {}, got {}. Position: {}", ch, read, stream.position())
      })
    }
  }

  Ok(())
}


pub fn skip_whitespace(stream: &mut Stream) -> () {
  while is_whitespace(stream.peek()) {
    stream.move_cursor();
  }
}


pub fn skip_space(stream: &mut Stream) -> ReadResult<()> {
  skip(stream, " ")
}