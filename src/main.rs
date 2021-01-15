<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
=======
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "creole.pest"]
pub struct CreoleParser;

fn main() {
    let successful_parse = CreoleParser::parse(Rule::paragraph, "sa**fdd**sa");
    println!("{:?}", successful_parse);
>>>>>>> 7a85d6884d655080ee4cb38392362c63093ff860
}
