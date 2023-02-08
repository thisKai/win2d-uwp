use bytes::Bytes;
use std::{
    fs::{create_dir_all, File},
    io::{self, Cursor},
    path::Path,
};
use zip::ZipArchive;

pub fn download_win2d_dependencies(output_dir: &Path) {
    let output_dir = output_dir.join(".windows");

    download_vrct_forwarders_package(&output_dir);

    download_win2d_uwp_package(&output_dir);
}

fn download_vrct_forwarders_package(output_dir: &Path) {
    let mut package = download_nuget_package("Microsoft.VCRTForwarders.140", "1.0.7");

    for index in 0..package.len() {
        let mut file = package.by_index(index).unwrap();
        if let Some(file_name) = file
            .name()
            .strip_prefix("runtimes/win10-x86/native/release/")
        {
            let mut output_file = dll_output_file("x86", file_name, output_dir).unwrap();

            io::copy(&mut file, &mut output_file).unwrap();
        } else if let Some(file_name) = file
            .name()
            .strip_prefix("runtimes/win10-x64/native/release/")
        {
            let mut output_file = dll_output_file("x64", file_name, output_dir).unwrap();

            io::copy(&mut file, &mut output_file).unwrap();
        } else if let Some(file_name) = file
            .name()
            .strip_prefix("runtimes/win10-arm64/native/release/")
        {
            let mut output_file = dll_output_file("arm64", file_name, output_dir).unwrap();

            io::copy(&mut file, &mut output_file).unwrap();
        }
    }
}

fn download_win2d_uwp_package(output_dir: &Path) {
    let mut package = download_nuget_package("Win2D.uwp", "1.26.0");

    for index in 0..package.len() {
        let mut file = package.by_index(index).unwrap();
        if let Some(file_name) = file.name().strip_prefix("runtimes/win10-x86/native/") {
            let mut output_file = dll_output_file("x86", file_name, output_dir).unwrap();

            io::copy(&mut file, &mut output_file).unwrap();
        } else if let Some(file_name) = file.name().strip_prefix("runtimes/win10-x64/native/") {
            let mut output_file = dll_output_file("x64", file_name, output_dir).unwrap();

            io::copy(&mut file, &mut output_file).unwrap();
        } else if let Some(file_name) = file.name().strip_prefix("runtimes/win10-arm64/native/") {
            let mut output_file = dll_output_file("arm64", file_name, output_dir).unwrap();

            io::copy(&mut file, &mut output_file).unwrap();
        }
    }
}

fn dll_output_file(arch: &str, file_name: &str, output_dir: &Path) -> io::Result<File> {
    let mut output_path = output_dir.to_path_buf();
    output_path.extend([arch, file_name]);
    create_dir_all(&output_path.parent().unwrap())?;
    File::create(output_path)
}

fn download_nuget_package(package: &str, version: &str) -> ZipArchive<Cursor<Bytes>> {
    let id_lower = package.to_lowercase();

    let package_uri = format!(
        "https://api.nuget.org/v3-flatcontainer/{id_lower}/{version}/{id_lower}.{version}.nupkg"
    );
    let res = reqwest::blocking::get(package_uri).unwrap();
    let cursor = Cursor::new(res.bytes().unwrap());
    zip::ZipArchive::new(cursor).unwrap()
}
