use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "the Servo build tool")]
pub struct Args {
    #[structopt(flatten)]
    pub common: CommonArgs,
    #[structopt(subcommand)]
    pub cmd: Subcommands,
}

#[derive(StructOpt)]
pub struct CommonArgs {
    #[structopt(long, short)]
    pub target: Option<String>,
    // TODO translation
    // mach limits this to "gstreamer" or "dummy", but defaults
    // to None. Can this be expressed in structopt?
    #[structopt(long)]
    pub media_stack: Option<String>,
    #[structopt(long)]
    pub android: bool,
    #[structopt(long)]
    pub magicleap: bool,
    #[structopt(long)]
    pub libsimpleservo: bool,
    #[structopt(long)]
    pub features: Vec<String>,
    #[structopt(long)]
    pub debug_mozjs: bool,
    #[structopt(long)]
    pub with_debug_assertions: bool,
    #[structopt(long)]
    pub with_frame_pointer: bool,
    #[structopt(long)]
    pub with_raqote: bool,
    #[structopt(long)]
    pub with_layout_2020: bool,
    #[structopt(long)]
    pub with_layout_2013: bool,
    #[structopt(long)]
    pub without_wgl: bool,
}

#[derive(StructOpt)]
pub enum Subcommands {
    Build(BuildArgs),
}

#[derive(StructOpt)]
/// Build Servo
pub struct BuildArgs {
    #[structopt(long, short)]
    pub release: bool,
    #[structopt(long, short)]
    pub dev: bool,
    #[structopt(long, short)]
    pub jobs: Option<u32>,
    #[structopt(long, short)]
    pub no_package: bool,
    #[structopt(long, short)]
    pub verbose: bool,
    // TODO
    // structopt doesn't support `short` as multiple characters
    // the python code had -vv as the short version of this command
    #[structopt(long, short = "-z")]
    pub very_verbose: bool,
    #[structopt(long, short)]
    pub uwp: bool,
    #[structopt(long, short)]
    pub win_arm64: bool,
    pub params: Vec<String>,
}
