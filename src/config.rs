pub struct Config {
    tools: ConfigTools,
    build: ConfigBuild,
    android: ConfigAndroid,
}

pub struct ConfigTools {
    pub use_rustup: bool,
    pub rustc_with_gold: bool,
    pub notify_command: Option<String>,
}

pub struct ConfigBuild {
    pub mode: Option<String>,
    pub android: bool,
    pub debug_assertions: bool,
    pub debug_mozjs: bool,
    pub webgl_backtrace: bool,
    pub dom_backtrace: bool,
    pub layout_2020: bool,
    pub ccache: Option<String>,
    pub rustflags: Option<String>,
    pub incremental: Option<bool>,
    pub thinlto: Option<bool>,
}

pub struct ConfigAndroid {
    pub sdk: Option<String>,
    pub ndk: Option<String>,
    pub toolchain: Option<String>,
    pub platform: Option<String>,
}
