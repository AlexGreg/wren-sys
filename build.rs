use std::env;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let src_dir = manifest_path.join("src");
    let wren_dir = src_dir.join("wren/lib");

    let mut make = Command::new("make");

    assert!(make.current_dir(&Path::new("src/wren"))
                .arg("release")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success());

    println!("cargo:rustc-link-search=native={}", wren_dir.display());
    println!("cargo:rustc-link-lib=static=wren");
}
