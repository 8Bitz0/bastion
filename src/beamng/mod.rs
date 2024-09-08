use std::path::{Path, PathBuf};

mod args;
mod exec;

pub use args::{CommonArgs, LinuxArgs};
pub use exec::{exec, ExecError, ExecMethod};

#[derive(Debug, Clone)]
pub struct BeamNGInstall {
    path: PathBuf,
}

impl BeamNGInstall {
    pub fn init<P: AsRef<Path>>(path: P) -> Self {
        BeamNGInstall {
            path: path.as_ref().to_path_buf(),
        }
    }
    pub fn exists(&self) -> bool {
        self.path.is_dir()
    }
}
