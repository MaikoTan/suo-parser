use std::process::Command;

fn main() {
    // Execute wasm-pack build
    let status = Command::new("wasm-pack")
        .arg("build")
        .arg("--target")
        .arg("web")
        .arg("--out-dir")
        .arg("pkg")
        .status()
        .expect("failed to execute wasm-pack");

    if !status.success() {
        panic!("wasm-pack build failed");
    }

    // Tell cargo to rerun this script if any of the source files change.
    println!("cargo:rerun-if-changed=src/");
}
