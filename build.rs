extern crate cmake;
use cmake::Config;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = std::fs::canonicalize(&manifest_dir).unwrap();

    let neovim_dir = manifest_dir.to_path_buf();
    let deps_dir = neovim_dir.join(".deps");
    let third_party_dir = neovim_dir.join("third-party");
    let build_dir = neovim_dir.join("build");

    Config::new(&deps_dir)
        .source_dir(&third_party_dir)
        .generator("Ninja")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        deps_dir.join("usr/lib").display()
    );
    println!("cargo:rustc-link-lib=static=luajit-5.1");
    println!("cargo:rustc-link-lib=static=luv");
    println!("cargo:rustc-link-lib=static=msgpackc");
    println!("cargo:rustc-link-lib=static=termkey");
    println!("cargo:rustc-link-lib=static=unibilium");
    println!("cargo:rustc-link-lib=static=utf8proc");
    println!("cargo:rustc-link-lib=static=uv");
    println!("cargo:rustc-link-lib=static=vterm");

    Config::new(&build_dir)
        .source_dir(&neovim_dir)
        .generator("Ninja")
        .target("libnvim")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        build_dir.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=nvim");
}
