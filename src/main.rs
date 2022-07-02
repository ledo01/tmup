mod cli;
mod load;

use crate::cli::Args;
use crate::load::Config;
use clap::Parser;
use load::BeforeCommand;
use tmux_interface::{AttachSession, NewSession, TmuxCommand};

struct Session {
    name: Option<String>,
    dir: Option<String>,
    windows: Vec<Window>,
}

impl Session {
    pub fn from_config(config: Config) -> Self {
        Session {
            name: config.session_name,
            dir: config.start_directory,
            windows: config
                .windows
                .into_iter()
                .map(Window::from_config)
                .collect(),
        }
    }

    pub fn build(mut self) {
        let root = self.dir.unwrap_or_else(|| ".".to_string());
        let mut session = NewSession::new();
        session.detached();
        session.start_directory(root.clone());

        if let Some(name) = self.name {
            session.session_name(name);
        }

        let first_window = self.windows.pop().unwrap();

        if let Some(name) = first_window.name {
            session.window_name(name);
        }

        session.output().unwrap();

        if let Some(cmds) = first_window.cmds {
            send_command(cmds)
        }

        for window in self.windows {
            window.build(root.clone())
        }

        AttachSession::new().output().unwrap();
    }
}

struct Window {
    name: Option<String>,
    dir: Option<String>,
    cmds: Option<Vec<String>>,
}

impl Window {
    pub fn from_config(config: load::Window) -> Self {
        Window {
            name: Some(config.name),
            dir: None,
            cmds: match config.before_command {
                None => None,
                Some(BeforeCommand::One(cmd)) => Some(vec![cmd]),
                Some(BeforeCommand::Many(cmds)) => Some(cmds),
            },
        }
    }

    pub fn build(self, root: String) {
        let mut cmd = TmuxCommand::new().new_window();

        if let Some(name) = self.name {
            cmd.window_name(name);
        }

        cmd.start_directory(self.dir.unwrap_or(root));

        cmd.output().unwrap();

        if let Some(cmds) = self.cmds {
            send_command(cmds);
        }
    }
}

pub fn send_command(cmds: Vec<String>) {
    for cmd in cmds {
        TmuxCommand::new()
            .send_keys()
            .key(cmd + "\n")
            .output()
            .unwrap();
    }
}

fn main() -> Result<(), ()> {
    let args = Args::parse();
    let config = Config::from_file(args.file);
    let session = Session::from_config(config);
    session.build();

    Ok(())
}
