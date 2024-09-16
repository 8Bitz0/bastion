use clap::{Parser, Subcommand};
use std::path::PathBuf;

use bastion::{exec, BeamNGInstall, CommonArgs, ExecMethod, LinuxArgs};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Start {
        #[clap(subcommand)]
        method: RunMethod,
    },
}

#[derive(Debug, Subcommand)]
pub enum RunMethod {
    /// Opens the game using Steam. No game arguments are supported.
    Steam {
        /// Override the command to execute Steam
        steam_exec: Option<PathBuf>,
    },
    /// Directly launches the game and passes requested arguments.
    Windows {
        /// Very root of the BeamNG.drive install
        install_path: PathBuf,
        /// Open with BeamNG.drive console
        #[arg(long, short = 'c')]
        console: bool,
        /// Override game graphics API
        #[arg(long)]
        gfx_api: Option<String>,
    },
    /// Opens the game's launcher. No game arguments are supported.
    WindowsIndirect {
        /// Very root of the BeamNG.drive install
        install_path: PathBuf,
    },
    /// Directly launches the game using the Linux binary and passes requested arguments.
    Linux {
        /// Very root of the BeamNG.drive install
        install_path: PathBuf,
        /// Override game graphics API
        #[arg(long, short = 'c')]
        gfx_api: Option<String>,
    },
    /// Directly launches the game using the Apple Game Porting Toolkit and passes requested arguments.
    MacGPTK {
        /// Very root of the BeamNG.drive install
        install_path: PathBuf,
        /// Path to the Apple GPTK Wine binary
        gptk_path: PathBuf,
        /// Open with BeamNG.drive console
        #[arg(long, short = 'c')]
        console: bool,
        /// Override game graphics API
        #[arg(long)]
        gfx_api: Option<String>,
    },
    /// Opens the game's launcher using the Apple Game Porting Toolkit. No game arguments are supported.
    MacGPTKIndirect {
        /// Very root of the BeamNG.drive install
        install_path: PathBuf,
        /// Path to the Apple GPTK Wine binary
        gptk_path: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Start { method } => match method {
            RunMethod::Steam { steam_exec } => {
                match exec(ExecMethod::Steam {
                    steam_path: steam_exec,
                }) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("Steam process failed: {e}");
                        std::process::exit(1);
                    }
                }
            }
            RunMethod::Windows {
                install_path,
                console,
                gfx_api,
            } => {
                let install = BeamNGInstall::init(install_path);

                if !install.exists() {
                    eprintln!("Given install does not exist.");
                    std::process::exit(1);
                }

                let args = CommonArgs { console, gfx_api };

                match exec(ExecMethod::Windows { install, args }) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("BeamNG.drive process failed: {e}");
                        std::process::exit(1);
                    }
                }
            }
            RunMethod::WindowsIndirect {
                install_path,
            } => {
                let install = BeamNGInstall::init(install_path);

                if !install.exists() {
                    eprintln!("Given install does not exist.");
                    std::process::exit(1);
                }

                match exec(ExecMethod::WindowsIndirect { install }) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("BeamNG.drive process failed: {e}");
                        std::process::exit(1);
                    }
                }
            }
            RunMethod::Linux {
                install_path,
                gfx_api,
            } => {
                let install = BeamNGInstall::init(install_path);

                if !install.exists() {
                    eprintln!("Given install does not exist.");
                    std::process::exit(1);
                }

                let args = LinuxArgs { gfx_api };

                match exec(ExecMethod::Linux { install, args }) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("BeamNG.drive process failed: {e}");
                        std::process::exit(1);
                    }
                }
            }
            RunMethod::MacGPTK {
                install_path,
                console,
                gfx_api,
                gptk_path
            } => {
                let install = BeamNGInstall::init(install_path);

                if !install.exists() {
                    eprintln!("Given install does not exist.");
                    std::process::exit(1);
                }

                let args = CommonArgs { console, gfx_api };

                match exec(ExecMethod::MacGPTK { install, args, gptk_path }) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("BeamNG.drive process failed: {e}");
                        std::process::exit(1);
                    }
                }
            }
            RunMethod::MacGPTKIndirect {
                install_path,
                gptk_path
            } => {
                let install = BeamNGInstall::init(install_path);

                if !install.exists() {
                    eprintln!("Given install does not exist.");
                    std::process::exit(1);
                }

                match exec(ExecMethod::MacGPTKIndirect { install, gptk_path }) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("BeamNG.drive process failed: {e}");
                        std::process::exit(1);
                    }
                }
            }
        },
    }
}
