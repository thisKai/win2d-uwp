mod util;

use rayon::prelude::*;
use std::{collections::HashSet, io::prelude::*, path::Path};

fn main() {
    let crate_dir = Path::new(std::env!("CARGO_MANIFEST_DIR"));
    let winmd = crate_dir.join(r#".windows\winmd\Microsoft.Graphics.Canvas.winmd"#);
    let output_crate_dir = crate_dir.parent().unwrap().join("win2d-uwp");

    let output = output_crate_dir.join("src");

    let files = metadata::reader::File::with_default(&[winmd.to_str().unwrap()]).unwrap();
    let reader = &metadata::reader::Reader::new(&files);

    let root = reader
        .tree("Microsoft", &["Microsoft.Graphics.Canvas.UI.Xaml"])
        .expect("`Microsoft` namespace not found");
    let trees = root.flatten();

    trees
        .par_iter()
        .for_each(|tree| gen_tree(reader, &output, tree, false));

    let output = output_crate_dir.join("Cargo.toml");
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"[package]
name = "win2d-uwp"
version = "0.44.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
documentation = "https://microsoft.github.io/windows-docs-rs/"
readme = "../../../docs/readme.md"
rust-version = "1.48"
categories = ["os::windows-apis"]

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []
rustc-args = ["--cfg", "docsrs"]

[dependencies.windows]
version = "0.44"

[features]
default = []
deprecated = []
implement = ["windows/implement"]
"#
        .as_bytes(),
    )
    .unwrap();

    // Skip the root Windows tree while writing features
    for tree in trees.iter().skip(1) {
        let feature = tree.namespace[root.namespace.len() + 1..].replace('.', "_");

        if let Some(pos) = feature.rfind('_') {
            let dependency = &feature[..pos];

            file.write_all(format!("{feature} = [\"{dependency}\"]\n").as_bytes())
                .unwrap();
        } else {
            file.write_all(format!("{feature} = []\n").as_bytes())
                .unwrap();
        }
    }

    file.write_all(b"# windows crate\n").unwrap();

    let mut windows_deps: Vec<_> = get_windows_crate_deps(reader, &trees).into_iter().collect();
    windows_deps.sort();
    for namespace in windows_deps.into_iter() {
        let feature = namespace.replace(".", "_");
        let windows_crate_feature = &feature["Windows_".len()..];

        file.write_all(format!("{feature} = [\"windows/{windows_crate_feature}\"]\n").as_bytes())
            .unwrap();
    }
}

fn gen_tree(
    reader: &metadata::reader::Reader,
    output: &std::path::Path,
    tree: &metadata::reader::Tree,
    rustfmt: bool,
) {
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.cfg = true;
    gen.doc = true;
    let mut tokens = bindgen::namespace(&gen, tree);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    util::format(tree.namespace, &mut tokens, rustfmt);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();
    let mut tokens = bindgen::namespace_impl(&gen, tree);
    util::format(tree.namespace, &mut tokens, rustfmt);
    std::fs::write(path.join("impl.rs"), tokens).unwrap();
}

fn get_windows_crate_deps<'a>(
    reader: &'a metadata::reader::Reader,
    trees: &'a Vec<&'a metadata::reader::Tree>,
) -> HashSet<&'a str> {
    let mut windows_deps = HashSet::new();

    for tree in trees.iter().skip(1) {
        for ty in reader.namespace_types(tree.namespace) {
            let fields = reader.type_def_fields(ty);

            for field in fields {
                let sig_cfg = reader.field_cfg(field);

                for (namespace, _) in sig_cfg.types {
                    if namespace.starts_with("Windows.") {
                        windows_deps.insert(namespace);
                    }
                }
            }

            let methods = reader.type_def_methods(ty);

            for method in methods {
                let sig = reader.method_def_signature(method, &[]);
                let sig_cfg = reader.signature_cfg(&sig);

                for (namespace, _) in sig_cfg.types {
                    if namespace.starts_with("Windows.") {
                        windows_deps.insert(namespace);
                    }
                }
            }
        }
    }

    windows_deps
}
