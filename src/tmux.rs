use crate::config;
use anyhow::{Context, Result};
use tmux_interface::{AttachSession, NewSession, TmuxCommand};

pub struct Session {
    pub name: Option<String>,
    pub dir: String,
    pub windows: Vec<Window>,
}

impl Session {
    pub fn from_config(config: config::Config) -> Self {
        Session {
            name: config.session_name,
            dir: config.start_directory.unwrap_or_else(|| String::from(".")),
            windows: config
                .windows
                .into_iter()
                .map(Window::from_config)
                .collect(),
        }
    }

    pub fn build(&self) -> Result<()> {
        let root = &self.dir;
        let mut session = NewSession::new();
        session.detached();
        session.start_directory(root);

        if let Some(name) = &self.name {
            session.session_name(name);
        }

        for (i, window) in self.windows.iter().enumerate() {
            if i == 0 {
                if let Some(name) = &window.name {
                    session.window_name(name);
                }

                session.output()?;

                if let Some(cmds) = &window.cmds {
                    send_command(cmds.to_vec())?;
                }
            } else {
                window.build(root)?;
            }
        }

        AttachSession::new()
            .output()
            .context("Failed to attach to session")?;

        Ok(())
    }
}

pub struct Window {
    pub name: Option<String>,
    pub dir: Option<String>,
    pub cmds: Option<Vec<String>>,
}

impl Window {
    pub fn from_config(config: config::Window) -> Self {
        Window {
            name: Some(config.name),
            dir: config.start_directory,
            cmds: match config.before_command {
                None => None,
                Some(config::BeforeCommand::One(cmd)) => Some(vec![cmd]),
                Some(config::BeforeCommand::Many(cmds)) => Some(cmds),
            },
        }
    }

    pub fn build(&self, root: &str) -> Result<()> {
        let mut cmd = TmuxCommand::new().new_window();

        if let Some(name) = &self.name {
            cmd.window_name(name);
        }

        if let Some(dir) = &self.dir {
            cmd.start_directory(dir);
        } else {
            cmd.start_directory(root);
        }

        cmd.output()?;

        if let Some(cmds) = &self.cmds {
            send_command(cmds.to_vec())?;
        }

        Ok(())
    }
}

pub fn send_command(cmds: Vec<String>) -> Result<()> {
    for cmd in cmds {
        TmuxCommand::new().send_keys().key(cmd + "\n").output()?;
    }
    Ok(())
}
