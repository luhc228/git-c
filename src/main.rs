use std::{env, str, path};
use console::style;
use git_url_parser::GitUrl;

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

  let command_args: Vec<String> = get_git_command_args(&git_url);

  println!("{} Start to run command: {}", style("INFO").cyan(), format!("git {}", command_args.join(" ")));

  let run_git_command_output = utils::run_git(command_args);
  if let Some(0) = run_git_command_output.status.code() {
    let path = env::current_dir().unwrap();
    let repo_path = path::Path::new(&path).join(git_url.project).into_os_string().into_string().unwrap();
    println!("{} {}", style("DONE").green(), format!("Clone to `{}` successfully!", repo_path));
  } else {
    println!("{} {}", style("ERROR").red(), str::from_utf8(&run_git_command_output.stderr).unwrap());
    std::process::exit(1);
  }
}

fn get_git_command_args(git_url: &GitUrl) -> Vec<String> {
  let mut command_args: Vec<String> = vec![
    "clone".to_string(),
    git_url.full_git_url.clone(),
  ];

  if let Some(branch) = git_url.branch.clone() {
    command_args.push(String::from("--branch"));
    command_args.push(branch);
  }

  command_args
}