use std::{env, fs, path::PathBuf};

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let libs = manifest.join("libs");

    println!("cargo:rustc-link-search=native={}", libs.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", libs.join("SDL3.dll").display());

    // OUT_DIR = target/<profile>/build/<pkg>-<hash>/out
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile_dir = out.ancestors().nth(3).unwrap();

    let src = libs.join("SDL3.dll");
    let dst = profile_dir.join("SDL3.dll");
    match fs::copy(&src, &dst) {
        Ok(_) => {}
        Err(e) => println!(
            "cargo:warning=failed to copy {} -> {}: {e}",
            src.display(),
            dst.display()
        ),
    }
}
