use directories::*;
use include_dir::*;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/embed/win64");

pub struct Python {
    directories: ProjectDirs,
}

impl Default for Python {
    fn default() -> Self {
        let directories = ProjectDirs::from("com", "sensorial-systems", "python310").unwrap();
        Python { directories }
    }
}

impl Python {
    pub fn setup(&self) {
        let root = self.root();
        if !root.exists() {
            std::fs::create_dir_all(&root).unwrap();
            PROJECT_DIR.extract(&root).unwrap();
        }
        std::env::set_var("PYTHONPATH", root.join("app-packages").display().to_string());
        std::env::set_var("PYTHONHOME", root.display().to_string());
    }

    pub fn root(&self) -> std::path::PathBuf {
        self.directories.data_dir().into()
    }

    pub fn python(&self) -> std::process::Command {
        self.setup();
        let program = self.root()
            .join("python");
        std::process::Command::new(program)
    }

    pub fn app_packages(&self) -> std::path::PathBuf {
        self.root()
            .join("app-packages")
    }

    pub fn pip(&self) -> PIP {
        let python = self.python();
        let app_packages = self.app_packages();
        PIP { python, app_packages }
    }
}

pub struct PIP {
    app_packages: std::path::PathBuf,
    python: std::process::Command
}

impl PIP {
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
