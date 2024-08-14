use std::path::PathBuf;
use std::process::Command;

use super::args;

const DEFAULT_STEAM_EXEC: &str = "steam";
const BEAMNG_STEAM_ID: &str = "284160";
const WINDOWS_EXEC_PATH: &str = "Bin64/BeamNG.drive.x64.exe";

pub enum ExecMethod {
    Steam {
        steam_path: Option<PathBuf>,
    },
    Windows {
        install: super::BeamNGInstall,
        args: args::CommonArgs,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum ExecError {
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    #[error("Directory at executable: {0}")]
    DirectoryAtExecutable(PathBuf),
    #[error("Process failed")]
    ProcessFailed(std::io::Error),
}

pub fn exec(method: ExecMethod) -> Result<(), ExecError> {
    match method {
        ExecMethod::Steam { steam_path } => {
            let steam_exec = match steam_path {
                Some(p) => p.to_string_lossy().to_string(),
                None => DEFAULT_STEAM_EXEC.to_string(),
            };

            let mut command = Command::new(steam_exec);
            command.arg(format!("steam://rungameid/{BEAMNG_STEAM_ID}"));

            command.status().map_err(ExecError::ProcessFailed)?;
        }
        ExecMethod::Windows { install, args } => {
            let exec_path = install.path.join(WINDOWS_EXEC_PATH);

            if !exec_path.exists() {
                return Err(ExecError::FileNotFound(exec_path));
            } else if exec_path.is_dir() {
                return Err(ExecError::DirectoryAtExecutable(exec_path));
            }

            let mut command = Command::new("cmd.exe");
            command.args(["/C", &exec_path.to_string_lossy()]);
            command.args(args.to_args());

            command.status().map_err(ExecError::ProcessFailed)?;
        }
    }

    Ok(())
}
