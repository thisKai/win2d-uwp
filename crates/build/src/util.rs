use std::io::*;

/// Formats the token string
pub fn format(namespace: &str, tokens: &mut String, use_rustfmt: bool) {
    if use_rustfmt {
        rustfmt(namespace, tokens);
    } else {
        let file = syn::parse_file(tokens).unwrap();
        *tokens = prettyplease::unparse(&file);
    }
}

/// Formats the token string with `rustfmt`
pub fn rustfmt(name: &str, tokens: &mut String) {
    let mut child = std::process::Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        *tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!(
            "rustfmt failed for `{name}` with status {}\nError:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
