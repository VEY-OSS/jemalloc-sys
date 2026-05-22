use std::env;
use std::fmt::Debug;
use std::path::PathBuf;

use bindgen::callbacks::{ItemInfo, ItemKind, ParseCallbacks};
use bindgen::{MacroTypeVariation, RustEdition};

#[derive(Debug)]
struct ParseCallback;

impl ParseCallbacks for ParseCallback {
    fn generated_name_override(&self, item_info: ItemInfo<'_>) -> Option<String> {
        if !matches!(item_info.kind, ItemKind::Function) {
            return None;
        }

        item_info
            .name
            .strip_prefix("je_")
            .map(|name| name.to_owned())
    }
}

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let bindgen_file = out_dir.join("bindgen.rs");
    let include_dirs = probe_installed().unwrap_or_default();

    let mut builder = bindgen::Builder::default();
    if let Some(dir) = include_dirs.first() {
        println!("cargo:rerun-if-changed={}", dir.display());
        builder = builder.clang_arg(format!("-I{}", dir.display()));
    }

    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = builder
        .header("wrapper.h")
        .rust_edition(RustEdition::Edition2024)
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .allowlist_file(".*[[:punct:]]jemalloc[[:punct:]]jemalloc\\.h")
        .parse_callbacks(Box::new(ParseCallback))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(&bindgen_file)
        .expect("Couldn't write bindings!");
}

fn probe_installed() -> Option<Vec<PathBuf>> {
    if let Ok(lib) = pkg_config::Config::new()
        .print_system_libs(false)
        .probe("jemalloc")
    {
        return Some(lib.include_paths);
    }

    if let Ok(lib) = vcpkg::Config::new()
        .emit_includes(true)
        .find_package("jemalloc")
    {
        return Some(lib.include_paths);
    }

    println!("cargo:warning=Could not find Jemalloc using pkg-config or vcpkg");
    None
}
