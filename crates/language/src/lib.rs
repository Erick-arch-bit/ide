#![allow(clippy::collapsible_if)]

pub mod lsp;

pub use lsp::LspClient;

pub fn init() {
    log::info!("language crate initialized");
}
