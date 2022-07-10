use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum BeforeCommand {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Window {
    pub name: String,
    pub start_directory: Option<String>,
    pub before_command: Option<BeforeCommand>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub start_directory: Option<String>,
    pub session_name: Option<String>,
    pub windows: Vec<Window>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self> {
        let f = std::fs::File::open(path)?;
        serde_yaml::from_reader(f).context("Failed to deserialized config file")
    }

    pub fn from_str(s: &str) -> Result<Self> {
        serde_yaml::from_str(s).context("Failed to deserialized config file")
    }
}

#[test]
fn read_config() {
    let input = "start_directory: /tmp
session_name: Test
windows:
  - name: vim
    before_command: vim
";
    let config = Config::from_str(input).expect("Failed to read from input");
    assert_eq!(config.session_name, Some("Test".to_string()));
    assert_eq!(config.start_directory, Some("/tmp".to_string()));
    assert_eq!(
        config.windows[0],
        Window {
            name: "vim".to_string(),
            before_command: Some(BeforeCommand::One("vim".to_string())),
            start_directory: None,
        }
    );
}

#[test]
fn multiple_cmds() {
    let input = "start_directory: /tmp
session_name: Test
windows:
  - name: vim
    before_command: 
        - cd
        - vim
";
    let config = Config::from_str(input).unwrap();
    assert_eq!(
        config.windows[0],
        Window {
            name: "vim".to_string(),
            start_directory: None,
            before_command: Some(BeforeCommand::Many(vec![
                "cd".to_string(),
                "vim".to_string()
            ])),
        }
    );
}

#[test]
fn no_cmd() {
    let input = "start_directory: /tmp
session_name: Test
windows:
  - name: vim
";
    let config = Config::from_str(input).unwrap();
    assert_eq!(
        config.windows[0],
        Window {
            name: "vim".to_string(),
            before_command: None,
            start_directory: None,
        }
    );
}

#[test]
fn start_directory() {
    let input = "start_directory: /tmp
session_name: Test
windows:
  - name: vim
    start_directory: /tmp/test
";
    let config = Config::from_str(input).unwrap();
    assert_eq!(
        config.windows[0],
        Window {
            name: "vim".to_string(),
            before_command: None,
            start_directory: Some(String::from("/tmp/test")),
        }
    );
}
