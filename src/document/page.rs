use std::any::Any;
use std::collections::HashMap;

use reader::result::ReadResult;
use reader::common::*;
use reader::characters::*;

use object::reference::*;
use object::*;
use object::date::Date;
use object::name::*;
use object::rectangle::*;

use document::map::*;
use document::group::*;
use document::boxed::*;
use document::dictionary::*;
use document::resources::*;
use document::contents::Contents;
use document::reader::DocumentReader;

use object::stream::Stream;

#[derive(Debug)]
struct Empty;

struct PageLike {
  tpe: String,
  parent: Option<Reference>,
  kids: Option<Vec<Reference>>,
  count: Option<i32>,
  last_modified: Option<Date>,
  resources: Option<Resources>,
  media_box: Option<Rectangle>,
  crop_box: Option<Rectangle>,
  bleed_box: Option<Rectangle>,
  trim_box: Option<Rectangle>,
  art_box: Option<Rectangle>,
  box_color_info: Option<Empty>,
  contents: Option<Reference>,
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


#[derive(Debug)]
pub enum Node {
  List(PageList),
  Page(Page)
}


#[derive(Debug)]
pub struct Page {
  tpe: String,
  parent: Option<Reference>,
  last_modified: Option<Date>,
  resources: Option<Resources>,
  media_box: Option<Rectangle>,
  crop_box: Option<Rectangle>,
  bleed_box: Option<Rectangle>,
  trim_box: Option<Rectangle>,
  art_box: Option<Rectangle>,
  box_color_info: Option<Empty>,
  contents: Option<Reference>,
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


#[derive(Debug)]
pub struct PageList {
  tpe: String,
  parent: Option<Reference>,
  kids: Vec<Reference>,
  count: i32,
  nodes: Vec<Node>
}


impl PageLike {
  pub fn is_page(&self) -> bool {
    self.tpe == "Page"
  }


  pub fn is_page_list(&self) -> bool {
    self.tpe == "Pages"
  }


  pub fn as_page(self) -> Option<Page> {
    if self.is_page() {
      Some(Page {
        tpe: self.tpe,
        parent: self.parent,
        last_modified: self.last_modified,
        resources: self.resources,
        media_box: self.media_box,
        crop_box: self.crop_box,
        bleed_box: self.bleed_box,
        trim_box: self.trim_box,
        art_box: self.art_box,
        box_color_info: self.box_color_info,
        contents: self.contents,
        rotate: self.rotate,
        group: self.group,
        thumb: self.thumb,
        b: self.b,
        dur: self.dur,
        trans: self.trans,
        annots: self.annots,
        aa: self.aa,
        metadata: self.metadata,
        piece_info: self.piece_info,
        struct_parents: self.struct_parents,
        id: self.id,
        pz: self.pz,
        separation_info: self.separation_info,
        tabs: self.tabs,
        template_instantiated: self.template_instantiated,
        pres_steps: self.pres_steps,
        user_unit: self.user_unit,
        vp: self.vp,
      })
    } else {
      None
    }
  }


  pub fn as_page_list(self) -> Option<PageList> {
    if self.is_page_list() {
      Some(PageList {
        tpe: self.tpe,
        parent: self.parent,
        kids: self.kids.unwrap(),
        count: self.count.unwrap(),
        nodes: Vec::new()
      })
    } else {
      None
    }
  }
}


pub fn read_pages(reader: &mut DocumentReader, reference: &Reference) -> ReadResult<PageList> {
  let node = read_node(reader, reference)?;

  match node {
    Node::List(page_list) =>
      Ok(page_list),
    _ =>
      unreachable!()
  }
}


fn read_node(reader: &mut DocumentReader, reference: &Reference) -> ReadResult<Node> {
  let position = reader.map.value.get(reference).unwrap().clone();
  let _ignored = read_object_id(&mut reader.stream, position)?;
  skip_whitespace(&mut reader.stream);

  let mut map: HashMap<&str, &Fn(&mut DocumentReader) -> ReadResult<Box<Any>>> = HashMap::new();

  //tpe
  map.insert("Type", &read_name_boxed);
  map.insert("Count", &read_int_boxed);
  map.insert("Kids", &read_kids_reference_array_boxed);
  map.insert("Parent", &read_reference_boxed);
  map.insert("Contents", &read_reference_boxed);
  map.insert("MediaBox", &read_rectangle_boxed);
  map.insert("CropBox", &read_rectangle_boxed);
  map.insert("BleedBox", &read_rectangle_boxed);
  map.insert("TrimBox", &read_rectangle_boxed);
  map.insert("ArtBox", &read_rectangle_boxed);
  map.insert("Group", &read_group_boxed);
  map.insert("Resources", &read_resources_boxed);
  //parent
  //kids
  //count
  //last_modified
  //resources
  //media_box
  //crop_box
  //bleed_box
  //trim_box
  //art_box
  //box_color_info
  //contents <----
  //rotate
  //group
  //thumb
  //b
  //dur
  //trans
  //annots
  //aa
  //metadata
  //piece_info
  //struct_parents
  //id
  //pz
  //separation_info
  //tabs
  //template_instantiated
  //pres_steps
  //user_unit
  //vp

  let dictionary = &mut read_dictionary(reader, &mut map)?;

  let tpe = *unfold("Type", dictionary)?;
  let count = *unfold_optional("Count", dictionary)?;
  let kids = *unfold_optional("Kids", dictionary)?;
  let parent = *unfold_optional("Parent", dictionary)?;

  let page_like = PageLike {
    tpe,
    parent,
    kids,
    count,
    last_modified: None,
    resources: None,
    media_box: None,
    crop_box: None,
    bleed_box: None,
    trim_box: None,
    art_box: None,
    box_color_info: None,
    contents: None,
    rotate: None,
    group: None,
    thumb: None,
    b: None,
    dur: None,
    trans: None,
    annots: None,
    aa: None,
    metadata: None,
    piece_info: None,
    struct_parents: None,
    id: None,
    pz: None,
    separation_info: None,
    tabs: None,
    template_instantiated: None,
    pres_steps: None,
    user_unit: None,
    vp: None,
  };

  if page_like.is_page_list() {
    let mut list = page_like.as_page_list().unwrap();
    for kid in &list.kids {
      let node = read_node(reader, &kid)?;
      list.nodes.push(node);
    }

    Ok(Node::List(list))
  } else {
    let mut page = page_like.as_page().unwrap();
    Ok(Node::Page(page))
  }
}

fn read_kids_reference_array(reader: &mut DocumentReader) -> ReadResult<Vec<Reference>> {
  skip_whitespace(&mut reader.stream);
  skip(&mut reader.stream, "[")?;
  skip_whitespace(&mut reader.stream);

  let mut buffer = Vec::new();

  while is_digit(reader.stream.peek()) {
    let reference = read_reference(&mut reader.stream)?;
    buffer.push(reference);
    skip_whitespace(&mut reader.stream);
  }

  skip(&mut reader.stream, "]")?;
  skip_whitespace(&mut reader.stream);

  Ok(buffer)
}


fn read_kids_reference_array_boxed(reader: &mut DocumentReader) -> ReadResult<Box<Any>> {
  boxed(read_kids_reference_array(reader))
}