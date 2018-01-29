use std;

use simple::*;

const NULL: u8 = 0;
const HORIZONTAL_TAB: u8 = 9;
const LINE_FEED: u8 = 10;
const FORM_FEED: u8 = 12;
const CARRIAGE_RETURN: u8 = 13;
const SPACE: u8 = 32;
const ZERO: u8 = '0' as u8;
const NINE: u8 = '9' as u8;


fn is_whitespace(byte: u8) -> bool {
  match byte {
    NULL |
    HORIZONTAL_TAB |
    LINE_FEED |
    FORM_FEED |
    CARRIAGE_RETURN |
    SPACE =>
      true,
    _ =>
      false
  }
}


fn non_whitespace(byte: u8) -> bool {
  !is_whitespace(byte)
}


fn is_digit(byte: u8) -> bool {
  ZERO <= byte && byte <= NINE
}


fn reverse_string(string: String) -> String {
  let mut buffer = String::new();

  for ch in string.chars().rev() {
    buffer.push(ch)
  }

  buffer

}



impl Reader {
  pub fn next_char(&mut self) -> ReaderResult<char> {
    let next_byte = self.next();
    let maybe_char = std::char::from_u32(next_byte as u32);

    match maybe_char {
      Some(ch) => Ok(ch),
      None => Err(format!("not char at position {}", self.position()))
    }
  }


  fn process_string(&self, string: String) -> String {
    if self.simple {
      string
    } else {
      reverse_string(string)
    }
  }


  pub fn skip_whitespace(&mut self) -> () {
    while is_whitespace(self.peek()) {
      self.move_cursor()
    }
  }


  pub fn read_exact(&mut self, what: &str) -> ReaderResult<()> {
    let mut buffer = String::new();

    for _ in 0 .. what.len() {
      let ch = self.next_char()?;
      buffer.push(ch);
    }

    let read = self.process_string(buffer);

    let what_chars = what.chars();
    let read_chars = read.chars();

    let is_same_length = read.len() == what.len();

    let is_same_content =
      what_chars
        .zip(read_chars)
        .all(|(a, b)| a == b);

    let is_same = is_same_length && is_same_content;

    if is_same {
      Ok(())
    } else {
      Err(format!("Expected {}, got {}. Position: {}", what, read, self.position()))
    }
  }


  fn read_with_predicate(&mut self, predicate: &Fn(u8) -> bool) -> ReaderResult<String> {
    let mut buffer = String::new();

    while predicate(self.peek()) {
      let ch = self.next_char()?;
      buffer.push(ch);
    }

    let string = self.process_string(buffer);

    Ok(string)
  }


  pub fn read_non_whitespace(&mut self) -> ReaderResult<String> {
    self.read_with_predicate(&non_whitespace)
  }


  pub fn read_int(&mut self) -> ReaderResult<i32> {
    let string = self.read_with_predicate(&is_digit)?;
    match string.parse() {
      Ok(integer) => Ok(integer),
      Err(err) => Err(format!("Error reading number: {}, at position {}", string, self.position()))
    }
  }
}
