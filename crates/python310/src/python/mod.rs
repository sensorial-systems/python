use std::path::{Path, PathBuf};

use directories::*;
use include_dir::*;

use crate::PIP;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/embed/win64");

pub struct Python {
    root: PathBuf,
}

impl Default for Python {
    fn default() -> Self {
        let directories = ProjectDirs::from("systems", "sensorial", "python310").unwrap();
        let root = directories.data_dir().into();
        Python::new(root)
    }
}

impl Python {
    pub fn new(root: PathBuf) -> Self {
        Python { root }
    }

    pub fn setup(&self) {
        let root = self.root();
        if !root.exists() {
            std::fs::create_dir_all(&root).unwrap();
            PROJECT_DIR.extract(&root).unwrap();
        }
        std::env::set_var("PYTHONPATH", root.join("app-packages").display().to_string());
        std::env::set_var("PYTHONHOME", root.display().to_string());
    }

    pub fn root(&self) -> &Path {
        &self.root
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
        PIP::new(self)
    }
}
