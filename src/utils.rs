use std::process::{ Command, Output };

pub fn run_git(args: Vec<String>) -> Output {
  Command::new("git").args(args).output().unwrap()
}
