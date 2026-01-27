use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("window.ui");
    let input_path = "window.blp";

    println!("cargo:rerun-if-changed={}", input_path);

    let status = Command::new("blueprint-compiler")
        .arg("compile")
        .arg("--output")
        .arg(&dest_path)
        .arg(input_path)
        .status()
        .expect("failed to run blueprint-compiler");

    if !status.success() {
        panic!("blueprint-compiler failed");
    }
}
