use std::env;

mod git_url_parser;
mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);
}
