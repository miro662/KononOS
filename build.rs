use std::{env, path::PathBuf};

fn use_platform_specific_linker_script() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let linker_script_name = format!("{}.ld", target_arch);
    let linker_script: PathBuf = [r"linker", &linker_script_name].iter().collect();
    println!("cargo:rustc-link-arg=-T{}", linker_script.to_str().unwrap_or_default())
}

fn main() {
    use_platform_specific_linker_script();
}