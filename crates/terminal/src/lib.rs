use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use vt100::Parser;

pub const DEFAULT_ROWS: u16 = 24;
pub const DEFAULT_COLS: u16 = 80;

pub struct TerminalSession {
    parser: Arc<Mutex<Parser>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    rows: u16,
    cols: u16,
}

impl TerminalSession {
    pub fn new(shell: Option<&str>) -> Result<Self> {
        let rows = DEFAULT_ROWS;
        let cols = DEFAULT_COLS;

        let pty_system = NativePtySystem::default();
        let size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };
        let pair = pty_system
            .openpty(size)
            .context("Failed to open PTY")?;

        let cmd = shell.unwrap_or("bash");
        let cmd = CommandBuilder::new(cmd);
        let mut child = pair
            .slave
            .spawn_command(cmd)
            .context("Failed to spawn shell")?;

        let writer = pair.master.take_writer().context("No writer")?;

        let parser = Parser::new(rows, cols, 0);
        let parser = Arc::new(Mutex::new(parser));
        let parser_clone = parser.clone();

        // PTY reader thread
        std::thread::spawn(move || {
            let mut reader = pair
                .master
                .try_clone_reader()
                .expect("Failed to clone reader");
            let mut buf = vec![0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Ok(mut p) = parser_clone.lock() {
                            p.process(&buf[..n]);
                        }
                    }
                    Err(e) => {
                        log::error!("PTY read error: {e}");
                        break;
                    }
                }
            }
            // Wait for child to avoid zombie
            let _ = child.wait();
        });

        Ok(Self {
            parser,
            writer: Arc::new(Mutex::new(writer)),
            rows,
            cols,
        })
    }

    pub fn write(&self, data: &[u8]) -> Result<()> {
        let mut w = self
            .writer
            .lock()
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        w.write_all(data)?;
        w.flush()?;
        Ok(())
    }

    pub fn write_str(&self, s: &str) -> Result<()> {
        self.write(s.as_bytes())
    }

    pub fn resize(&self, rows: u16, cols: u16) {
        if rows == self.rows && cols == self.cols {
            return;
        }
        if let Ok(mut p) = self.parser.lock() {
            // Recreate parser with new size (no set_size method in vt100 0.16)
            let new_parser = vt100::Parser::new(rows, cols, 0);
            *p = new_parser;
        }
    }

    pub fn contents(&self) -> String {
        if let Ok(p) = self.parser.lock() {
            p.screen().contents()
        } else {
            String::new()
        }
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn cols(&self) -> u16 {
        self.cols
    }
}

pub fn init() {
    log::info!("terminal crate initialized");
}
