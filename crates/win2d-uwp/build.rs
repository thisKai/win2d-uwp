use std::{
    fs::{copy, read_dir},
    iter::FromIterator,
    path::PathBuf,
};

#[cfg(target_arch = "x86")]
const ARCH: &str = "x86";
#[cfg(target_arch = "x86_64")]
const ARCH: &str = "x64";
#[cfg(target_arch = "aarch64")]
const ARCH: &str = "arm64";

fn main() {
    let dlls = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".windows", ARCH]);

    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let target_dir = out_dir.ancestors().skip(3).next().unwrap();

    for entry in read_dir(dlls).unwrap().filter_map(Result::ok) {
        let output_path = target_dir.join(entry.file_name());

        copy(entry.path(), output_path).unwrap();
    }
}
