use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

#[derive(Clone, Debug)]
pub struct Project {
    pub root: PathBuf,
    pub files: Vec<PathBuf>,
}

impl Project {
    pub fn open(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        let mut files = Vec::new();
        collect_files(&root, &root, &mut files)
            .with_context(|| format!("No se pudo leer {:?}", root))?;
        files.sort_by(|a, b| {
            let a_is_dir = a.is_dir();
            let b_is_dir = b.is_dir();
            if a_is_dir != b_is_dir {
                b_is_dir.cmp(&a_is_dir)
            } else {
                a.file_name().cmp(&b.file_name())
            }
        });
        Ok(Self { root, files })
    }
}

fn collect_files(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if file_name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            if path == root.join(".git") {
                continue;
            }
            collect_files(root, &path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}
