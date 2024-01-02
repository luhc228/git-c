use std::{env, str, path};
use console::style;

mod git_url_parser;
mod utils;

fn main() {
  let args: Vec<String> = env::args().collect();

  let git_url = git_url_parser::GitUrl::new(&args[1]).unwrap_or_else(
    |err| {
      println!("{} {}", style("ERROR").red(), err);
      std::process::exit(1);
    }
  );

  let mut command_args: Vec<String> = vec![
    "clone".to_string(),
    git_url.full_git_url,
  ];

  match git_url.branch {
    Some(branch) => {
      command_args.push(String::from("--branch"));
      command_args.push(branch);
    }
    None => {},
  }

  println!("{} Start to run command: git {}", style("INFO").cyan(), command_args.join(" "));

  let res = utils::run_git(command_args);

  match res.status.code() {
    Some(0) => {
      let path = env::current_dir().unwrap();
      let repo_path = path::Path::new(&path).join(git_url.project).into_os_string().into_string().unwrap();
      println!("{} Cloning to `{}` Successfully!", style("DONE").green(), repo_path);
    },
    Some(_) => {
      println!("{} {}", style("ERROR").red(), str::from_utf8(&res.stderr).unwrap());
      std::process::exit(1);
    },
    None => {
      println!("{} {}", style("ERROR").red(), str::from_utf8(&res.stderr).unwrap());
    }
  };
}
