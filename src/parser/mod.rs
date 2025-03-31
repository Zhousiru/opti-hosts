use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/definition.pest"]
pub struct DirectiveParser;
