use std::{env, fs, path::PathBuf};

const DLLS: &[&str] = &["SDL3.dll", "SDL3_image.dll"];

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let libs = manifest.join("libs");

    println!("cargo:rustc-link-search=native={}", libs.display());
    println!("cargo:rerun-if-changed=build.rs");

    // OUT_DIR = target/<profile>/build/<pkg>-<hash>/out
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let profile_dir = out.ancestors().nth(3).unwrap();

    for dll in DLLS {
        let src = libs.join(dll);
        let dst = profile_dir.join(dll);

        println!("cargo:rerun-if-changed={}", src.display());

        if let Err(e) = fs::copy(&src, &dst) {
            println!(
                "cargo:warning=failed to copy {} -> {}: {e}",
                src.display(),
                dst.display()
            );
        }
    }
}
