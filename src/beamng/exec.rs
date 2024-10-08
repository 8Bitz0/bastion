use std::path::PathBuf;
use std::process::Command;

use super::{args, BeamNGInstall};

const DEFAULT_STEAM_EXEC: &str = "steam";
const BEAMNG_STEAM_ID: &str = "284160";
const WINDOWS_EXEC_PATH: &str = "Bin64/BeamNG.x64.exe";
const LINUX_EXEC_PATH: &str = "BinLinux/BeamNG.x64";
const WINDOWS_LAUNCHER_EXEC_PATH: &str = "BeamNG.drive.exe";
const APPLE_GPTK_WINE_PATH: &str = "Contents/Resources/wine/bin/wine";

pub enum ExecMethod {
    Steam {
        steam_path: Option<PathBuf>,
    },
    Windows {
        install: BeamNGInstall,
        args: args::CommonArgs,
    },
    WindowsIndirect {
        install: BeamNGInstall,
    },
    Linux {
        install: BeamNGInstall,
        args: args::LinuxArgs,
    },
    MacGPTK {
        install: BeamNGInstall,
        args: args::CommonArgs,
        gptk_path: PathBuf,
    },
    MacGPTKIndirect {
        install: BeamNGInstall,
        gptk_path: PathBuf,
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
        ExecMethod::WindowsIndirect { install } => {
            let exec_path = install.path.join(WINDOWS_LAUNCHER_EXEC_PATH);

            if !exec_path.exists() {
                return Err(ExecError::FileNotFound(exec_path));
            } else if exec_path.is_dir() {
                return Err(ExecError::DirectoryAtExecutable(exec_path));
            }

            let mut command = Command::new("cmd.exe");
            command.args(["/C", &exec_path.to_string_lossy()]);

            command.status().map_err(ExecError::ProcessFailed)?;
        }
        ExecMethod::Linux { install, args } => {
            let exec_path = install.path.join(LINUX_EXEC_PATH);

            if !exec_path.exists() {
                return Err(ExecError::FileNotFound(exec_path));
            } else if exec_path.is_dir() {
                return Err(ExecError::DirectoryAtExecutable(exec_path));
            }

            let mut command = Command::new(&exec_path);
            command.args(args.to_args());

            command.status().map_err(ExecError::ProcessFailed)?;
        }
        ExecMethod::MacGPTK { install, args, gptk_path } => {
            let exec_path = install.path.join(WINDOWS_EXEC_PATH);

            if !exec_path.exists() {
                return Err(ExecError::FileNotFound(exec_path));
            } else if exec_path.is_dir() {
                return Err(ExecError::DirectoryAtExecutable(exec_path));
            }

            let gptk_wine_path = gptk_path.join(APPLE_GPTK_WINE_PATH);

            let mut command = Command::new(&gptk_wine_path);
            command.arg(exec_path.to_string_lossy().as_ref());
            command.args(args.to_args());

            command.status().map_err(ExecError::ProcessFailed)?;
        }
        ExecMethod::MacGPTKIndirect { install, gptk_path } => {
            let exec_path = install.path.join(WINDOWS_LAUNCHER_EXEC_PATH);

            if !exec_path.exists() {
                return Err(ExecError::FileNotFound(exec_path));
            } else if exec_path.is_dir() {
                return Err(ExecError::DirectoryAtExecutable(exec_path));
            }

            let gptk_wine_path = gptk_path.join(APPLE_GPTK_WINE_PATH);

            let mut command = Command::new(&gptk_wine_path);
            command.arg(exec_path.to_string_lossy().as_ref());

            command.status().map_err(ExecError::ProcessFailed)?;
        }
    }

    Ok(())
}
