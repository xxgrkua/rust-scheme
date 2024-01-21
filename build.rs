use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

fn gen_token_categories(out_dir: &Path) -> PathBuf {
    let src_path = out_dir.join("token_cat.rs");

    src_path
}

fn main() {
    let out_dir_str = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);
    gen_token_categories(out_dir);

    println!("cargo:rerun-if-changed=build.rs");
}
