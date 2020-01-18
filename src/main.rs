use std::{
    env,
    path::{Path, PathBuf},
    process::{exit, Command},
};
use structopt::StructOpt;

mod build;
use build::build;

mod config;
use config::Config;

#[derive(StructOpt)]
#[structopt(about = "the Servo build tool")]
struct Args {
    #[structopt(flatten)]
    common: CommonArgs,
    #[structopt(subcommand)]
    cmd: Subcommands,
}

#[derive(StructOpt)]
pub struct CommonArgs {
    #[structopt(long, short)]
    target: Option<String>,
    // TODO translation
    // mach limits this to "gstreamer" or "dummy", but defaults
    // to None. Can this be expressed in structopt?
    #[structopt(long)]
    media_stack: Option<String>,
    #[structopt(long)]
    android: bool,
    #[structopt(long)]
    magicleap: bool,
    #[structopt(long)]
    libsimpleservo: bool,
    #[structopt(long)]
    features: Vec<String>,
    #[structopt(long)]
    debug_mozjs: bool,
    #[structopt(long)]
    with_debug_assertions: bool,
    #[structopt(long)]
    with_frame_pointer: bool,
    #[structopt(long)]
    with_raqote: bool,
    #[structopt(long)]
    with_layout_2020: bool,
    #[structopt(long)]
    with_layout_2013: bool,
    #[structopt(long)]
    without_wgl: bool,
}

#[derive(StructOpt)]
enum Subcommands {
    Build(BuildArgs),
}

#[derive(StructOpt)]
/// Build Servo
pub struct BuildArgs {
    #[structopt(long, short)]
    release: bool,
    #[structopt(long, short)]
    dev: bool,
    #[structopt(long, short)]
    jobs: Option<u32>,
    #[structopt(long, short)]
    no_package: bool,
    #[structopt(long, short)]
    verbose: bool,
    // TODO
    // structopt doesn't support `short` as multiple characters
    // the python code had -vv as the short version of this command
    #[structopt(long, short = "-z")]
    very_verbose: bool,
    #[structopt(long, short)]
    uwp: bool,
    #[structopt(long, short)]
    win_arm64: bool,
    params: Vec<String>,
}

fn main() {
    let repo_root = match get_repo_root() {
        Some(path) => path,
        None => {
            eprintln!("You must run mars within a servo repository.");
            exit(1)
        }
    };
    let args = Args::from_args();
    match args.cmd {
        Subcommands::Build(build_args) => build(repo_root, build_args, args.common),
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
