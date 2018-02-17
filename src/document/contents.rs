use reader::common::*;
use reader::result::ReadResult;

use object::Reference;

use document::reader::DocumentReader;
use document::map::*;

pub struct Contents {

}


struct ContentInfo {
  filter: String,
  length: i32
}



pub fn read_content(reader: &mut DocumentReader, reference: &Reference) -> ReadResult<Contents> {
  let position = reader.map.get(reference).unwrap().clone();
  let _ignored = read_object_id(&mut reader.stream, position)?;

  skip_whitespace(&mut reader.stream);

  let info = read_content_info(reader)?;
  
  unimplemented!()
}



fn read_content_info(reader: &mut DocumentReader) -> ReadResult<ContentInfo> {
  unimplemented!()
}
