use std::path::PathBuf;

use serde::Deserialize;

/// Representation of a valid `.servobuild` file.
///
/// All sections of the file are optional, and all fields
/// in each section are optional.
#[derive(Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    pub tools: ConfigTools,
    #[serde(default)]
    pub build: ConfigBuild,
    #[serde(default)]
    pub android: ConfigAndroid,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigTools {
    pub cache_dir: Option<PathBuf>,
    pub cargo_home_dir: Option<PathBuf>,
    pub use_rustup: Option<bool>,
    pub rustc_with_gold: Option<bool>,
    pub notify_command: Option<String>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigBuild {
    pub mode: Option<String>,
    pub android: Option<bool>,
    pub debug_assertions: Option<bool>,
    pub debug_mozjs: Option<bool>,
    pub webgl_backtrace: Option<bool>,
    pub dom_backtrace: Option<bool>,
    pub layout_2020: Option<bool>,
    pub ccache: Option<String>,
    pub rustflags: Option<String>,
    pub incremental: Option<bool>,
    pub thinlto: Option<bool>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigAndroid {
    pub sdk: Option<String>,
    pub ndk: Option<String>,
    pub toolchain: Option<String>,
    pub platform: Option<String>,
}

impl Config {
    pub fn from_str(s: &str) -> Self {
        toml::from_str(s)
            .unwrap_or(Default::default())
    }
}
