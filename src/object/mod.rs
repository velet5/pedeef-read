pub mod integer;
pub mod name;
pub mod reference;

use std::collections::HashMap;


#[derive(Eq, PartialEq, Debug)]
pub struct Reference {
  id: i32,
  generation: i32
}


#[derive(Debug)]
pub struct IndirectObject {
  pub id: i32,
  pub generation: i32,
  pub object: Object
}


#[derive(Debug)]
pub enum Object {
  Null,
  Boolean(bool),
  Real(f32),
  Integer(i32),
  String(String),
  Name(String),
  Dictionary(HashMap<String, Object>),
  Reference(Reference)
}