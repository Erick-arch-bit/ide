use crate::ui_layout::MainWindow;

pub fn update_editor_text(ui: &MainWindow, text: String) {
    ui.set_editor_text(text.into());
}

pub fn clear_editor(ui: &MainWindow) {
    ui.set_editor_text("".into());
}
