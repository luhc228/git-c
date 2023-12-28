use std::env;
use console::style;

mod git_url_parser;
mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();

  let git_url = git_url_parser::GitUrl::new(&args[1]).unwrap();

  let mut command_args: Vec<String> = vec![
    "clone".to_owned(),
    git_url.full_git_url.to_owned(),
  ];

  match git_url.branch {
    Some(branch) => {
      // TODO: how to remove to_owned()?
      command_args.push("--branch".to_owned());
      command_args.push(branch.to_owned());
    }
    None => {},
  }

  println!("{} Start to run: `git {}`", style("INFO").cyan(), command_args.join(" "));

  let res = utils::run_git(command_args);
  eprintln!("{} {:?}", style("DONE").green(), res.stderr);
}
