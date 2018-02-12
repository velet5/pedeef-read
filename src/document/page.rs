use std::any::Any;

use reader::result::ReadResult;

use object::reference::*;
use object::*;
use object::date::Date;
use object::rectangle::Rectangle;

use document::resources::Resources;
use document::contents::Contents;
use document::reader::DocumentReader;

use object::stream::Stream;

struct Empty;

struct PageLike {
  tpe: String,
  parent: Option<Reference>,
  kids: Option<Vec<Reference>>,
  count: i32,
  last_modified: Option<Date>,
  resources: Resources,
  media_box: Rectangle,
  crop_box: Option<Rectangle>,
  bleed_box: Option<Rectangle>,
  trim_box: Option<Rectangle>,
  art_box: Option<Rectangle>,
  box_color_info: Option<Empty>,
  contents: Option<Stream>,
  rotate: Option<i32>,
  group: Option<Empty>,
  thumb: Option<Stream>,
  b: Option<Vec<Reference>>,
  dur: Option<f32>,
  trans: Option<Empty>,
  annots: Option<Vec<Reference>>,
  aa: Option<Empty>,
  metadata: Option<Stream>,
  piece_info: Option<Empty>,
  struct_parents: Option<i32>,
  id: Option<String>,
  pz: Option<f32>,
  separation_info: Option<Empty>,
  tabs: Option<String>,
  template_instantiated: Option<String>,
  pres_steps: Option<Empty>,
  user_unit: Option<f32>,
  vp: Option<Empty>
}


pub enum Node {
  Lists(Vec<PageList>),
  Pages(Vec<Page>)
}


pub struct Page {
  tpe: String,
  parent: Option<Reference>,
  last_modified: Option<Date>,
  resources: Resources,
  media_box: Rectangle,
  crop_box: Option<Rectangle>,
  bleed_box: Option<Rectangle>,
  trim_box: Option<Rectangle>,
  art_box: Option<Rectangle>,
  box_color_info: Option<Empty>,
  contents: Option<Stream>,
  rotate: Option<i32>,
  group: Option<Empty>,
  thumb: Option<Stream>,
  b: Option<Vec<Reference>>,
  dur: Option<f32>,
  trans: Option<Empty>,
  annots: Option<Vec<Reference>>,
  aa: Option<Empty>,
  metadata: Option<Stream>,
  piece_info: Option<Empty>,
  struct_parents: Option<i32>,
  id: Option<String>,
  pz: Option<f32>,
  separation_info: Option<Empty>,
  tabs: Option<String>,
  template_instantiated: Option<String>,
  pres_steps: Option<Empty>,
  user_unit: Option<f32>,
  vp: Option<Empty>
}


pub struct PageList {
  tpe: String,
  parent: Option<Reference>,
  kids: Option<Vec<Reference>>,
  count: i32,
  nodes: Vec<Node>
}


impl PageLike {

  pub fn is_page(&self) -> bool {
    unimplemented!()
  }

  
  pub fn is_page_list(&self) -> bool {
    unimplemented!()
  }


  pub fn as_page(&self) -> Option<Page> {
    unimplemented!()
  }


  pub fn as_page_list(&self) -> Option<Page> {
    unimplemented!()
  }

}


pub fn read_pages(reader: &mut DocumentReader) -> ReadResult<PageList> {
  unimplemented!()
}


fn read_page_like(reader: &mut DocumentReader) -> ReadResult<PageLike> {
  unimplemented!()
}