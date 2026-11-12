use std::path::PathBuf;
use std::process::Command;

fn libxml2_include() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("TEDS_LIBXML2_INCLUDE") {
        return Some(PathBuf::from(dir));
    }
    if cfg!(target_os = "macos") {
        let out = Command::new("xcrun").arg("--show-sdk-path").output().ok()?;
        let sdk = String::from_utf8(out.stdout).ok()?;
        return Some(PathBuf::from(sdk.trim()).join("usr/include/libxml2"));
    }
    let p = PathBuf::from("/usr/include/libxml2");
    p.exists().then_some(p)
}

fn main() {
    println!("cargo:rerun-if-changed=csrc/html_shim.c");
    println!("cargo:rerun-if-env-changed=TEDS_LIBXML2_INCLUDE");
    println!("cargo:rerun-if-env-changed=TEDS_LIBXML2_LIB");
    let mut build = cc::Build::new();
    build.file("csrc/html_shim.c").warnings(false);
    if let Some(inc) = libxml2_include() {
        build.include(inc);
    }
    build.compile("teds_html_shim");
    if let Ok(dir) = std::env::var("TEDS_LIBXML2_LIB") {
        println!("cargo:rustc-link-search=native={dir}");
    }
    println!("cargo:rustc-link-lib=xml2");
}
