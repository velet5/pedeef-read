use simple::*;
use object::*;
use token::*;

pub const SOLIDUS: u8 = '/' as u8;
pub const EXCLAMATION: u8 = '!' as u8;
pub const TILDE: u8 = '~' as u8;
pub const BRACKET: u8 = '[' as u8;
pub const A: u8 = 'a' as u8;
pub const F: u8 = 'f' as u8;


#[derive(Debug)]
pub struct Reference(i32, i32);


#[derive(Debug)]
pub struct Trailer {
  root: Reference,
  info: Option<Reference>
}


impl ToObject for Reference {
  
  fn to_object(&self) -> Object {
    Object::Reference(self.0, self.1)
  }

}


fn is_correct_name_symbol(byte: u8) -> bool {
  EXCLAMATION <= byte && byte <= TILDE && byte != BRACKET && byte != SOLIDUS
}


impl Reader {

  pub fn read_trailer(&mut self) -> ReaderResult<Trailer> {
    self.read_exact("trailer")?;
    self.skip_whitespace();
    self.read_exact("<<")?;
    self.skip_whitespace();

    let mut maybe_root: Option<Reference> = None;
    let mut maybe_info: Option<Reference> = None;

    while self.peek() == SOLIDUS {
      let key = self.read_name()?;
      match key.as_str() {
        "Size" => {
          self.read_exact(" ")?;
          let _ignored = self.read_int()?;
        }
        "Root" => {
          self.read_exact(" ")?;
          let reference = self.read_reference()?;
          maybe_root = Some(reference);
        },
        "Info" => {
          self.read_exact(" ")?;
          let reference = self.read_reference()?;
          maybe_info = Some(reference);
        },
        "ID" => {
          self.read_id()?;
        },
        "Prev" => {
          self.read_int()?;
        },
        "Encrypt" => {
          unimplemented!()
        }
        other =>
          Err(format!("Trailer. Unknown key: {}. Position: {}", other, self.position()))?
      }
      self.skip_whitespace();
    }

    let some_or_error = |option| {
      match option {
        Some(value) => Ok(value),
        None => Err(format!("Cannot read trailer: root, info, or size. Position: {}", self.position()))
      }
    };

    let root = some_or_error(maybe_root)?;

    Ok(Trailer { root, info: maybe_info })
  }


  pub fn read_name(&mut self) -> ReaderResult<String> {
    self.read_exact("/")?;
    self.read_with_predicate(&is_correct_name_symbol)
  }


  pub fn read_id(&mut self) -> ReaderResult<()> {
    self.skip_whitespace();
    self.read_exact("[")?;
    self.skip_whitespace();
    self.read_hex_string()?;
    self.read_hex_string()?;
    self.skip_whitespace();
    self.read_exact("]")?;

    Ok(())
  }

  pub fn read_hex_string(&mut self) -> ReaderResult<String> {
    fn is_hexadecimal(byte: u8) -> bool {
      (ZERO <= byte && byte <= NINE) ||
      (A <= byte && byte <= F)
    }

    self.read_exact("<")?;
    let value = self.read_with_predicate(&is_hexadecimal)?;
    self.read_exact(">")?;
    self.skip_whitespace();

    Ok(value)
  }


  pub fn read_reference(&mut self) -> ReaderResult<Reference> {
    let id = self.read_int()?;
    self.read_exact(" ")?;
    let generation = self.read_int()?;
    self.read_exact(" R")?;

    Ok(Reference(id, generation))
  }
  
}