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
}
