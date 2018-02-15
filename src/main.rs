mod reader;
mod object;
mod document;

use document::reader::DocumentReader;

use reader::stream::Stream;
use reader::result::ReadResult;



fn main() {
  read_file("/home/oarshinskii/getting-real.pdf");

  println!();
  println!("====================================");
  println!();

  read_file("/home/oarshinskii/worktime-2018-02-01.pdf")
}


fn read_file(file_name: &str) -> () {

  match Stream::from_file(file_name) {
    Ok(mut stream) => {
      let result = read_from_stream(stream);
      match result {
        Ok(_) =>  { /* ignore */ },
        Err(err) => println!("Error: {:?}", err)
      }
    },

    Err(err) => println!("{:?}", err.to_string())
  }
}


fn read_from_stream(s: Stream) -> ReadResult<()> {
  let mut stream = s;

  let xref = document::xref::read_xref(&mut stream)?;
  let position = stream.position();
  println!("{:?}", xref);

  let object_map = document::map::read_object_map(&mut stream, &xref)?;
  println!("{:?}", object_map);

  stream.set_position(position);
  let mut reader = DocumentReader {stream, map: object_map};

  let trailer = document::trailer::read_trailer(&mut reader)?;
  println!("{:?}", trailer);

  let root = document::root::read_root(&mut reader, &trailer.root)?;
  println!("{:?}", root);

  let info = document::info::read_info(&mut reader, &trailer.info)?;
  println!("{:?}", info);

  let pages = document::page::read_pages(&mut reader, &root.pages)?;
  println!("{:?}", pages);

  Ok(())
}
