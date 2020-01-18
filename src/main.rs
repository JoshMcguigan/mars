use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
};
use structopt::StructOpt;

mod args;
pub use args::{Args, BuildArgs, CommonArgs, Subcommands};

mod build;
use build::build;

mod config;
pub use config::Config;

fn main() {
    let args = Args::from_args();

    let repo_root = match get_repo_root() {
        Some(path) => path,
        None => {
            eprintln!("You must run mars within a servo repository.");
            exit(1)
        }
    };

    let config = Config::new(&repo_root);

    match args.cmd {
        Subcommands::Build(build_args) => build(repo_root, config, build_args, args.common),
    };
}

/// When the current working directory is either the root
/// of a servo repository or a subdirectory of a servo
/// repository, this will return the path to the root of
/// the repository. Otherwise it will return None.
fn get_repo_root() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().expect("failed to read current working directory");

    loop {
        if is_servo_repo_root(&current_dir) {
            return Some(current_dir);
        }

        // mutate current_dir into it's parent directory
        // and return None if there is no parent (we've
        // recursed all the way to root)
        if !current_dir.pop() {
            return None;
        }
    }
}

fn is_servo_repo_root(path: &Path) -> bool {
    let mut path = path.to_path_buf();
    path.push("servobuild.example");

    path.exists()
}
