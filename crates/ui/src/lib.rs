#![allow(clippy::collapsible_if)]

use egui::Color32;
use egui::Frame;
use rfd::FileDialog;

pub struct EmberApp {
    state: ide_core::AppState,
    palette_query: String,
    lsp: Option<language::LspClient>,
    file_version: i32,
    completion_open: bool,
    completion_items: Vec<String>,
    completion_idx: usize,
    hover_text: Option<String>,
}

impl EmberApp {
    fn lsp_change_file(&mut self) {
        if let Some(buf) = self.state.active_buffer() {
            if let Some(ref path) = buf.path {
                if let Some(lsp) = self.lsp.as_mut() {
                    lsp.change_file(path, &buf.text, self.file_version);
                }
            }
        }
    }

    fn ensure_lsp(&mut self) {
        if self.lsp.is_some() {
            return;
        }
        if let Some(ref project) = self.state.project {
            if let Ok(uri) = lsp_types::Url::from_directory_path(&project.root) {
                let mut client = language::LspClient::new();
                if client.start(&uri).is_ok() {
                    for buf in &self.state.buffers {
                        if let Some(ref path) = buf.path {
                            client.open_file(path, &buf.text);
                        }
                    }
                    self.lsp = Some(client);
                } else {
                    log::warn!("rust-analyzer not available");
                }
            }
        }
    }

    fn request_completions(&mut self) {
        if let Some(buf) = self.state.active_buffer() {
            let line = 0;
            let character = 0;
            if let Some(ref path) = buf.path {
                if let Some(lsp) = self.lsp.as_mut() {
                    match lsp.request_completion(path, line, character) {
                        Ok(Some(resp)) => {
                            let items = match resp {
                                lsp_types::CompletionResponse::Array(arr) => arr,
                                lsp_types::CompletionResponse::List(list) => list.items,
                            };
                            self.completion_items =
                                items.iter().map(|item| item.label.clone()).collect();
                            self.completion_idx = 0;
                            self.completion_open = true;
                        }
                        Ok(None) => {}
                        Err(e) => log::error!("Completion request failed: {e}"),
                    }
                }
            }
        }
    }
}

impl eframe::App for EmberApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        let prev_dirty = self
            .state
            .active_buffer()
            .map(|b| b.dirty)
            .unwrap_or(false);

        // Keyboard shortcuts
        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::S)
                && let Err(e) = self.state.save_current()
            {
                log::error!("Failed to save: {e}");
            }
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::P) {
                self.state.command_palette_open = !self.state.command_palette_open;
                if self.state.command_palette_open {
                    self.palette_query.clear();
                }
            }
            if i.consume_key(egui::Modifiers::CTRL, egui::Key::W) {
                self.state.close_active();
            }
            if i.consume_key(egui::Modifiers::CTRL | egui::Modifiers::SHIFT, egui::Key::Space) {
                self.completion_open = !self.completion_open;
                if self.completion_open {
                    self.request_completions();
                }
            }
        });

        if self.state.project.is_some() && self.lsp.is_none() {
            self.ensure_lsp();
        }

        if let Some(buf) = self.state.active_buffer() {
            if buf.dirty && buf.dirty != prev_dirty {
                self.lsp_change_file();
            }
        }

        // Menu bar
        egui::TopBottomPanel::top("menu_bar")
            .min_height(28.0)
            .frame(Frame {
                fill: Color32::from_rgb(0x14, 0x14, 0x14),
                ..Default::default()
            })
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New File").clicked() {
                            self.state.buffers.push(editor::Buffer::new_empty());
                            self.state.active_index = self.state.buffers.len() - 1;
                            ui.close_menu();
                        }
                        if ui.button("Open Folder...").clicked() {
                            if let Some(path) = FileDialog::new().pick_folder() {
                                self.state.open_folder(path);
                                self.lsp = None;
                                self.ensure_lsp();
                            }
                            ui.close_menu();
                        }
                        if ui.button("Save").clicked() {
                            if let Err(e) = self.state.save_current() {
                                log::error!("Failed to save: {e}");
                            }
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.button("Completion").clicked() {
                            self.completion_open = !self.completion_open;
                            if self.completion_open {
                                self.request_completions();
                            }
                            ui.close_menu();
                        }
                    });
                    ui.menu_button("View", |_ui| {});
                    ui.menu_button("Terminal", |_ui| {});
                    ui.menu_button("Help", |_ui| {});
                });
            });

        // Tab bar
        egui::TopBottomPanel::top("tab_bar")
            .min_height(28.0)
            .frame(Frame {
                fill: Color32::from_rgb(0x1e, 0x1e, 0x1e),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let titles = self.state.tab_titles();
                let active = self.state.active_index;
                ui.horizontal(|ui| {
                    for (i, title) in titles.iter().enumerate() {
                        let is_active = i == active;
                        if ui.selectable_label(is_active, title).clicked() {
                            self.state.switch_to(i);
                        }
                        if ui
                            .add(
                                egui::Button::new(egui::RichText::new("x").size(11.0))
                                    .frame(false),
                            )
                            .clicked()
                        {
                            let p = self.state.buffers[i].path.clone();
                            self.state.switch_to(i);
                            self.state.close_active();
                            if let Some(ref path) = p {
                                if let Some(lsp) = self.lsp.as_mut() {
                                    lsp.close_file(path);
                                }
                            }
                            break;
                        }
                    }
                });
            });

        // Icon bar
        egui::SidePanel::left("icon_bar")
            .default_width(44.0)
            .resizable(false)
            .frame(Frame {
                fill: Color32::from_rgb(0x14, 0x14, 0x14),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("E")
                            .size(22.0)
                            .color(Color32::from_rgb(0x00, 0x88, 0xff))
                            .strong(),
                    );
                });
            });

        // File panel
        egui::SidePanel::left("file_panel")
            .default_width(200.0)
            .min_width(100.0)
            .max_width(500.0)
            .resizable(true)
            .frame(Frame {
                fill: Color32::from_rgb(0x1e, 0x1e, 0x1e),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if self.state.project.is_some() {
                    let files = self.state.file_list();
                    for rel in &files {
                        if ui.selectable_label(false, rel).clicked()
                            && let Err(e) = self.state.open_file_relative(rel)
                        {
                            log::error!("Failed to open {rel}: {e}");
                        }
                    }
                }
            });

        // Editor area
        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgb(0x1e, 0x1e, 0x1e),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if self.state.active_index < self.state.buffers.len() {
                    let buf = &mut self.state.buffers[self.state.active_index];
                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut buf.text)
                            .font(egui::TextStyle::Monospace)
                            .code_editor(),
                    );
                    if response.changed() {
                        buf.dirty = true;
                    }
                    if let Some(ref path) = buf.path {
                        if let Some(lsp) = self.lsp.as_ref() {
                            let diags = lsp.get_diagnostics(path);
                            if !diags.is_empty() {
                                let first = &diags[0];
                                response.on_hover_text_at_pointer(
                                    egui::RichText::new(&first.message)
                                        .color(Color32::from_rgb(0xff, 0x66, 0x66)),
                                );
                            }
                        }
                    }
                } else {
                    let available = ui.available_height();
                    ui.add_space(available / 2.0 - 8.0);
                    ui.vertical_centered(|ui| {
                        ui.colored_label(
                            Color32::from_rgb(0x55, 0x55, 0x55),
                            "EmberIDE",
                        );
                    });
                }
            });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar")
            .min_height(22.0)
            .frame(Frame {
                fill: Color32::from_rgb(0x14, 0x14, 0x14),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let text = self
                    .state
                    .active_buffer()
                    .map(|b| b.file_name())
                    .unwrap_or_else(|| "EmberIDE".to_string());
                ui.label(
                    egui::RichText::new(&text)
                        .size(11.0)
                        .color(Color32::from_rgb(0x66, 0x66, 0x66)),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(buf) = self.state.active_buffer() {
                        if let Some(ref path) = buf.path {
                            if let Some(lsp) = self.lsp.as_ref() {
                                let diags = lsp.get_diagnostics(path);
                                let n_errors = diags
                                    .iter()
                                    .filter(|d| {
                                        d.severity
                                            == Some(lsp_types::DiagnosticSeverity::ERROR)
                                    })
                                    .count();
                                let n_warnings = diags
                                    .iter()
                                    .filter(|d| {
                                        d.severity
                                            == Some(lsp_types::DiagnosticSeverity::WARNING)
                                    })
                                    .count();
                                if n_errors > 0 || n_warnings > 0 {
                                    let label =
                                        format!("{}, {}", n_errors, n_warnings);
                                    let color = if n_errors > 0 {
                                        Color32::from_rgb(0xff, 0x55, 0x55)
                                    } else {
                                        Color32::from_rgb(0xff, 0xcc, 0x55)
                                    };
                                    let resp = ui.label(
                                        egui::RichText::new(label)
                                            .size(11.0)
                                            .color(color),
                                    );
                                    let msgs: Vec<&str> = diags
                                        .iter()
                                        .take(5)
                                        .map(|d| d.message.as_str())
                                        .collect();
                                    resp.on_hover_text(msgs.join("\n"));
                                }
                            }
                        }
                    }
                });
            });

        // Window title
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(
            self.state.window_title(),
        ));

        // Command palette
        if self.state.command_palette_open {
            egui::Area::new("command_palette".into())
                .anchor(egui::Align2::CENTER_TOP, [0.0, 120.0])
                .default_size([420.0, 320.0])
                .show(ctx, |ui| {
                    let frame = egui::Frame {
                        fill: Color32::from_rgb(0x25, 0x25, 0x25),
                        stroke: egui::Stroke::new(1.0, Color32::from_rgb(0x44, 0x44, 0x44)),
                        ..Default::default()
                    };
                    frame.show(ui, |ui| {
                        ui.set_min_width(420.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut self.palette_query)
                                .hint_text("Type a command...")
                                .desired_width(f32::INFINITY),
                        );
                        ui.separator();
                        let query = self.palette_query.to_lowercase();
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for &cmd in ide_core::COMMANDS {
                                let lower = cmd.to_lowercase();
                                if (query.is_empty() || lower.contains(&query))
                                    && ui.selectable_label(false, cmd).clicked()
                                {
                                    match cmd {
                                        "Abrir carpeta..." => {
                                            if let Some(path) =
                                                FileDialog::new().pick_folder()
                                            {
                                                self.state.open_folder(path);
                                            }
                                        }
                                        "Guardar archivo" => {
                                            if let Err(e) = self.state.save_current() {
                                                log::error!("{e}");
                                            }
                                        }
                                        "Cerrar archivo" => {
                                            self.state.close_active();
                                        }
                                        "Siguiente tab" => {
                                            self.state.switch_to_next();
                                        }
                                        "Tab anterior" => {
                                            self.state.switch_to_prev();
                                        }
                                        _ => {}
                                    }
                                    self.state.command_palette_open = false;
                                }
                            }
                        });
                    });
                });

            if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                self.state.command_palette_open = false;
            }
        }

        // Completion popup
        if self.completion_open {
            let mut close = false;
            egui::Area::new("completion_popup".into())
                .anchor(egui::Align2::LEFT_TOP, [300.0, 300.0])
                .default_size([300.0, 200.0])
                .show(ctx, |ui| {
                    let frame = egui::Frame {
                        fill: Color32::from_rgb(0x2a, 0x2a, 0x2a),
                        stroke: egui::Stroke::new(1.0, Color32::from_rgb(0x55, 0x55, 0x55)),
                        ..Default::default()
                    };
                    frame.show(ui, |ui| {
                        ui.set_min_width(280.0);
                        ui.label(
                            egui::RichText::new("Completions")
                                .size(12.0)
                                .color(Color32::from_rgb(0x88, 0x88, 0x88)),
                        );
                        ui.separator();
                        egui::ScrollArea::vertical()
                            .max_height(180.0)
                            .show(ui, |ui| {
                                for (i, item) in self.completion_items.iter().enumerate() {
                                    let selected = i == self.completion_idx;
                                    if ui.selectable_label(selected, item).clicked() {
                                        if let Some(buf) = self.state.active_buffer_mut() {
                                            buf.text.push_str(item);
                                            buf.dirty = true;
                                        }
                                        close = true;
                                    }
                                }
                            });
                    });
                });

            if close {
                self.completion_open = false;
            }
            if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape)) {
                self.completion_open = false;
            }
            if ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)
            }) {
                self.completion_idx = self.completion_idx.saturating_add(1).min(
                    self.completion_items.len().saturating_sub(1),
                );
            }
            if ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)
            }) {
                self.completion_idx = self.completion_idx.saturating_sub(1);
            }
            if ctx.input_mut(|i| {
                i.consume_key(egui::Modifiers::NONE, egui::Key::Enter)
            }) {
                if self.completion_idx < self.completion_items.len() {
                    if let Some(buf) = self.state.active_buffer_mut() {
                        buf.text.push_str(&self.completion_items[self.completion_idx]);
                        buf.dirty = true;
                    }
                }
                self.completion_open = false;
            }
        }

        if let Some(ref hover) = self.hover_text.clone() {
            egui::Area::new("hover_tooltip".into())
                .anchor(egui::Align2::LEFT_BOTTOM, [300.0, 400.0])
                .show(ctx, |ui| {
                    let frame = egui::Frame {
                        fill: Color32::from_rgb(0x2a, 0x2a, 0x2a),
                        stroke: egui::Stroke::new(1.0, Color32::from_rgb(0x55, 0x55, 0x55)),
                        ..Default::default()
                    };
                    frame.show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(hover)
                                .size(12.0)
                                .color(Color32::WHITE),
                        );
                    });
                });
        }
    }
}

pub fn run(state: ide_core::AppState) -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "EmberIDE",
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(EmberApp {
                state,
                palette_query: String::new(),
                lsp: None,
                file_version: 1,
                completion_open: false,
                completion_items: vec![],
                completion_idx: 0,
                hover_text: None,
            }))
        }),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;

    Ok(())
}
