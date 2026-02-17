use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .join("../..")
        .canonicalize()
        .expect("workspace root path");
    let dark_core_workdir = workspace_root.join("dark_core");
    let dark_core_executable = dark_core_workdir.join("darkcore");

    println!(
        "cargo:rustc-env=DARKFACTORY_WORKSPACE_ROOT={}",
        workspace_root.display()
    );
    println!(
        "cargo:rustc-env=DARKFACTORY_DARK_CORE_WORKDIR={}",
        dark_core_workdir.display()
    );
    println!(
        "cargo:rustc-env=DARKFACTORY_DARK_CORE_EXECUTABLE={}",
        dark_core_executable.display()
    );
    println!("cargo:rerun-if-changed=build.rs");
}
