use std::vec;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/definition.pest"]
pub struct DirectiveParser;

#[derive(Debug, Clone)]
pub struct LocationParam {
  pub location: String,
  pub limit: usize,
}

#[derive(Debug, Clone)]
pub struct DirectiveMeta {
  pub line_index: usize,
  pub domain: String,
  pub location_params: Vec<LocationParam>,
}

pub fn parse_hosts(hosts: &str) -> Vec<DirectiveMeta> {
  let mut directive_metas: Vec<DirectiveMeta> = vec![];

  for (line_index, line) in hosts.lines().enumerate() {
    if !line.starts_with("#") {
      continue;
    }

    match DirectiveParser::parse(Rule::directive, line) {
      Ok(mut parsed) => {
        let directive = parsed.next().unwrap();
        let mut expression_inner = directive.into_inner().nth(1).unwrap().into_inner();
        let domain_value = String::from(expression_inner.next().unwrap().as_str());
        let array_inner = expression_inner.next().unwrap().into_inner();

        let mut location_params: Vec<LocationParam> = vec![];

        for item in array_inner {
          let mut item_inner = item.into_inner();
          let location_value = String::from(item_inner.next().unwrap().as_str());
          let limit_value: usize = item_inner
            .next()
            .and_then(|v| v.as_str().parse().ok())
            .unwrap_or(1);

          if limit_value < 1 {
            continue;
          }

          location_params.push(LocationParam {
            location: location_value,
            limit: limit_value,
          });
        }

        if location_params.len() < 1 {
          continue;
        }

        directive_metas.push(DirectiveMeta {
          line_index,
          domain: domain_value,
          location_params,
        });
      }
      Err(_) => (),
    }
  }

  return directive_metas;
}
