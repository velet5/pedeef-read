mod simple;
mod xref;
mod token;
mod object;

use xref::Xref;
use xref::XrefParser;
use object::*;
use simple::*;

// replace with full file name
const FILE_NAME: &str = "/home/oarshinskii/getting-real.pdf";


fn main() {

  match Reader::read_from_file(FILE_NAME) {
    Ok(_) => println!("ok"),
    Err(message) => println!("{}", message)
  }

}
