use std;

use simple::BackwardReader;
use simple::Reader;
use token::TokenReader;
use simple::ReaderResult;


pub struct XrefParser {
  reader: BackwardReader
}


pub struct Xref;


impl XrefParser {

  pub fn from_file(file_name: &str) -> std::io::Result<XrefParser> {
    let mut reader = BackwardReader::from_file(file_name)?;
    Ok(XrefParser { reader })
  }


  pub fn parse(&mut self) -> ReaderResult<Xref> {
    let mut reader = &mut self.reader;
    reader.skip_whitespace();
    reader.read_exact("%%EOF")?;
    reader.skip_whitespace();

    let position = reader.read_int()?;

    println!("{}", position);

    Ok(Xref {})
  }

}