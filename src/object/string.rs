use object::Object;
use reader::result::ReadResult;
use reader::error::ReadError;
use reader::stream::Stream;
use reader::common::*;
use reader::characters::*;


pub fn read_pdf_string(stream: &mut Stream) -> ReadResult<String> {
  skip_whitespace(stream);

  match stream.peek() {
    LEFT_PARENTHESIS =>
      read_as_is_string(stream),
    LESS_THAN =>
      read_byte_string(stream),
    other =>
      Err(ReadError{ message :
        format!("Unknown string beginning: {}. Position: {}", char::from(other), stream.position())
      })
  }
}


fn read_as_is_string(stream: &mut Stream) -> ReadResult<String> {
  let mut buffer = String::new();

  skip(stream, "(")?;

  let mut open_paren_count = 0;

  loop {
    match stream.next() {
      BACKSLASH =>
        buffer.push(read_char(stream)),
      LEFT_PARENTHESIS => {
        open_paren_count += 1;
        buffer.push(char::from(LEFT_PARENTHESIS));
      },
      RIGHT_PARENTHESIS if open_paren_count > 0 => {
        open_paren_count -= 1;
        buffer.push(char::from(RIGHT_PARENTHESIS));
      },
      RIGHT_PARENTHESIS =>
        break,
      other =>
        buffer.push(char::from(other))
    }
  }

  Ok(buffer)
}


pub fn read_byte_string(stream: &mut Stream) -> ReadResult<String> {
  skip(stream, "<")?;
  let value = read_regular_string(stream);
  skip(stream, ">")?;

  Ok(value)
}


pub fn read_byte_string_object(stream: &mut Stream) -> ReadResult<Object> {
  let string = read_byte_string(stream)?;

  Ok(Object::String(string))
}
