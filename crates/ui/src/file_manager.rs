use slint::SharedString;
use std::rc::Rc;

use crate::ui_layout::MainWindow;

pub fn update_file_list(ui: &MainWindow, files: Vec<String>) {
    let file_strings: Vec<SharedString> = files
        .into_iter()
        .map(SharedString::from)
        .collect();
    
    let model = Rc::new(slint::VecModel::from(file_strings));
    ui.set_files(model.into());
}

pub fn clear_file_list(ui: &MainWindow) {
    let model = Rc::new(slint::VecModel::from(Vec::<SharedString>::new()));
    ui.set_files(model.into());
}
