use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn emit_rerun_if_changed(path: &PathBuf) {
    if path.exists() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}

fn emit_rerun_if_changed_recursive(path: &PathBuf) {
    if !path.exists() {
        return;
    }

    emit_rerun_if_changed(path);

    if path.is_dir() {
        let Ok(entries) = fs::read_dir(path) else {
            return;
        };

        for entry in entries.flatten() {
            emit_rerun_if_changed_recursive(&entry.path());
        }
    }
}

fn run_dark_core_build(dark_core_workdir: &PathBuf) {
    let output = Command::new("bun")
        .args(["run", "build:exec"])
        .current_dir(dark_core_workdir)
        .output()
        .unwrap_or_else(|error| {
            panic!(
                "dark_rust build.rs failed to run dark_core build (workdir={}, error={error})",
                dark_core_workdir.display()
            )
        });

    if output.status.success() {
        return;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    panic!(
        "dark_rust build.rs failed to build dark_core executable (workdir={}, stdout={}, stderr={})",
        dark_core_workdir.display(),
        stdout.trim(),
        stderr.trim()
    );
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .join("../..")
        .canonicalize()
        .expect("workspace root path");
    let dark_core_workdir = workspace_root.join("dark_core");
    let dark_core_executable = dark_core_workdir.join("darkcore");

    run_dark_core_build(&dark_core_workdir);

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
    emit_rerun_if_changed(&dark_core_workdir.join("package.json"));
    emit_rerun_if_changed(&workspace_root.join("bun.lock"));
    emit_rerun_if_changed_recursive(&dark_core_workdir.join("src"));
}
