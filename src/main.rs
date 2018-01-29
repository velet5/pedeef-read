mod simple;
mod xref;
mod token;

use xref::Xref;
use xref::XrefParser;

// replace with full file name
const FILE_NAME: &str = "/home/oarshinskii/getting-real.pdf";


fn main() {

  match read_xref() {
    Ok(_) => println!("ok"),
    Err(message) => println!("{}", message)
  }

}


fn read_xref() -> Result<Xref, String> {
  let mut reader_or_error = XrefParser::from_file(FILE_NAME);

  match reader_or_error {
    Ok(mut reader) => reader.parse(),
    Err(err) => Err(err.to_string())
  }
}