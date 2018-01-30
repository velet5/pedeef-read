mod simple;
mod xref;
mod token;
mod object;
mod trailer;

use xref::Xref;
use xref::XrefParser;
use object::*;
use simple::*;

fn main() {

  read_file("/home/oarshinskii/getting-real.pdf");

  println!();
  println!("====================================");
  println!();

  read_file("/home/oarshinskii/worktime-2018-01-30.pdf")

}


fn read_file(file_name: &str) -> () {
  match Reader::read_from_file(file_name) {
    Ok(items) =>
      items.iter()
        .filter(|item| match item.2 { Object::Integer(777) => false, _ => true})
        .for_each(|item| println!("{:?}", item)),
    Err(message) => println!("{}", message)
  }
}
