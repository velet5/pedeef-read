use std;

use reader::BackwardReader;
use reader::Reader;
use reader::ReaderResult;

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
    self.reader.skip_whitespace();
    self.reader.read_exact("%%EOF")?;

    Ok(Xref {})
  }

}