use std::{path::PathBuf, process::Command};

use crate::Python;

pub struct PIP {
    app_packages: PathBuf,
    python: Command
}

impl PIP {
    pub fn new(python: &Python) -> Self {
        let python = python.python();
        let app_packages = python.app_packages();
        Self { app_packages, python }
    }

    pub fn install_manifest(&self) {
        let cargo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let cargo_toml = std::path::PathBuf::from(cargo_dir).join("Cargo.toml");
        let manifest = cargo_toml::Manifest::from_path(cargo_toml).unwrap();
        let package = manifest.package.unwrap();
        let pip = package.metadata.as_ref().and_then(|metadata| metadata.get("pip"));
        if let Some(cargo_toml::Value::Table(table)) = pip {
            for (package, _version) in table {
                if !Python::default().pip().install(package, None) {
                    panic!("Failed to install {}", package);
                }        
            }
        }
    }

    pub fn install(&mut self, package: impl AsRef<str>, target_dir: Option<&std::path::Path>) -> bool {
        let target_dir = target_dir.unwrap_or(&self.app_packages);
        self
            .python
            .arg("-m")
            .arg("pip")
            .arg("install")
            .arg(package.as_ref())
            .arg("--target")
            .arg(target_dir)
            .status()
            .unwrap()
            .success()
    }
}
