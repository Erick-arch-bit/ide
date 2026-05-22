use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use anyhow::{Context, Result};
use lsp_types::notification::{
    DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument, Notification,
};
use lsp_types::request::{GotoDefinition, HoverRequest, Request};
use lsp_types::*;

fn send_message<W: Write>(writer: &mut BufWriter<W>, body: &str) -> Result<()> {
    let header = format!("Content-Length: {}\r\n\r\n", body.len());
    writer.write_all(header.as_bytes())?;
    writer.write_all(body.as_bytes())?;
    writer.flush()?;
    Ok(())
}

fn read_message<R: BufRead>(reader: &mut R) -> Result<Option<String>> {
    let mut header = String::new();
    loop {
        header.clear();
        if reader.read_line(&mut header)? == 0 {
            return Ok(None);
        }
        let trimmed = header.trim();
        if trimmed.is_empty() {
            break;
        }
    }
    let mut content_len = 0usize;
    for line in header.lines() {
        if let Some(len_str) = line.strip_prefix("Content-Length: ") {
            content_len = len_str.trim().parse().unwrap_or(0);
        }
    }
    if content_len == 0 {
        return Ok(None);
    }
    let mut buf = vec![0u8; content_len];
    reader.read_exact(&mut buf)?;
    Ok(Some(String::from_utf8_lossy(&buf).to_string()))
}

fn uri_from_path(path: &std::path::Path) -> Url {
    Url::from_file_path(path).expect("valid path")
}

type SharedDiagnostics = Arc<Mutex<HashMap<Url, Vec<Diagnostic>>>>;

struct PendingRequest {
    #[allow(dead_code)]
    method: String,
    response_tx: mpsc::Sender<serde_json::Value>,
}

pub struct LspClient {
    process: Option<Child>,
    writer: Option<BufWriter<ChildStdin>>,
    next_id: Arc<AtomicU64>,
    diagnostics: SharedDiagnostics,
    reader_handle: Option<thread::JoinHandle<()>>,
    pending: Arc<Mutex<HashMap<u64, PendingRequest>>>,
    initialized: bool,
}

impl Default for LspClient {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for LspClient {
    fn drop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

impl LspClient {
    pub fn new() -> Self {
        Self {
            process: None,
            writer: None,
            next_id: Arc::new(AtomicU64::new(1)),
            diagnostics: Arc::new(Mutex::new(HashMap::new())),
            reader_handle: None,
            pending: Arc::new(Mutex::new(HashMap::new())),
            initialized: false,
        }
    }

    pub fn start(&mut self, root_uri: &Url) -> Result<()> {
        let mut child = Command::new("rust-analyzer")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to spawn rust-analyzer. Is it installed?")?;

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        let writer = BufWriter::new(stdin);
        let reader = BufReader::new(stdout);

        let diagnostics = self.diagnostics.clone();
        let _next_id = self.next_id.clone();
        let pending = self.pending.clone();

        let handle = thread::spawn(move || {
            let mut reader = reader;
            loop {
                let body = match read_message(&mut reader) {
                    Ok(Some(b)) => b,
                    Ok(None) => break,
                    Err(e) => {
                        log::error!("LSP reader error: {e}");
                        break;
                    }
                };
                let val: serde_json::Value = match serde_json::from_str(&body) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if let Some(id) = val.get("id").and_then(|v| v.as_u64()) {
                    if let Ok(mut p) = pending.lock()
                        && let Some(req) = p.remove(&id)
                    {
                        let _ = req.response_tx.send(val);
                    }
                } else if let Some(method) = val.get("method").and_then(|v| v.as_str())
                    && method == "textDocument/publishDiagnostics"
                    && let Some(params) = val.get("params")
                    && let Ok(diag) = serde_json::from_value::<PublishDiagnosticsParams>(params.clone())
                    && let Ok(mut d) = diagnostics.lock()
                {
                    d.insert(diag.uri, diag.diagnostics);
                }
            }
        });

        self.process = Some(child);
        self.writer = Some(writer);
        self.reader_handle = Some(handle);

        let params = serde_json::json!({
            "processId": std::process::id(),
            "rootUri": root_uri,
            "capabilities": {},
            "workspaceFolders": null
        });
        let result = self.request::<lsp_types::request::Initialize>(params)?;
        log::info!("LSP initialized: {:?}", result);
        self.notify::<lsp_types::notification::Initialized>(serde_json::json!({}));
        self.initialized = true;
        Ok(())
    }

    fn send_raw(&mut self, body: &str) -> Result<()> {
        if let Some(w) = self.writer.as_mut() {
            send_message(w, body)
        } else {
            Ok(())
        }
    }

    fn notify<N: Notification>(&mut self, params: serde_json::Value)
    where
        N::Params: serde::Serialize,
    {
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": N::METHOD,
            "params": params,
        });
        if let Err(e) = self.send_raw(&msg.to_string()) {
            log::error!("LSP notify error: {e}");
        }
    }

    fn request<R: Request>(
        &mut self,
        params: serde_json::Value,
    ) -> Result<serde_json::Value>
    where
        R::Params: serde::Serialize,
        R::Result: serde::de::DeserializeOwned,
    {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": R::METHOD,
            "params": params,
        });
        let (tx, rx) = mpsc::channel();
        {
            let mut p = self.pending.lock().unwrap();
            p.insert(
                id,
                PendingRequest {
                    method: R::METHOD.to_string(),
                    response_tx: tx,
                },
            );
        }
        self.send_raw(&msg.to_string())?;
        let response = rx.recv().context("LSP reader died")?;
        if let Some(error) = response.get("error") {
            anyhow::bail!("LSP error: {error}");
        }
        Ok(response.get("result").cloned().unwrap_or(serde_json::Value::Null))
    }

    pub fn open_file(&mut self, path: &std::path::Path, text: &str) {
        if !self.initialized {
            return;
        }
        let uri = uri_from_path(path);
        let params = DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: uri.clone(),
                language_id: "rust".to_string(),
                version: 1,
                text: text.to_string(),
            },
        };
        // We use serde_json::to_value to serialize the params
        let val = serde_json::to_value(&params).unwrap();
        self.notify::<DidOpenTextDocument>(val);
    }

    pub fn change_file(&mut self, path: &std::path::Path, text: &str, version: i32) {
        if !self.initialized {
            return;
        }
        let uri = uri_from_path(path);
        let params = DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri,
                version,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: text.to_string(),
            }],
        };
        let val = serde_json::to_value(&params).unwrap();
        self.notify::<DidChangeTextDocument>(val);
    }

    pub fn close_file(&mut self, path: &std::path::Path) {
        if !self.initialized {
            return;
        }
        let uri = uri_from_path(path);
        let params = DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri },
        };
        let val = serde_json::to_value(&params).unwrap();
        self.notify::<DidCloseTextDocument>(val);
    }

    pub fn get_diagnostics(&self, path: &std::path::Path) -> Vec<Diagnostic> {
        let uri = uri_from_path(path);
        if let Ok(d) = self.diagnostics.lock() {
            d.get(&uri).cloned().unwrap_or_default()
        } else {
            vec![]
        }
    }

    pub fn request_completion(
        &mut self,
        path: &std::path::Path,
        line: u32,
        character: u32,
    ) -> Result<Option<CompletionResponse>> {
        if !self.initialized {
            return Ok(None);
        }
        let uri = uri_from_path(path);
        let params = serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character },
            "context": { "triggerKind": 1 }
        });
        let result = self.request::<lsp_types::request::Completion>(params)?;
        if result.is_null() {
            return Ok(None);
        }
        Ok(serde_json::from_value(result).ok())
    }

    pub fn request_hover(
        &mut self,
        path: &std::path::Path,
        line: u32,
        character: u32,
    ) -> Result<Option<Hover>> {
        if !self.initialized {
            return Ok(None);
        }
        let uri = uri_from_path(path);
        let params = serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character }
        });
        let result = self.request::<HoverRequest>(params)?;
        if result.is_null() {
            return Ok(None);
        }
        Ok(serde_json::from_value(result).ok())
    }

    pub fn request_definition(
        &mut self,
        path: &std::path::Path,
        line: u32,
        character: u32,
    ) -> Result<Option<GotoDefinitionResponse>> {
        if !self.initialized {
            return Ok(None);
        }
        let uri = uri_from_path(path);
        let params = serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character }
        });
        let result = self.request::<GotoDefinition>(params)?;
        if result.is_null() {
            return Ok(None);
        }
        Ok(serde_json::from_value(result).ok())
    }
}
