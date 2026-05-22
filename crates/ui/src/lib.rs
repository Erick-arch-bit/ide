use egui::Color32;
use egui::Frame;

pub struct EmberApp {
    state: ide_core::AppState,
}

impl eframe::App for EmberApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        // Menu bar — 28px
        egui::TopBottomPanel::top("menu_bar")
            .min_height(28.0)
            .frame(Frame {
                fill: Color32::from_rgb(0x14, 0x14, 0x14),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    for item in ["File", "Edit", "View", "Terminal", "Help"] {
                        if ui.add(egui::Button::new(item).frame(false)).clicked() {
                            // no-op for Sprint 1
                        }
                    }
                });
            });

        // Icon bar — 44px
        egui::SidePanel::left("icon_bar")
            .default_width(44.0)
            .resizable(false)
            .frame(Frame {
                fill: Color32::from_rgb(0x14, 0x14, 0x14),
                ..Default::default()
            })
            .show(ctx, |_ui| {});

        // File panel — 200px
        egui::SidePanel::left("file_panel")
            .default_width(200.0)
            .resizable(false)
            .frame(Frame {
                fill: Color32::from_rgb(0x1e, 0x1e, 0x1e),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if let Some(project) = &self.state.project {
                    for file in &project.files {
                        ui.label(file.to_string_lossy().to_string());
                    }
                }
            });

        // Editor area — fills remaining space
        egui::CentralPanel::default()
            .frame(Frame {
                fill: Color32::from_rgb(0x1e, 0x1e, 0x1e),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if self.state.active_buffer().is_none() {
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

        // Status bar — 22px
        egui::TopBottomPanel::bottom("status_bar")
            .min_height(22.0)
            .frame(Frame {
                fill: Color32::from_rgb(0x14, 0x14, 0x14),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("EmberIDE")
                        .size(11.0)
                        .color(Color32::from_rgb(0x66, 0x66, 0x66)),
                );
            });
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
        Box::new(|_cc| Ok(Box::new(EmberApp { state }))),
    )
    .map_err(|e| anyhow::anyhow!("{e}"))?;

    Ok(())
}
