use std::num::ParseIntError;

use reader::stream::Stream;
use reader::result::ReadResult;
use reader::error::ReadError;
use reader::common::*;
use reader::characters::*;
use std::iter::Map;


#[derive(Debug)]
pub struct XrefEntry {
  offset: usize,
  generation: i32,
  is_used: bool
}


#[derive(Debug)]
pub struct XrefTable {
  version: i32,
  entries: Vec<XrefEntry>
}


pub fn read_xref(stream: &mut Stream) -> ReadResult<XrefTable> {
  let offset = read_xref_offset(stream)?;

  stream.set_position(offset);
  stream.set_forward_mode();

  skip(stream, "xref")?;
  skip_whitespace(stream);

  let version = read_int(stream)?;
  skip(stream, " ")?;

  let number = read_int(stream)?;
  skip_whitespace(stream);

  let mut entries = Vec::new();

  for _ in 0 .. number {
    let entry = read_entry(stream)?;
    entries.push(entry)
  }

  Ok(XrefTable { version, entries })
}


fn read_entry(stream: &mut Stream) -> ReadResult<XrefEntry> {
  let offset = read_int(stream)? as usize;
  skip_space(stream)?;

  let generation = read_int(stream)?;
  skip_space(stream)?;

  let status = read_char(stream);
  let is_used = status == 'n';
  skip_whitespace(stream);

  Ok(XrefEntry { offset, generation, is_used })
}


fn read_xref_offset(stream: &mut Stream) -> ReadResult<usize> {
  stream.set_position_to_end();
  stream.set_backward_mode();
  
  skip_whitespace(stream);
  skip(stream, "FOE%%")?;
  skip_whitespace(stream);

  let string: String = read_with_predicate(stream, &is_digit);
  let reversed: String = string.chars().rev().collect();

  reversed
    .parse()
    .map_err(|err: ParseIntError| ReadError { message: err.to_string() })
}


