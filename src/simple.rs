use std;
use std::io::Read;
use std::io::Result;
use std::fs::File;

pub type ReaderResult<T> = std::result::Result<T, String>;


pub struct Reader {
  position: usize,
  bytes: Vec<u8>,
  // set to "true" to read backwards (but tokens will be inversed)
  pub simple: bool
}


impl Reader {
  
  pub fn from_file(file_name: &str) -> Result<Self> {
    let bytes = bytes_from_file(file_name)?;
    let reader = Reader { position: 0, bytes, simple: true };

    Ok(reader)
  }

  pub fn position(&self) -> usize {
    self.position
  }

  fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  pub fn set_position(&mut self, position: usize) -> () {
    self.position = position
  }


  pub fn set_simple(&mut self, simple: bool) -> () {
    self.simple = simple
  }


  pub fn set_position_to_end(&mut self) -> () {
    let position = self.bytes.len() - 1;
    self.set_position(position);
  }


  pub fn move_cursor(&mut self) {
    if self.simple {
      self.position += 1;
    } else {
      self.position -= 1;
    }
  }

  pub fn peek(&self) -> u8 {
    let bytes = self.bytes();
    let position = self.position();

    bytes[position]
  }

  pub fn next(&mut self) -> u8 {
    let value = self.peek();
    self.move_cursor();
    value
  }

}

fn bytes_from_file(file_name: &str) -> Result<Vec<u8>> {
  let mut file = File::open(file_name)?;
  let metadata = file.metadata()?;
  let size = metadata.len();
  let mut buffer = Vec::new();

  file.read_to_end(&mut buffer)?;

  Ok(buffer)
}