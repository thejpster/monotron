use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Put the git version into the binary using
    // and environment variable.
    let git_desc = Command::new("git")
        .args(&["describe", "--all", "--tags", "--dirty"])
        .output()
        .unwrap();
    println!(
        "cargo:rustc-env=GIT_DESCRIBE={}",
        String::from_utf8_lossy(&git_desc.stdout)
    );
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}
