use std::env;
use std::process::{Command, exit};

fn main() {
    env::set_var("PYO3_PYTHON", python310::Python::default().root().join("python"));
    python310::Python::default().setup();

    let args: Vec<String> = env::args().skip(2).collect();
    let status = Command::new("cargo")
        .args(&args)
        .status().expect("Failed to execute cargo command");

    exit(status.code().unwrap_or(1));
}
