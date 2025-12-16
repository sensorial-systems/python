use std::path::PathBuf;
use std::env;

fn main() {
    let target_dir = if let Ok(dir) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(dir)
    } else {
        let out_dir = env::var("OUT_DIR").unwrap();
        let target_dir = out_dir.split("target").next().unwrap().to_string() + "target";
        PathBuf::from(target_dir)
    };

    let target_dir = if env::var("PROFILE").unwrap() == "release" {
        target_dir.join("release")
    } else {
        target_dir.join("debug")
    };

    let cargo_manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let files = ["python310.dll", "python3.dll", "vcruntime140.dll", "vcruntime140_1.dll"];
    for file in files{
        let origin = cargo_manifest_dir.join("src").join("embed").join("win64").join(file);
        let destination = target_dir.join(file);
        if !destination.exists() {
            std::fs::copy(origin, destination).unwrap();    
        }
    }
}
