extern crate cc;
extern crate metadeps;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(t) => t,
        Err(e) => panic!("{} return the error {}", stringify!($e), e),
    })
}

fn main() {
    // Use the installed libcares if it is available.
    if metadeps::probe().is_ok() {
        return;
    }

    // Rerun if the c-ares source code has changed.
    println!("cargo:rerun-if-changed=c-ares");

    // MSVC builds are different.
    let target = env::var("TARGET").unwrap();
    if target.contains("msvc") {
        build_msvc(&target);
        return;
    }

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build = dst.join("build");
    let _ = fs::create_dir(&build);

    // Prepare.
    let src = env::current_dir().unwrap();
    run(
        Command::new("sh")
            .current_dir(&src.join("c-ares"))
            .arg("buildconf"),
    );

    // Configure.
    let cfg = cc::Build::new();
    let compiler = cfg.get_compiler();
    let mut cflags = OsString::new();
    for arg in compiler.args() {
        cflags.push(arg);
        cflags.push(" ");
    }

    let mut cmd = Command::new("sh");
    cmd.env("CFLAGS", &cflags)
        .env("CC", compiler.path())
        .current_dir(&build)
        .arg(format!("{}", src.join("c-ares/configure").display()))
        .arg("--enable-static=yes")
        .arg("--enable-shared=no")
        .arg("--enable-optimize")
        .arg(format!("--prefix={}", dst.display()));

    let host = env::var("HOST").unwrap();
    if target != host &&
       (!target.contains("windows") || !host.contains("windows")) {
        // NOTE GNU terminology
        // BUILD = machine where we are (cross) compiling curl
        // HOST = machine where the compiled curl will be used
        // TARGET = only relevant when compiling compilers
        if target.contains("windows") {
            // curl's configure can't parse `-windows-` triples when used
            // as `--host`s. In those cases we use this combination of
            // `host` and `target` that appears to do the right thing.
            cmd.arg(format!("--host={}", host));
            cmd.arg(format!("--target={}", target));
        } else {
            cmd.arg(format!("--build={}", host));
            cmd.arg(format!("--host={}", target));
        }
    }
    run(&mut cmd);

    // Compile.
    run(
        Command::new(make())
            .arg(format!("-j{}", env::var("NUM_JOBS").unwrap()))
            .current_dir(&build),
    );

    // Link to compiled library.
    println!("cargo:rustc-link-search={}/.libs", build.display());
    println!("cargo:rustc-link-lib=static=cares");
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(t!(cmd.status()).success());
}

fn make() -> &'static str {
    if cfg!(target_os = "freebsd") {
        "gmake"
    } else {
        "make"
    }
}

fn nmake(target: &str) -> Command {
    // cargo messes with the environment in a way that nmake does not like -
    // see https://github.com/rust-lang/cargo/issues/4156.  Explicitly remove
    // the unwanted variables.
    let mut cmd = cc::windows_registry::find(target, "nmake.exe").unwrap();
    cmd.env_remove("MAKEFLAGS").env_remove("MFLAGS");
    cmd
}

fn build_msvc(target: &str) {
    // Prepare.
    let src = env::current_dir().unwrap();
    let c_ares_dir = &src.join("c-ares");
    run(
        Command::new("cmd")
            .current_dir(c_ares_dir)
            .arg("/c")
            .arg("buildconf.bat"),
    );

    // Compile.
    let mut cmd = nmake(target);
    cmd.current_dir(c_ares_dir);
    cmd.args(&["/f", "Makefile.msvc", "CFG=lib-release", "c-ares"]);
    run(&mut cmd);

    // Install library.
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let build = dst.join("build");
    let mut cmd = nmake(target);
    cmd.current_dir(c_ares_dir);
    cmd.args(&["/f", "Makefile.msvc", "/a", "CFG=lib-release", "install"]);
    cmd.env("INSTALL_DIR", format!("{}", build.display()));
    run(&mut cmd);

    // Link to compiled library.
    println!("cargo:rustc-link-search={}/lib", build.display());
    println!("cargo:rustc-link-lib=static=libcares");
}
