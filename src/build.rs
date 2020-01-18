use std::{collections::HashMap, fs::read_to_string, path::PathBuf, process::{Command, exit}};
use crate::{BuildArgs, CommonArgs};

pub fn build(repo_root: PathBuf, build_args: BuildArgs, common_args: CommonArgs) {
    let BuildArgs {
        mut dev,
        mut release,
        jobs,
        no_package,
        verbose,
        very_verbose,
        mut uwp,
        win_arm64,
        params,
    } = build_args;
    let CommonArgs {
        mut target,
        mut features,
        mut android,
        magicleap,
        media_stack,
        libsimpleservo,
        ..
    } = common_args;

    // Force the UWP-enabled target if the convenience UWP flags are passed.
    if uwp && target.is_none() {
        if win_arm64 {
            target = Some(String::from("aarch64-uwp-windows-msvc"));
        } else {
            target = Some(String::from("x86_64-uwp-windows-msvc"));
        }
    }

    let mut opts = params;

    // TODO - translation
    // The lines that follow were originally defined in
    // CommandBase.pick_target_triple. For now I've inlined them.
    if !android {
        // TODO - translation
        // skipping parsing config file for now
        // android = self.config["build"]["android"]
    }
    match (&target, android) {
        (Some(target), true) => assert!(handle_android_target(&target)),
        (None, true) => {
            // TODO - translation
            // skipping parsing config file for now
            // target = self.config["android"]["target"]
        },
        _ => {},
    }
    if magicleap && target.is_none() {
        target = Some(String::from("aarch64-linux-android"));
    }
    match (&target, android, magicleap) {
        (Some(target), false, false) => android = handle_android_target(&target),
        _ => {},
    }

    if !uwp {
        uwp = if let Some(target) = &target {
            target.contains("uwp")
        } else {
            false
        }
    }

    features.append(&mut pick_media_stack(media_stack, &target));
    let mut target_path = get_target_dir(&repo_root);
    let mut base_path = get_target_dir(&repo_root);
    if android {
        target_path.push("android");
        base_path = target_path.clone();
        base_path.push(target.clone().unwrap_or(String::new()));
    } else if magicleap {
        target_path.push("magicleap");
        base_path = target_path.clone();
        base_path.push(target.clone().unwrap_or(String::new()));
    }
    let mut release_path = base_path.clone();
    release_path.push("release");
    release_path.push("servo");

    let mut dev_path = base_path.clone();
    dev_path.push("debug");
    dev_path.push("servo");

    if !(release || dev) {
        // TODO translation
        // if self.config["build"]["mode"] == "dev":
        //     dev = True
        // elif self.config["build"]["mode"] == "release":
        //     release = True
        if release_path.exists() && !dev_path.exists() {
            release = true;
        } else if !release_path.exists() && dev_path.exists() {
            dev = true;
        } else {
            println!("Please specify either --dev (-d) for a development build, or --release (-r) for an optimized build.");

            exit(1);
        }
    }

    if release && dev {
        println!("Please specify either --dev or --release.");
        exit(1);
    }

    let servo_path;
    if release {
        opts.push(String::from("--release"));
        servo_path = &release_path;
    } else {
        servo_path = &dev_path;
    }

    if let Some(jobs) = jobs {
        opts.push(String::from("-j"));
        opts.push(format!("{}", jobs));
    }

    if verbose {
        opts.push(String::from("-v"));
    }
    if very_verbose {
        opts.push(String::from("-vv"));
    }

    let mut env = build_env(&target, true, uwp, &features);
    ensure_bootstrapped(&target);
    ensure_clobbered();

    // TODO translation
    // build_start = time()
    
    let host = host_triple();
    let target_triple = target.clone().unwrap_or(host.clone());

    if host.contains("apple-darwin") && target_triple == host {
        // if 'CXXFLAGS' not in env:
        //     env['CXXFLAGS'] = ''
        // env["CXXFLAGS"] += "-mmacosx-version-min=10.10"
        unimplemented!();
    }

    if host.contains("windows") {
        // vs_dirs = vs_dirs()
        unimplemented!();
    }

    if host != target_triple && target_triple.contains("windows") {
        unimplemented!();
        // if os.environ.get('VisualStudioVersion'):
        //     print("Can't cross-compile for Windows inside of a Visual Studio shell.\n"
        //           "Please run `python mach build [arguments]` to bypass automatic "
        //           "Visual Studio shell.")
        //     sys.exit(1)
        // vcinstalldir = vs_dirs['vcdir']
        // if not os.path.exists(vcinstalldir):
        //     print("Can't find Visual C++ %s installation at %s." % (vs_dirs['vs_version'], vcinstalldir))
        //     sys.exit(1)

        // env['PKG_CONFIG_ALLOW_CROSS'] = "1"
    }
    if uwp {
        unimplemented!();
        // # Ensure libstd is ready for the new UWP target.
        // check_call(["rustup", "component", "add", "rust-src"])
        // env['RUST_SYSROOT'] = path.expanduser('~\\.xargo')

        // # Don't try and build a desktop port.
        // libsimpleservo = True

        // arches = {
        //     "aarch64": {
        //         "angle": "arm64",
        //         "gst": "ARM64",
        //         "gst_root": "arm64",
        //     },
        //     "x86_64": {
        //         "angle": "x64",
        //         "gst": "X86_64",
        //         "gst_root": "x64",
        //     },
        // }
        // arch = arches.get(target_triple.split('-')[0])
        // if not arch:
        //     print("Unsupported UWP target.")
        //     sys.exit(1)

        // # Ensure that the NuGet ANGLE package containing libEGL is accessible
        // # to the Rust linker.
        // append_to_path_env(angle_root(target_triple, env), env, "LIB")

        // # Don't want to mix non-UWP libraries with vendored UWP libraries.
        // if "gstreamer" in env['LIB']:
        //     print("Found existing GStreamer library path in LIB. Please remove it.")
        //     sys.exit(1)

        // # Override any existing GStreamer installation with the vendored libraries.
        // env["GSTREAMER_1_0_ROOT_" + arch['gst']] = path.join(
        //     self.msvc_package_dir("gstreamer-uwp"), arch['gst_root']
        // )
        // env["PKG_CONFIG_PATH"] = path.join(
        //     self.msvc_package_dir("gstreamer-uwp"), arch['gst_root'],
        //     "lib", "pkgconfig"
        // )
    }

    // Ensure that GStreamer libraries are accesible when linking.
    if target_triple.contains("windows") {
        unimplemented!();
        // gst_root = gstreamer_root(target_triple, env)
        // if gst_root:
        //     append_to_path_env(os.path.join(gst_root, "lib"), env, "LIB")
    }

    if android {
        unimplemented!();
        // if "ANDROID_NDK" not in env:
        //     print("Please set the ANDROID_NDK environment variable.")
        //     sys.exit(1)
        // if "ANDROID_SDK" not in env:
        //     print("Please set the ANDROID_SDK environment variable.")
        //     sys.exit(1)

        // android_platform = self.config["android"]["platform"]
        // android_toolchain_name = self.config["android"]["toolchain_name"]
        // android_toolchain_prefix = self.config["android"]["toolchain_prefix"]
        // android_lib = self.config["android"]["lib"]
        // android_arch = self.config["android"]["arch"]

        // # Build OpenSSL for android
        // env["OPENSSL_VERSION"] = "1.0.2k"
        // make_cmd = ["make"]
        // if jobs is not None:
        //     make_cmd += ["-j" + jobs]
        // openssl_dir = path.join(target_path, target, "native", "openssl")
        // if not path.exists(openssl_dir):
        //     os.makedirs(openssl_dir)
        // shutil.copy(path.join(self.android_support_dir(), "openssl.makefile"), openssl_dir)
        // shutil.copy(path.join(self.android_support_dir(), "openssl.sh"), openssl_dir)

        // # Check if the NDK version is 15
        // if not os.path.isfile(path.join(env["ANDROID_NDK"], 'source.properties')):
        //     print("ANDROID_NDK should have file `source.properties`.")
        //     print("The environment variable ANDROID_NDK may be set at a wrong path.")
        //     sys.exit(1)
        // with open(path.join(env["ANDROID_NDK"], 'source.properties')) as ndk_properties:
        //     lines = ndk_properties.readlines()
        //     if lines[1].split(' = ')[1].split('.')[0] != '15':
        //         print("Currently only support NDK 15. Please re-run `./mach bootstrap-android`.")
        //         sys.exit(1)

        // env["RUST_TARGET"] = target
        // with cd(openssl_dir):
        //     status = call(
        //         make_cmd + ["-f", "openssl.makefile"],
        //         env=env,
        //         verbose=verbose)
        //     if status:
        //         return status
        // openssl_dir = path.join(openssl_dir, "openssl-{}".format(env["OPENSSL_VERSION"]))
        // env['OPENSSL_LIB_DIR'] = openssl_dir
        // env['OPENSSL_INCLUDE_DIR'] = path.join(openssl_dir, "include")
        // env['OPENSSL_STATIC'] = 'TRUE'
        // # Android builds also require having the gcc bits on the PATH and various INCLUDE
        // # path munging if you do not want to install a standalone NDK. See:
        // # https://dxr.mozilla.org/mozilla-central/source/build/autoconf/android.m4#139-161
        // os_type = platform.system().lower()
        // if os_type not in ["linux", "darwin"]:
        //     raise Exception("Android cross builds are only supported on Linux and macOS.")
        // cpu_type = platform.machine().lower()
        // host_suffix = "unknown"
        // if cpu_type in ["i386", "i486", "i686", "i768", "x86"]:
        //     host_suffix = "x86"
        // elif cpu_type in ["x86_64", "x86-64", "x64", "amd64"]:
        //     host_suffix = "x86_64"
        // host = os_type + "-" + host_suffix

        // host_cc = env.get('HOST_CC') or _get_exec_path(["clang"]) or _get_exec_path(["gcc"])
        // host_cxx = env.get('HOST_CXX') or _get_exec_path(["clang++"]) or _get_exec_path(["g++"])

        // llvm_toolchain = path.join(env['ANDROID_NDK'], "toolchains", "llvm", "prebuilt", host)
        // gcc_toolchain = path.join(env['ANDROID_NDK'], "toolchains",
        //                           android_toolchain_prefix + "-4.9", "prebuilt", host)
        // gcc_libs = path.join(gcc_toolchain, "lib", "gcc", android_toolchain_name, "4.9.x")

        // env['PATH'] = (path.join(llvm_toolchain, "bin") + ':' + env['PATH'])
        // env['ANDROID_SYSROOT'] = path.join(env['ANDROID_NDK'], "sysroot")
        // support_include = path.join(env['ANDROID_NDK'], "sources", "android", "support", "include")
        // cpufeatures_include = path.join(env['ANDROID_NDK'], "sources", "android", "cpufeatures")
        // cxx_include = path.join(env['ANDROID_NDK'], "sources", "cxx-stl",
        //                         "llvm-libc++", "include")
        // clang_include = path.join(llvm_toolchain, "lib64", "clang", "3.8", "include")
        // cxxabi_include = path.join(env['ANDROID_NDK'], "sources", "cxx-stl",
        //                            "llvm-libc++abi", "include")
        // sysroot_include = path.join(env['ANDROID_SYSROOT'], "usr", "include")
        // arch_include = path.join(sysroot_include, android_toolchain_name)
        // android_platform_dir = path.join(env['ANDROID_NDK'], "platforms", android_platform, "arch-" + android_arch)
        // arch_libs = path.join(android_platform_dir, "usr", "lib")
        // clang_include = path.join(llvm_toolchain, "lib64", "clang", "5.0", "include")
        // android_api = android_platform.replace('android-', '')
        // env['HOST_CC'] = host_cc
        // env['HOST_CXX'] = host_cxx
        // env['HOST_CFLAGS'] = ''
        // env['HOST_CXXFLAGS'] = ''
        // env['CC'] = path.join(llvm_toolchain, "bin", "clang")
        // env['CPP'] = path.join(llvm_toolchain, "bin", "clang") + " -E"
        // env['CXX'] = path.join(llvm_toolchain, "bin", "clang++")
        // env['ANDROID_TOOLCHAIN'] = gcc_toolchain
        // env['ANDROID_TOOLCHAIN_DIR'] = gcc_toolchain
        // env['ANDROID_VERSION'] = android_api
        // env['ANDROID_PLATFORM_DIR'] = android_platform_dir
        // env['GCC_TOOLCHAIN'] = gcc_toolchain
        // gcc_toolchain_bin = path.join(gcc_toolchain, android_toolchain_name, "bin")
        // env['AR'] = path.join(gcc_toolchain_bin, "ar")
        // env['RANLIB'] = path.join(gcc_toolchain_bin, "ranlib")
        // env['OBJCOPY'] = path.join(gcc_toolchain_bin, "objcopy")
        // env['YASM'] = path.join(env['ANDROID_NDK'], 'prebuilt', host, 'bin', 'yasm')
        // # A cheat-sheet for some of the build errors caused by getting the search path wrong...
        // #
        // # fatal error: 'limits' file not found
        // #   -- add -I cxx_include
        // # unknown type name '__locale_t' (when running bindgen in mozjs_sys)
        // #   -- add -isystem sysroot_include
        // # error: use of undeclared identifier 'UINTMAX_C'
        // #   -- add -D__STDC_CONSTANT_MACROS
        // #
        // # Also worth remembering: autoconf uses C for its configuration,
        // # even for C++ builds, so the C flags need to line up with the C++ flags.
        // env['CFLAGS'] = ' '.join([
        //     "--target=" + target,
        //     "--sysroot=" + env['ANDROID_SYSROOT'],
        //     "--gcc-toolchain=" + gcc_toolchain,
        //     "-isystem", sysroot_include,
        //     "-I" + arch_include,
        //     "-B" + arch_libs,
        //     "-L" + arch_libs,
        //     "-D__ANDROID_API__=" + android_api,
        // ])
        // env['CXXFLAGS'] = ' '.join([
        //     "--target=" + target,
        //     "--sysroot=" + env['ANDROID_SYSROOT'],
        //     "--gcc-toolchain=" + gcc_toolchain,
        //     "-I" + cpufeatures_include,
        //     "-I" + cxx_include,
        //     "-I" + clang_include,
        //     "-isystem", sysroot_include,
        //     "-I" + cxxabi_include,
        //     "-I" + clang_include,
        //     "-I" + arch_include,
        //     "-I" + support_include,
        //     "-L" + gcc_libs,
        //     "-B" + arch_libs,
        //     "-L" + arch_libs,
        //     "-D__ANDROID_API__=" + android_api,
        //     "-D__STDC_CONSTANT_MACROS",
        //     "-D__NDK_FPABI__=",
        // ])
        // env['CPPFLAGS'] = ' '.join([
        //     "--target=" + target,
        //     "--sysroot=" + env['ANDROID_SYSROOT'],
        //     "-I" + arch_include,
        // ])
        // env["NDK_ANDROID_VERSION"] = android_api
        // env["ANDROID_ABI"] = android_lib
        // env["ANDROID_PLATFORM"] = android_platform
        // env["NDK_CMAKE_TOOLCHAIN_FILE"] = path.join(env['ANDROID_NDK'], "build", "cmake", "android.toolchain.cmake")
        // env["CMAKE_TOOLCHAIN_FILE"] = path.join(self.android_support_dir(), "toolchain.cmake")
        // # Set output dir for gradle aar files
        // aar_out_dir = self.android_aar_dir()
        // if not os.path.exists(aar_out_dir):
        //     os.makedirs(aar_out_dir)
        // env["AAR_OUT_DIR"] = aar_out_dir
        // # GStreamer and its dependencies use pkg-config and this flag is required
        // # to make it work in a cross-compilation context.
        // env["PKG_CONFIG_ALLOW_CROSS"] = '1'
        // # Build the name of the package containing all GStreamer dependencies
        // # according to the build target.
        // gst_lib = "gst-build-{}".format(self.config["android"]["lib"])
        // gst_lib_zip = "gstreamer-{}-1.16.0-20190517-095630.zip".format(self.config["android"]["lib"])
        // gst_dir = os.path.join(target_path, "gstreamer")
        // gst_lib_path = os.path.join(gst_dir, gst_lib)
        // pkg_config_path = os.path.join(gst_lib_path, "pkgconfig")
        // env["PKG_CONFIG_PATH"] = pkg_config_path
        // if not os.path.exists(gst_lib_path):
        //     # Download GStreamer dependencies if they have not already been downloaded
        //     # This bundle is generated with `libgstreamer_android_gen`
        //     # Follow these instructions to build and deploy new binaries
        //     # https://github.com/servo/libgstreamer_android_gen#build
        //     print("Downloading GStreamer dependencies")
        //     gst_url = "https://servo-deps.s3.amazonaws.com/gstreamer/%s" % gst_lib_zip
        //     print(gst_url)
        //     urllib.request.urlretrieve(gst_url, gst_lib_zip)
        //     zip_ref = zipfile.ZipFile(gst_lib_zip, "r")
        //     zip_ref.extractall(gst_dir)
        //     os.remove(gst_lib_zip)

        //     # Change pkgconfig info to make all GStreamer dependencies point
        //     # to the libgstreamer_android.so bundle.
        //     for each in os.listdir(pkg_config_path):
        //         if each.endswith('.pc'):
        //             print("Setting pkgconfig info for %s" % each)
        //             pc = os.path.join(pkg_config_path, each)
        //             expr = "s#libdir=.*#libdir=%s#g" % gst_lib_path
        //             subprocess.call(["perl", "-i", "-pe", expr, pc])
    }

    if magicleap {
        // if platform.system() not in ["Darwin"]:
        //     raise Exception("Magic Leap builds are only supported on macOS. "
        //                     "If you only wish to test if your code builds, "
        //                     "run ./mach build -p libmlservo.")

        // ml_sdk = env.get("MAGICLEAP_SDK")
        // if not ml_sdk:
        //     raise Exception("Magic Leap builds need the MAGICLEAP_SDK environment variable")
        // if not os.path.exists(ml_sdk):
        //     raise Exception("Path specified by MAGICLEAP_SDK does not exist.")

        // ml_support = path.join(self.get_top_dir(), "support", "magicleap")

        // # We pretend to be an Android build
        // env.setdefault("ANDROID_VERSION", "21")
        // env.setdefault("ANDROID_NDK", env["MAGICLEAP_SDK"])
        // env.setdefault("ANDROID_NDK_VERSION", "16.0.0")
        // env.setdefault("ANDROID_PLATFORM_DIR", path.join(env["MAGICLEAP_SDK"], "lumin"))
        // env.setdefault("ANDROID_TOOLCHAIN_DIR", path.join(env["MAGICLEAP_SDK"], "tools", "toolchains"))
        // env.setdefault("ANDROID_CLANG", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "clang"))

        // # A random collection of search paths
        // env.setdefault("STLPORT_LIBS", " ".join([
        //     "-L" + path.join(env["MAGICLEAP_SDK"], "lumin", "stl", "libc++-lumin", "lib"),
        //     "-lc++"
        // ]))
        // env.setdefault("STLPORT_CPPFLAGS", " ".join([
        //     "-I" + path.join(env["MAGICLEAP_SDK"], "lumin", "stl", "libc++-lumin", "include")
        // ]))
        // env.setdefault("CPPFLAGS", " ".join([
        //     "--no-standard-includes",
        //     "--sysroot=" + env["ANDROID_PLATFORM_DIR"],
        //     "-I" + path.join(env["ANDROID_PLATFORM_DIR"], "usr", "include"),
        //     "-isystem" + path.join(env["ANDROID_TOOLCHAIN_DIR"], "lib64", "clang", "3.8", "include"),
        // ]))
        // env.setdefault("CFLAGS", " ".join([
        //     env["CPPFLAGS"],
        //     "-L" + path.join(env["ANDROID_TOOLCHAIN_DIR"], "lib", "gcc", target, "4.9.x"),
        // ]))
        // env.setdefault("CXXFLAGS", " ".join([
        //     # Sigh, Angle gets confused if there's another EGL around
        //     "-I./gfx/angle/checkout/include",
        //     env["STLPORT_CPPFLAGS"],
        //     env["CFLAGS"]
        // ]))

        // # The toolchain commands
        // env.setdefault("AR", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-ar"))
        // env.setdefault("AS", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-clang"))
        // env.setdefault("CC", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-clang"))
        // env.setdefault("CPP", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-clang -E"))
        // env.setdefault("CXX", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-clang++"))
        // env.setdefault("LD", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-ld"))
        // env.setdefault("OBJCOPY", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-objcopy"))
        // env.setdefault("OBJDUMP", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-objdump"))
        // env.setdefault("RANLIB", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-ranlib"))
        // env.setdefault("STRIP", path.join(env["ANDROID_TOOLCHAIN_DIR"], "bin", "aarch64-linux-android-strip"))

        // # Undo all of that when compiling build tools for the host
        // env.setdefault("HOST_CFLAGS", "")
        // env.setdefault("HOST_CXXFLAGS", "")
        // env.setdefault("HOST_CC", "/usr/local/opt/llvm/bin/clang")
        // env.setdefault("HOST_CXX", "/usr/local/opt/llvm/bin/clang++")
        // env.setdefault("HOST_LD", "ld")

        // # Some random build configurations
        // env.setdefault("HARFBUZZ_SYS_NO_PKG_CONFIG", "1")
        // env.setdefault("PKG_CONFIG_ALLOW_CROSS", "1")
        // env.setdefault("CMAKE_TOOLCHAIN_FILE", path.join(ml_support, "toolchain.cmake"))
        // env.setdefault("_LIBCPP_INLINE_VISIBILITY", "__attribute__((__always_inline__))")

        // # The Open SSL configuration
        // env.setdefault("OPENSSL_DIR", path.join(target_path, target, "native", "openssl"))
        // env.setdefault("OPENSSL_VERSION", "1.0.2k")
        // env.setdefault("OPENSSL_STATIC", "1")

        // # GStreamer configuration
        // env.setdefault("GSTREAMER_DIR", path.join(target_path, target, "native", "gstreamer-1.16.0"))
        // env.setdefault("GSTREAMER_URL", "https://servo-deps.s3.amazonaws.com/gstreamer/gstreamer-magicleap-1.16.0-20190823-104505.tgz")
        // env.setdefault("PKG_CONFIG_PATH", path.join(env["GSTREAMER_DIR"], "system", "lib64", "pkgconfig"))

        // # Override the linker set in .cargo/config
        // env.setdefault("CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER", path.join(ml_support, "fake-ld.sh"))

        // # Only build libmlservo
        // opts += ["--package", "libmlservo"]

        // # Download and build OpenSSL if necessary
        // status = call(path.join(ml_support, "openssl.sh"), env=env, verbose=verbose)
        // if status:
        //     return status

        // # Download prebuilt Gstreamer if necessary
        // if not os.path.exists(path.join(env["GSTREAMER_DIR"], "system")):
        //     if not os.path.exists(env["GSTREAMER_DIR"] + ".tgz"):
        //         check_call([
        //             'curl',
        //             '-L',
        //             '-f',
        //             '-o', env["GSTREAMER_DIR"] + ".tgz",
        //             env["GSTREAMER_URL"],
        //         ])
        //     check_call([
        //         'mkdir',
        //         '-p',
        //         env["GSTREAMER_DIR"],
        //     ])
        //     check_call([
        //         'tar',
        //         'xzf',
        //         env["GSTREAMER_DIR"] + ".tgz",
        //         '-C', env["GSTREAMER_DIR"],
        //     ])
    }
    // https://internals.rust-lang.org/t/exploring-crate-graph-build-times-with-cargo-build-ztimings/10975
    // Prepend so that e.g. `-Ztimings` (which means `-Ztimings=info,html`)
    // given on the command line can override it
    opts.insert(0, String::from("-Ztimings=info"));

    if very_verbose {
        println!("Calling cargo build");
        println!("{:?}", opts);
        for (k, v) in &env {
            println!("{} {}", k, v);
        }
    }

    // TODO translation
    // for now I've only translated the else block
    // if sys.platform == "win32":
    //     env.setdefault("CC", "clang-cl.exe")
    //     env.setdefault("CXX", "clang-cl.exe")
    //     if uwp:
    //         env.setdefault("CFLAGS", "")
    //         env.setdefault("CXXFLAGS", "")
    //         env["CFLAGS"] += " -DWINAPI_FAMILY=WINAPI_FAMILY_APP"
    //         env["CXXFLAGS"] += " -DWINAPI_FAMILY=WINAPI_FAMILY_APP"
    // else:
    //     env.setdefault("CC", "clang")
    //     env.setdefault("CXX", "clang++")
    if !env.contains_key("CC") {
        env.insert(String::from("CC"), String::from("clang"));
    }
    if !env.contains_key("CXX") {
        env.insert(String::from("CXX"), String::from("clang++"));
    }
    let status = run_cargo_build_like_command(
        &repo_root,
        "build", opts, env, verbose,
        target, android, magicleap, libsimpleservo, uwp,
        features, // TODO translation **kwargs
    );

    // elapsed = time() - build_start
    // TODO continue translation
    //
    // TODO compare mach/mars args to cargo build to
    // figure out why mars build fails

}

fn handle_android_target(target: &str) -> bool {
    unimplemented!();
}

fn pick_media_stack(
    media_stack: Option<String>,
    target: &Option<String>
    ) -> Vec<String> {
    let media_stack = media_stack.unwrap_or_else(|| {
        let use_gstreamer = match target {
            Some(target) => {
                let android = target.contains("arm7") &&
                    target.contains("android");
                let x86_64 = target.contains("x86_64");

                android || x86_64
            },
            None => true,
        };

        if use_gstreamer {
            String::from("gstreamer")
        } else {
            String::from("dummy")
        }
    });

    vec![format!("media-{}", media_stack)]
}

fn get_target_dir(repo_root: &PathBuf) -> PathBuf {
    // def get_target_dir(self):
    //     if "CARGO_TARGET_DIR" in os.environ:
    //         return os.environ["CARGO_TARGET_DIR"]
    //     else:
    //         return path.join(self.context.topdir, "target")

    // TODO translation
    let mut path = repo_root.clone();
    path.push("target");

    path
}

fn build_env(target: &Option<String>, is_build: bool, uwp: bool, features: &Vec<String>) -> HashMap<String, String> {
    // TODO translation
    HashMap::new()
}

fn ensure_clobbered() {
    // TODO translation
}

fn ensure_bootstrapped(target: &Option<String>) {
    // TODO translation
}

fn host_triple() -> String {
    // os_type = host_platform()
    // cpu_type = platform.machine().lower()
    // if cpu_type in ["i386", "i486", "i686", "i768", "x86"]:
    //     cpu_type = "i686"
    // elif cpu_type in ["x86_64", "x86-64", "x64", "amd64"]:
    //     cpu_type = "x86_64"
    // elif cpu_type == "arm":
    //     cpu_type = "arm"
    // elif cpu_type == "aarch64":
    //     cpu_type = "aarch64"
    // else:
    //     cpu_type = "unknown"

    // return "{}-{}".format(cpu_type, os_type)
    let cpu_type = "x86_64";
    let os_type = "linux";
    format!("{}-{}", cpu_type, os_type)
}

fn run_cargo_build_like_command(
    repo_root: &PathBuf,
    command: &str, mut cargo_args: Vec<String>,
    env: HashMap<String, String>, verbose: bool,
    target: Option<String>, android: bool, magicleap: bool, libsimpleservo: bool, uwp: bool,
    mut features: Vec<String>
) {
    // TODO translation - these used to be optional args
    let debug_mozjs = false; 
    let with_debug_assertions = false;
    let with_frame_pointer = false; 
    let with_raqote = false; 
    let without_wgl = false;
    let with_layout_2020 = false;
    let with_layout_2013 = false;
    let media_stack = Option::<String>::None;
    // end optional args

    // TODO translation
    // env = env or self.build_env()
    // target, android = self.pick_target_triple(target, android, magicleap)

    let mut args = vec![];
    let port = if libsimpleservo || android {
        let api = if android {
            "jniapi"
        } else {
            "capi"
        };
        // TODO make this path join cross platform
        format!("libsimpleservo/{}", api)
    } else {
        String::from("glutin")
    };
    args.push(String::from("--manifest-path"));
    let manifest_path = {
        let mut manifest_path = repo_root.clone();
        manifest_path.push("ports");
        manifest_path.push(port);
        manifest_path.push("Cargo.toml");
        manifest_path.to_str()
            .expect("failed to convert manifest path to string")
            .to_owned()
    };
    args.push(manifest_path);

    if let Some(target) = target {
        args.push(String::from("--target"));
        args.push(target);
    }

    // TODO translation
    // the existing python code had this comment which makes me think it takes features as &mut
    // (does code beyond this method reference features list?)
    // # If we're passed a list, mutate it even if it's empty

    // TODO translation
    // if self.config["build"]["debug-mozjs"] or debug_mozjs:
    //     features.append("debugmozjs")
    if !magicleap {
        features.push(String::from("native-bluetooth"));
    }
    // if uwp:
    //     features.append("canvas2d-raqote")
    //     features.append("no-wgl")
    //     features.append("uwp")
    // else:
    //     # Non-UWP builds provide their own libEGL via mozangle.
    //     the append egl below was in this else block
    features.push(String::from("egl"));

    // TODO translation
    // I only translated the else case here
    // if with_raqote and "canvas2d-azure" not in features:
    //     features.append("canvas2d-raqote")
    // elif "canvas2d-azure" not in features:
    features.push(String::from("canvas2d-raqote"));

    // TODO translation I only translated the else case
    // if with_layout_2020 or (self.config["build"]["layout-2020"] and not with_layout_2013):
    //     features.append("layout-2020")
    // elif "layout-2020" not in features:
    features.push(String::from("layout-2013"));

    // TODO translation
    // all these default to unused, so skipping them for now
    // if with_frame_pointer:
    //     env['RUSTFLAGS'] = env.get('RUSTFLAGS', "") + " -C force-frame-pointers=yes"
    //     features.append("profilemozjs")

    // if without_wgl:
    //     features.append("no-wgl")

    // if self.config["build"]["webgl-backtrace"]:
    //     features.append("webgl-backtrace")

    // if self.config["build"]["dom-backtrace"]:
    //     features.append("dom-backtrace")

    // if with_debug_assertions or self.config["build"]["debug-assertions"]:
    //     env['RUSTFLAGS'] = env.get('RUSTFLAGS', "") + " -C debug_assertions"

    assert!(!cargo_args.contains(&String::from("--features")));
    args.push(String::from("--features"));
    args.push(features.join(" "));

    // TODO translation
    // only translated the else case here
    // if target and 'uwp' in target:
    //     return call(["xargo", command] + args + cargo_args, env=env, verbose=verbose)
    // else:
    let mut full_args = vec![String::from(command)];
    full_args.append(&mut args);
    full_args.append(&mut cargo_args);
    return call_rustup_run(repo_root, "cargo", full_args, env, verbose);
}

// TODO translation originally this used **kwargs to pass
// arbitrary args to the call method
fn call_rustup_run(repo_root: &PathBuf, command: &str, mut args: Vec<String>, env: HashMap<String, String>, verbose: bool) {
    // BIN_SUFFIX = ".exe" if sys.platform == "win32" else ""
    let mut bin_suffix = String::new();
    // TODO translation
    // skipping config parsing for now
    // default config is use-rustup=true
    let use_rustup = true;
    let (command, args) = if use_rustup {
        let rustup_command = "rustup";
        let mut rustup_args = vec![
            String::from("run"),
            String::from("--install"),
            rust_toolchain(repo_root),
        ];
        args.insert(0, String::from(command));
        rustup_args.extend(args);

        (rustup_command, rustup_args)
    } else {
        (command, args)
    };

    let command = format!("{}{}", command, bin_suffix);

    call(command, args, env, verbose)
}

fn rust_toolchain(repo_root: &PathBuf) -> String {
    // TODO translation
    // mach caches this function call to only read the file once

    let mut path = repo_root.clone();
    path.push("rust-toolchain");
    let toolchain = read_to_string(path).expect("Failed to read rust_toolchain file.")
        .trim().to_owned();

    // if windows
    // toolchain += "-x86_64-pc-windows-msvc";
    toolchain
}

/// Wrap std::process::Command printing the command if verbose=true.
fn call(command: String, args: Vec<String>, env: HashMap<String, String>, verbose: bool) {
    if verbose {
        println!("{} {:?}", command, args);
    };
    // TODO translation
    // the original code calls normalize_env here
    // also sets shell=true for windows users in the subprocess.call
    Command::new(command)
        .args(args)
        .envs(env)
        .status();
}
