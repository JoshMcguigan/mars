use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};

mod config_file;

/// Represents a configuration item with all default values set.
pub struct Config {
    pub tools: ConfigTools,
    pub build: ConfigBuild,
    pub android: ConfigAndroid,
}

pub struct ConfigTools {
    pub cache_dir: PathBuf,
    pub cargo_home_dir: PathBuf,
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
    pub thinlto: bool,
}

pub struct ConfigAndroid {
    pub sdk: Option<String>,
    pub ndk: Option<String>,
    pub toolchain: Option<String>,
    pub platform: String,
    pub target: String,
    pub toolchain_prefix: String,
    pub arch: String,
    pub lib: String,
    pub toolchain_name: String,
}

impl Config {
    pub fn new(repo_root: &Path) -> Self {
        // In mach, this code is in command_base
        let config_path = {
            let mut config_path = repo_root.to_path_buf();
            config_path.push(".servobuild");

            config_path
        };
        let config_file = match read_to_string(config_path) {
            Ok(toml_str) => config_file::Config::from_str(&toml_str),
            Err(_) => config_file::Config::default(),
        };

        // TODO tranlation
        // resolve_relative paths
        let cache_dir = config_file.tools.cache_dir.unwrap_or_else(|| {
            env::var("SERVO_CACHE_DIR")
                .map(|path| PathBuf::from(path))
                .unwrap_or_else(|_| {
                    let mut path = repo_root.to_path_buf();
                    path.push(".servo");

                    path
                })
        });
        let cargo_home_dir = config_file.tools.cargo_home_dir.unwrap_or_else(|| {
            env::var("SERVO_CACHE_DIR")
                .map(|path| PathBuf::from(path))
                .unwrap_or_else(|_| {
                    let mut path = repo_root.to_path_buf();
                    path.push(".cargo");

                    path
                })
        });
        let use_rustup = config_file.tools.use_rustup.unwrap_or(true);
        let rustc_with_gold = config_file.tools.rustc_with_gold.unwrap_or_else(|| {
            match env::var("SERVO_RUSTC_WITH_GOLD") {
                Ok(val) => match val.as_str() {
                    "True" => true,
                    "False" => false,
                    _ => true,
                },
                Err(_) => true,
            }
        });

        let config_tools = ConfigTools {
            cache_dir,
            cargo_home_dir,
            use_rustup,
            rustc_with_gold,
            notify_command: config_file.tools.notify_command,
        };

        let config_build = ConfigBuild {
            mode: config_file.build.mode,
            android: config_file.build.android.unwrap_or(false),
            debug_assertions: config_file.build.debug_assertions.unwrap_or(false),
            debug_mozjs: config_file.build.debug_mozjs.unwrap_or(false),
            webgl_backtrace: config_file.build.webgl_backtrace.unwrap_or(false),
            dom_backtrace: config_file.build.dom_backtrace.unwrap_or(false),
            layout_2020: config_file.build.layout_2020.unwrap_or(false),
            ccache: config_file.build.ccache,
            rustflags: config_file.build.rustflags,
            incremental: config_file.build.incremental,
            thinlto: config_file.build.thinlto.unwrap_or(false),
        };

        let config_android = ConfigAndroid {
            sdk: config_file.android.sdk,
            ndk: config_file.android.ndk,
            toolchain: config_file.android.toolchain,
            platform: String::from("android-21"),
            target: String::from("armv7-linux-androideabi"),
            toolchain_prefix: String::from("arm-linux-androideabi"),
            arch: String::from("arm"),
            lib: String::from("armeabi-v7a"),
            toolchain_name: String::from("arm-linux-androideabi"),
        };

        Self {
            tools: config_tools,
            build: config_build,
            android: config_android,
        }
    }
}
