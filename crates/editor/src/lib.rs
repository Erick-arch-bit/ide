use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

#[derive(Clone, Debug, Default)]
pub struct Buffer {
    pub path: Option<PathBuf>,
    pub text: String,
    pub dirty: bool,
}

impl Buffer {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let text =
            fs::read_to_string(&path).with_context(|| format!("No se pudo leer {:?}", path))?;
        Ok(Self {
            path: Some(path),
            text,
            dirty: false,
        })
    }

    pub fn new_empty() -> Self {
        Self::default()
    }

    pub fn file_name(&self) -> String {
        self.path
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "sin título".to_string())
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.dirty = true;
    }

    pub fn save(&mut self) -> Result<()> {
        let path = self.path.clone().context("Buffer sin ruta")?;
        fs::write(&path, &self.text).with_context(|| format!("No se pudo guardar {:?}", path))?;
        self.dirty = false;
        Ok(())
    }

    pub fn save_as(&mut self, path: PathBuf) -> Result<()> {
        fs::write(&path, &self.text)
            .with_context(|| format!("No se pudo guardar {:?}", path))?;
        self.path = Some(path);
        self.dirty = false;
        Ok(())
    }
}
