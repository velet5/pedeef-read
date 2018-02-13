use reader::characters::*;
use reader::common::*;
use reader::stream::Stream;
use reader::result::ReadResult;


#[derive(Debug)]
pub struct Date {
  year: u16,
  month: Option<u8>,
  day: Option<u8>,
  hour: Option<u8>,
  minute: Option<u8>,
  second: Option<u8>,
  offset: Option<char>,
  offset_hour: Option<u8>,
  offset_minute: Option<u8>
}


pub fn read_date(stream: &mut Stream) -> ReadResult<Date> {
  skip(stream, "(D:")?;

  let year = read_four_digit_number(stream)?;
  let month = read_two_digit_number(stream)?;
  let day = read_two_digit_number(stream)?;
  let hour = read_two_digit_number(stream)?;
  let minute = read_two_digit_number(stream)?;
  let second = read_two_digit_number(stream)?;

  let offset =
    if stream.peek() != RIGHT_PARENTHESIS {
      Some(read_char(stream))
    } else {
      None
    };

  let offset_hour = read_two_digit_number(stream)?;

  if stream.peek() == APOSTROPHE {
    skip(stream, "'")?;
  }
  let offset_minute = read_two_digit_number(stream)?;

  if stream.peek() == APOSTROPHE {
    skip(stream, "'")?;
  }
  skip(stream, ")")?;

  Ok(Date {
    year,
    month,
    day,
    hour,
    minute,
    second,
    offset,
    offset_hour,
    offset_minute,
  })
}


fn read_four_digit_number(stream: &mut Stream) -> ReadResult<u16> {
  let mut buffer = String::new();
  buffer.push(read_char(stream));
  buffer.push(read_char(stream));
  buffer.push(read_char(stream));
  buffer.push(read_char(stream));

  let number = buffer.parse().unwrap();

  Ok(number)
}


fn read_two_digit_number(stream: &mut Stream) -> ReadResult<Option<u8>> {
  if is_digit(stream.peek()) {
    let mut buffer = String::new();
    buffer.push(read_char(stream));
    buffer.push(read_char(stream));

    let number = buffer.parse().unwrap();

    Ok(Some(number))
  } else {
    Ok(None)
  }
}