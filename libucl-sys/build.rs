use std::process::Command;
use std::env;
use std::io::ErrorKind;
use std::path::PathBuf;

fn main() {
    let mut cflags = env::var("CFLAGS").unwrap_or(String::new());
    cflags.push_str(" -fPIC");

    let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let dst = PathBuf::from(&env::var("OUT_DIR").unwrap());

    let mut cmd = Command::new("./autogen.sh");
    cmd.current_dir(&src.join("libucl"));

    run(&mut cmd, "autogen.sh");

    let mut cmd = Command::new("./configure");
    cmd.current_dir(&src.join("libucl"));
    cmd.arg(&format!("--prefix={}", dst.display()));
    run(cmd.arg("--enable-urls")
           .arg("--enable-regex")
           .arg("--disable-shared")
           .arg("--with-pic"), "configure");

    let mut cmd = Command::new("make");
    cmd.env("CFLAGS", cflags);
    cmd.current_dir(&src.join("libucl"));
    run(cmd.arg("install"), "make");

    println!("cargo:rustc-link-lib=static=ucl");
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}
