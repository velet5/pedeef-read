use std;

use simple::Reader;
use simple::ReaderResult;


pub struct XrefParser {
  pub reader: Reader
}


#[derive(Debug)]
pub struct XrefEntry {
  pub offset: usize,
  pub generation: i32,
  pub is_used: bool
}


pub struct Xref {
  pub entries: Vec<XrefEntry>
}


impl XrefParser {

  pub fn from_file(file_name: &str) -> std::io::Result<XrefParser> {
    let mut reader = Reader::from_file(file_name)?;
    Ok(XrefParser { reader })
  }


  pub fn parse(&mut self) -> ReaderResult<Xref> {
    let position = self.parse_position()? as u32;
    let entries = self.parse_table(position)?;
    Ok(Xref { entries })
  }


  fn parse_table(&mut self, position: u32) -> ReaderResult<Vec<XrefEntry>> {
    self.reader.set_simple(true);
    self.reader.set_position(position as usize);
        
    self.reader.read_exact("xref")?;
    self.reader.skip_whitespace();

    let number = self.reader.read_int()?;
    self.reader.skip_whitespace();
    
    let quantity = self.reader.read_int()?;
    self.reader.skip_whitespace();

    let mut entries = Vec::new();

    for _ in 0 .. quantity {
      let entry = self.parse_entry()?;
      entries.push(entry);
    }

    Ok(entries)
  }


  fn parse_entry(&mut self) -> ReaderResult<XrefEntry> {
    let offset = self.reader.read_int()?;
    self.reader.skip_whitespace();
    let generation = self.reader.read_int()?;
    self.reader.skip_whitespace();
    let mark = self.reader.next_char()?;
    self.reader.skip_whitespace();

    Ok(XrefEntry {
      offset: offset as usize,
      generation,
      is_used: mark == 'n'
    })
  }


  fn parse_position(&mut self) -> ReaderResult<i32> {
    self.reader.set_simple(false);
    self.reader.set_position_to_end();

    let mut reader = &mut self.reader;
    reader.skip_whitespace();
    reader.read_exact("%%EOF")?;
    reader.skip_whitespace();
    let position = reader.read_int()?;
    reader.skip_whitespace();
    reader.read_exact("startxref")?;

    Ok(position)
  }

}