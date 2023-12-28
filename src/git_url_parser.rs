use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct GitUrl {
  pub full_git_url: String,
  pub branch: Option<String>,
}

impl GitUrl {
  pub fn new(url: &str) -> Result<GitUrl, &'static str> {
    let with_git_suffix_re = Regex::new(r"\.git$").unwrap();
    let with_git_suffix_caps = with_git_suffix_re.captures(&url);
    if with_git_suffix_caps.is_some() {
      return Ok(GitUrl {
        full_git_url: url.to_string(),
        branch: None,
      })
    }

    let re = Regex::new(r"(?P<repo_url>^https?://[^/]+/[^/]+/[^/]+)/?(?P<rest_path>.*)?").unwrap();
    let caps = re.captures(&url).expect("Parse Error. Invalid url.");

    let repo_url = &caps["repo_url"];
    let rest_path = &caps["rest_path"];
    
    let mut full_git_url = String::from(repo_url);
    full_git_url.push_str(".git");

    if rest_path == "" {
      return Ok(GitUrl {
        full_git_url,
        branch: None,
      })
    } else if rest_path.contains("tree/") {
      let rest_path_str = String::from(rest_path);
      let pos: Vec<&str> = rest_path_str.split("tree/").collect();
      let branch = pos[pos.len() - 1].to_string();
      return Ok(GitUrl {
        full_git_url,
        branch: Some(branch),
      })
    } else {
      return Err("Invalid Url.")
    }
   
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_git_url_with_https() {
    let git_url = String::from("https://github.com/microsoft/vscode-docs.git");
    let result = GitUrl::new(&git_url).expect("URL parse failed");
    let expected = GitUrl {
      full_git_url: git_url, 
      branch: None,
    };
    assert_eq!(expected, result);
  }

  #[test]
  fn parse_git_url_with_ssh() {
    let git_url = String::from("git@github.com:microsoft/vscode-docs.git");
    let result = GitUrl::new(&git_url).expect("URL parse failed");
    let expected = GitUrl {
      full_git_url: git_url, 
      branch: None,
    };
    assert_eq!(expected, result);
  }

  #[test]
  fn parse_git_url_without_any() {
    let git_url = String::from("https://github.com/microsoft/vscode-docs");
    let result = GitUrl::new(&git_url).expect("URL parse failed");
    
    let mut full_git_url = git_url;
    full_git_url.push_str(".git");
    let expected = GitUrl {
      full_git_url,
      branch: None,
    };
    assert_eq!(expected, result);
  }

  #[test]
  fn parse_git_url_with_branch() {
    // It will use git clone
    let git_url = String::from("https://github.com/microsoft/vscode-docs/tree/vnext");
    let result = GitUrl::new(&git_url).expect("URL parse failed");

    let expected = GitUrl {
      full_git_url: String::from("https://github.com/microsoft/vscode-docs.git"),
      branch: Some(String::from("vnext")),
    };

    assert_eq!(expected, result);
  }
}
