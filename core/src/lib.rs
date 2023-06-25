#![deny(clippy::all)]

use tree_sitter::{Parser, Tree};

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub struct TSWrapper {
  parser: Parser,
  tree: Tree,
}

#[napi]
impl TSWrapper {
  #[napi(constructor)]
  pub fn new(text: String) -> Self {
    let mut parser = Parser::new();
    parser
      .set_language(tree_sitter_javascript::language())
      .unwrap();

    let tree = parser.parse(text, None).unwrap();

    Self { parser, tree }
  }

  #[napi]
  pub fn get_text(&self) -> String {
    self.tree.root_node().to_sexp()
  }
}
