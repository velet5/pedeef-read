
#[derive(Debug)]
pub struct ReadError {
  pub message: String
}


impl From<String> for ReadError {
  fn from(string: String) -> Self {
    ReadError { message: string }
  }
}