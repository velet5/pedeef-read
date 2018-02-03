mod reader;
mod object;
mod document;

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
    Ok(mut stream) => read_from_stream(&mut stream),
    Err(err) => println!("{}", err.to_string())
  }
}


fn read_from_stream(stream: &mut Stream) -> () {
  let xref = document::xref::read_xref(stream);

  println!("{:?}", xref);
}