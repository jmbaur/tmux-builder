use std::fs;
use std::io;
use std::path::PathBuf;
use tmux_interface::TmuxCommand;

pub struct Job {
    name: String,
    session_path: PathBuf,
    command: Vec<String>,
}

impl Job {
    fn next_entry_number(&self) -> Result<u32, io::Error> {
        let readdir = fs::read_dir(self.session_path.clone())?;
        let last = readdir
            .map(|path| match path {
                Ok(p) => p.file_name().to_string_lossy().parse::<u32>().unwrap_or(0),
                _ => 0,
            })
            .filter(|num| num > &0)
            .last()
            .unwrap_or(0);
        Ok(last + 1)
    }

    fn create_session(&self) -> Result<(), io::Error> {
        if !self.session_path.exists() && !self.session_path.is_dir() {
            fs::create_dir_all(&self.session_path)?;
        }

        let tmux = TmuxCommand::new();
        if !tmux
            .has_session()
            .target_session(self.name.to_string())
            .output()
            .unwrap()
            .success()
        {
            tmux.new_session()
                .session_name(self.name.to_string())
                .start_directory(self.session_path.to_str().unwrap())
                .print()
                .detached()
                .output()
                .unwrap()
                .to_string()
                .trim()
                .to_string();
            fs::create_dir_all(self.session_path.join("1"))?;

            tmux.send_keys()
                .target_pane(self.name.to_string())
                .key(format!(
                    "cd {}",
                    self.session_path.join("1").to_string_lossy()
                ))
                .key("Enter")
                .output()
                .unwrap();
        }

        Ok(())
    }
}
