use std::ascii;
use std::io::{self, Read};

use std::io::prelude::*;
use flate2::read::ZlibDecoder;

use document::page::Page as SourcePage;
use document::page::PageList;

use document::page::Node;


#[derive(Debug)]
pub struct Page {
  content: Vec<u8>
}


impl Page {

  pub fn from_pdf_page_list(page_list: &PageList) -> Vec<Page> {
    let mut buffer = Vec::new();

    for node in &page_list.nodes {
      match node {
        &Node::List(ref list) => {
          let mut converted = Page::from_pdf_page_list(list);
          buffer.append(&mut converted)
        },
        &Node::Page(ref page) => {
          let converted = Page::from_pdf(page);
          buffer.push(converted);
        }
      }
    }

    buffer
  }


  pub fn from_pdf(source: &SourcePage) -> Page {
    let bytes =
      match source.contents {
        Some(ref contents) => {
          let mut decoder = ZlibDecoder::new(&contents.bytes[..]);
          let mut buffer = Vec::new();
          decoder.read_to_end(&mut buffer).unwrap();
          buffer
        }
        None => {
          Vec::new()
        }
      };

    Page {
      content: bytes
    }
  }

}