
pub const LEFT_PARENTHESIS: u8 = '(' as u8;
pub const RIGHT_PARENTHESIS: u8 = ')' as u8;
pub const LESS_THAN: u8 = '<' as u8;
pub const GREATER_THAN: u8 = '>' as u8;
pub const LEFT_SQUARE_BRACKET: u8 = '[' as u8;
pub const RIGHT_SQUARE_BRACKET: u8 = ']' as u8;
pub const LEFT_CURLY_BRACKET: u8 = '{' as u8;
pub const RIGHT_CURLY_BRACKET: u8 = '}' as u8;
pub const SOLIDUS: u8 = '/' as u8;
pub const PERCENT: u8 = '%' as u8;

pub const NULL: u8 = 0;
pub const HORIZONTAL_TAB: u8 = 9;
pub const LINE_FEED: u8 = 10;
pub const FORM_FEED: u8 = 12;
pub const CARRIAGE_RETURN: u8 = 13;
pub const SPACE: u8 = 32;
pub const ZERO: u8 = '0' as u8;
pub const NINE: u8 = '9' as u8;


pub fn is_delimiter(byte: u8) -> bool {
  match byte {
    LEFT_PARENTHESIS | RIGHT_PARENTHESIS |
    LESS_THAN | GREATER_THAN |
    LEFT_SQUARE_BRACKET | RIGHT_SQUARE_BRACKET |
    LEFT_CURLY_BRACKET | RIGHT_CURLY_BRACKET |
    SOLIDUS | PERCENT =>
      true,
    _ =>
      false
  }
}


pub fn is_whitespace(byte: u8) -> bool {
  match byte {
    NULL | HORIZONTAL_TAB | SPACE |
    LINE_FEED | FORM_FEED | CARRIAGE_RETURN =>
      true,
    _ =>
      false
  }
}


pub fn is_digit(byte: u8) -> bool {
  ZERO <= byte && byte <= NINE
}


pub fn is_regular(byte: u8) -> bool {
  !is_whitespace(byte) && !is_delimiter(byte)
}