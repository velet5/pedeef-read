use std;
use std::io::Read;
use std::io::Result;
use std::fs::File;
use std::fs::Metadata;


const NULL: u8 = 0;
const HORIZONTAL_TAB: u8 = 9;
const LINE_FEED: u8 = 10;
const FORM_FEED: u8 = 12;
const CARRIAGE_RETURN: u8 = 13;
const SPACE: u8 = 32;


pub type ReaderResult<T> = std::result::Result<T, String>;


pub struct SimpleReader {
  position: usize,
  bytes: Vec<u8>
}


pub struct BackwardReader {
  position: usize,
  bytes: Vec<u8>
}


fn bytes_from_file(file_name: &str) -> Result<Vec<u8>> {
  let mut file = File::open(file_name)?;
  let metadata = file.metadata()?;
  let size = metadata.len();
  let mut buffer = Vec::new();

  file.read_to_end(&mut buffer)?;

  Ok(buffer)
}


pub trait Reader: Sized {

  fn position(&self) -> usize;

  fn bytes(&self) -> &[u8];

  fn move_cursor(&mut self);

  fn process_string(&self, string: String) -> String;

  fn peek(&self) -> u8 {
    let bytes = self.bytes();
    let position = self.position();

    bytes[position]
  }

  fn next(&mut self) -> u8 {
    let value = self.peek();
    self.move_cursor();
    value
  }


  fn next_char(&mut self) -> ReaderResult<char> {
    let next_byte = self.next();
    let maybe_char = std::char::from_u32(next_byte as u32);

    match maybe_char {
      Some(ch) => Ok(ch),
      None => Err(format!("not char at position {}", self.position()))
    }
  }


  fn skip_whitespace(&mut self) -> () {
    while self.is_whitespace_ahead() {
      self.move_cursor()
    }
  }


  fn read_exact(&mut self, what: &str) -> ReaderResult<()> {
    let read = self.read_non_whitespace()?;
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

  
  fn read_non_whitespace(&mut self) -> ReaderResult<String> {
    let mut buffer = String::new();

    while !self.is_whitespace_ahead() {
      let ch = self.next_char()?;
      buffer.push(ch);
    }

    let string = self.process_string(buffer);

    Ok(string)
  }


  fn is_whitespace_ahead(&self) -> bool {
    let peek = self.peek();

    match peek {
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

}


impl SimpleReader {
  
  pub fn from_file(file_name: &str) -> Result<Self> {
    let bytes = bytes_from_file(file_name)?;
    let reader = SimpleReader { position: 0, bytes };

    Ok(reader)
  }

}


impl BackwardReader {

  pub fn from_file(file_name: &str) -> Result<Self> {
    let bytes = bytes_from_file(file_name)?;
    let reader = BackwardReader { position: bytes.len() - 1, bytes };

    Ok(reader)
  }

}


impl Reader for SimpleReader {

  fn position(&self) -> usize {
    self.position
  }

  fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  fn move_cursor(&mut self) {
    self.position += 1
  }
  fn process_string(&self, string: String) -> String {
    string
  }
}


impl Reader for BackwardReader {

  fn position(&self) -> usize {
    self.position
  }

  fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  fn move_cursor(&mut self) {
    self.position -= 1
  }

  fn process_string(&self, string: String) -> String {
    let mut buffer = String::new();

    for ch in string.chars().rev() {
      buffer.push(ch)
    }

    buffer
  }
}