use std::path::PathBuf;

use anyhow::Result;
use editor::Buffer;
use workspace::Project;

#[derive(Clone, Debug)]
pub enum Action {
    OpenFolder,
    SaveFile,
    OpenFile(String),
    CloseTab,
    SwitchTab(usize),
    ToggleCommandPalette,
    ExecuteCommand(String),
}

#[derive(Default)]
pub struct AppState {
    pub project: Option<Project>,
    pub buffers: Vec<Buffer>,
    pub active_index: usize,
    pub command_palette_open: bool,
}

pub const COMMANDS: &[&str] = &[
    "Abrir carpeta...",
    "Guardar archivo",
    "Cerrar archivo",
    "Siguiente tab",
    "Tab anterior",
];

impl AppState {
    pub fn active_buffer(&self) -> Option<&Buffer> {
        self.buffers.get(self.active_index)
    }

    pub fn active_buffer_mut(&mut self) -> Option<&mut Buffer> {
        self.buffers.get_mut(self.active_index)
    }

    pub fn open_folder(&mut self, path: PathBuf) {
        if let Ok(project) = Project::open(&path) {
            self.project = Some(project);
        }
    }

    pub fn open_file_relative(&mut self, rel: &str) -> Result<()> {
        let path = self
            .project
            .as_ref()
            .map(|p| p.root.join(rel))
            .unwrap_or_else(|| PathBuf::from(rel));

        for (i, buf) in self.buffers.iter().enumerate() {
            if buf.path == Some(path.clone()) {
                self.active_index = i;
                return Ok(());
            }
        }

        let buffer = Buffer::open(&path)?;
        self.active_index = self.buffers.len();
        self.buffers.push(buffer);
        Ok(())
    }

    pub fn close_active(&mut self) {
        if self.buffers.is_empty() {
            return;
        }
        self.buffers.remove(self.active_index);
        if !self.buffers.is_empty() {
            self.active_index = self.active_index.min(self.buffers.len() - 1);
        } else {
            self.active_index = 0;
        }
    }

    pub fn switch_to(&mut self, idx: usize) {
        if idx < self.buffers.len() {
            self.active_index = idx;
        }
    }

    pub fn switch_to_next(&mut self) {
        if self.buffers.len() > 1 {
            self.active_index = (self.active_index + 1) % self.buffers.len();
        }
    }

    pub fn switch_to_prev(&mut self) {
        if self.buffers.len() > 1 {
            self.active_index = if self.active_index == 0 {
                self.buffers.len() - 1
            } else {
                self.active_index - 1
            };
        }
    }

    pub fn tab_titles(&self) -> Vec<String> {
        self.buffers
            .iter()
            .map(|b| {
                let name = b.file_name();
                if b.dirty {
                    format!("● {name}")
                } else {
                    name
                }
            })
            .collect()
    }

    pub fn file_list(&self) -> Vec<String> {
        self.project
            .as_ref()
            .map(|p| {
                p.files
                    .iter()
                    .filter_map(|f| f.strip_prefix(&p.root).ok())
                    .map(|rel| rel.to_string_lossy().to_string())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn save_current(&mut self) -> Result<()> {
        if let Some(buf) = self.active_buffer_mut() {
            buf.save()?;
        }
        Ok(())
    }

    pub fn window_title(&self) -> String {
        let file = self
            .active_buffer()
            .map(|b| b.file_name())
            .unwrap_or_else(|| "sin título".to_string());
        let mark = self.active_buffer().map(|b| b.dirty).unwrap_or(false);
        let dirty_mark = if mark { " ●" } else { "" };
        format!("{dirty_mark}{file} - EmberIDE")
    }

    pub fn dispatch(&mut self, action: &Action) -> Result<()> {
        match action {
            Action::SaveFile => self.save_current(),
            Action::OpenFile(path) => self.open_file_relative(path),
            Action::CloseTab => {
                self.close_active();
                Ok(())
            }
            Action::SwitchTab(i) => {
                self.switch_to(*i);
                Ok(())
            }
            Action::ToggleCommandPalette => {
                self.command_palette_open = !self.command_palette_open;
                Ok(())
            }
            Action::ExecuteCommand(_) => Ok(()),
            Action::OpenFolder => Ok(()),
        }
    }
}

pub fn init() {
    println!("core listo");
}
