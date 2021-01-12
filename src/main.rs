extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "creole.pest"]
pub struct CreoleParser;

fn main() {
    let successful_parse = CreoleParser::parse(Rule::word, "safddsa");
    println!("{:?}", successful_parse);
}
