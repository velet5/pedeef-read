use std::io::Read;
use std::io::Result;
use std::fs::File;


pub struct Stream {
  /// current position in a stream
  position: usize,
  /// all bytes in a stream
  bytes: Vec<u8>,
  /// set to "true" to read backwards (but tokens will be inversed)
  pub simple: bool
}


impl Stream {
  
  pub fn from_file(file_name: &str) -> Result<Stream> {
    let bytes = bytes_from_file(file_name)?;
    let reader = Stream { position: 0, bytes, simple: true };

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

  pub fn set_forward_mode(&mut self) -> () {
    self.simple = true
  }
  
  pub fn set_backward_mode(&mut self) -> () {
    self.simple = false;
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
  let mut buffer = Vec::new();

  file.read_to_end(&mut buffer)?;

  Ok(buffer)
}