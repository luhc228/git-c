use regex::Regex;

pub struct GitUrl {
  // 
  full_git_url: String,
  branch: Option<String>,
}

impl GitUrl {
  pub fn new(args: &[String]) -> Result<GitUrl, &'static str> {
    if args.len() == 0 {
      return Err("No args are found.")
    }
    let url  = &args[0];

    let with_git_suffix_re = Regex::new(r"\.git$").unwrap();
    let with_git_suffix_caps = with_git_suffix_re.captures(&url);


    let re = Regex::new(r"(?P<git_url>^https?://[^/]+)/(?:?P<pathname>.*)").unwrap();
    let caps = re.captures(&url).unwrap();

    println!("111 {:?} {:?}", &caps["host"], &caps["pathname"]);

    Ok(GitUrl {
      full_git_url: String::from("xxx"),
      branch: None,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_git_url_with_https() {
    // It will use git clone
    let git_url = String::from("https://github.com/microsoft/vscode-docs.git");
    let args = vec![git_url];
    GitUrl::new(&args);
  }

  #[test]
  fn parse_git_url_with_ssh() {
    // It will use git clone
    let git_url = String::from("git@github.com:microsoft/vscode-docs.git");
    let args = vec![git_url];
    GitUrl::new(&args);
  }

  #[test]
  fn parse_git_url_without_any() {
    // It will use git clone
    let git_url = String::from("https://github.com/microsoft/vscode-docs");
    let args = vec![git_url];
    GitUrl::new(&args);
  }

  #[test]
  fn parse_git_url_with_branch() {
    // It will use git clone
    let git_url = String::from("https://github.com/microsoft/vscode-docs/tree/vnext");
    let args = vec![git_url];
    GitUrl::new(&args);
  }
}
