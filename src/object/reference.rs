use reader::stream::Stream;
use reader::result::ReadResult;
use object::Reference;
use reader::common::*;


fn read_reference(stream: &mut Stream) -> ReadResult<Reference> {
  let id = read_int(stream)?;
  skip(stream, " ")?;
  let generation = read_int(stream)?;
  skip(stream, " R")?;

  Ok(Reference { id, generation })
}