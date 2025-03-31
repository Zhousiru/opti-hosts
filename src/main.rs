use parser::{DirectiveParser, Rule};
use pest::Parser;

mod parser;

fn main() {
  let parsed = DirectiveParser::parse(
    Rule::directive,
    "# OPTI-HOSTS example.com [Beijing, HK * 2, AS6939]",
  )
  .unwrap();
  println!("{:#?}", parsed)
}
