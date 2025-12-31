use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn run(cmd: &mut Command, context: &str) {
    let status = cmd
        .status()
        .unwrap_or_else(|err| panic!("failed to run {context}: {err}"));

    if !status.success() {
        panic!("{context} exited with status {status}");
    }
}

fn build_cadical() {
    let mut configure = Command::new("./configure");
    configure
        .current_dir("cadical")
        .args(["--no-contracts", "--no-tracing"]);
    run(&mut configure, "configure cadical");

    let mut build = Command::new("make");
    build.current_dir("cadical").arg("cadical");
    run(&mut build, "build cadical");
}

fn build_uwrmaxsat() {
    let mut build = Command::new("make");
    build
        .current_dir("UWrMaxSat")
        .args(["MAXPRE=", "USESCIP=", "LDFLAG_STATIC=", "r"]);
    run(&mut build, "build UWrMaxSat release");
}

fn emit_link_directives() {
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new("UWrMaxSat/build/release/lib").display()
    );
    println!("cargo:rustc-link-lib=static=uwrmaxsat");
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new("cadical/build").display()
    );
    println!("cargo:rustc-link-lib=static=cadical");
    println!("cargo:rustc-link-lib=dylib=gmp");
    println!("cargo:rustc-link-lib=dylib=z");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=pthread");
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=UWrMaxSat");
    println!("cargo:rerun-if-changed=cadical");

    build_cadical();
    build_uwrmaxsat();
    emit_link_directives();
    generate_bindings();
}
