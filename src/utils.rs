use std::process::{ Command, Output };

pub fn run_git(args: Vec<&str>) -> Output {
  Command::new("git").args(args).output().unwrap()
}
