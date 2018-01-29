use std;
use std::io::Read;
use std::io::Result;
use std::fs::File;

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