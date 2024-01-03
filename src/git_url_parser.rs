use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct GitUrl {
  pub full_git_url: String,
  pub group: String,
  pub project: String,
  pub branch: Option<String>,
}

impl GitUrl {
  pub fn new(url: &str) -> Result<GitUrl, &'static str> {
    let with_git_suffix_re = Regex::new(r"[/:](?P<group>[^/]+)/(?P<project>[^/]+)\.git$").unwrap();
    let with_git_suffix_caps = with_git_suffix_re.captures(&url);
    if let Some(caps) = with_git_suffix_caps {
      return Ok(GitUrl {
        full_git_url: url.to_string(),
        group: String::from(&caps["group"]),
        project: String::from(&caps["project"]),
        branch: None,
      })
    }

    let http_url_re = Regex::new(r"(?P<repo_url>^https?://(?P<host>[^/]+)/(?P<group>[^/]+)/(?P<project>[^/]+))/?(?P<rest_path>.*)?").unwrap();
    let http_url_caps = http_url_re.captures(&url);

    if let Some(caps) = http_url_caps {
      let host = &caps["host"];
      let rest_path = &caps["rest_path"];
      let group = &caps["group"];
      let project = &caps["project"];

      // default use `git@<host>:<group>/<project>.git`` url
      let full_git_url = format!("git@{}:{}/{}.git", host, group, project);

      if rest_path == "" {
        return Ok(GitUrl {
          full_git_url,
          group: String::from(group),
          project: String::from(project),
          branch: None,
        })
      } else if rest_path.contains("tree/") {
        let rest_path_str = String::from(rest_path);
        let pos: Vec<&str> = rest_path_str.split("tree/").collect();
        let branch = pos[pos.len() - 1].to_string();
        return Ok(GitUrl {
          full_git_url,
          group: String::from(group),
          project: String::from(project),
          branch: Some(branch),
        })
      } else {
        return Err("Invalid Url.")
      }
    } else {
      return Err("Not support tot parse this url.")
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
      group: String::from("microsoft"),
      project: String::from("vscode-docs"),
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
      group: String::from("microsoft"),
      project: String::from("vscode-docs"),
    };
    assert_eq!(expected, result);
  }

  #[test]
  fn parse_git_url_without_any() {
    let git_url = String::from("https://github.com/microsoft/vscode-docs");
    let result = GitUrl::new(&git_url).expect("URL parse failed");

    let expected = GitUrl {
      full_git_url: String::from("git@github.com:microsoft/vscode-docs.git"),
      branch: None,
      group: String::from("microsoft"),
      project: String::from("vscode-docs"),
    };
    assert_eq!(expected, result);
  }

  #[test]
  fn parse_git_url_with_branch() {
    // It will use git clone
    let git_url = String::from("https://github.com/microsoft/vscode-docs/tree/vnext");
    let result = GitUrl::new(&git_url).expect("URL parse failed");

    let expected = GitUrl {
      full_git_url: String::from("git@github.com:microsoft/vscode-docs.git"),
      branch: Some(String::from("vnext")),
      group: String::from("microsoft"),
      project: String::from("vscode-docs"),
    };

    assert_eq!(expected, result);
  }
}
