use slint::SharedString;
use std::rc::Rc;
use ide_core::COMMANDS;

use crate::ui_layout::{MainWindow, TabItem};

pub fn init_command_list(ui: &MainWindow) {
    let list: Vec<SharedString> = COMMANDS.iter().map(|s| SharedString::from(*s)).collect();
    let model = Rc::new(slint::VecModel::from(list));
    ui.set_command_list(model.into());
}

pub fn update_tabs(ui: &MainWindow, titles: Vec<String>, active_index: usize) {
    let tabs: Vec<TabItem> = titles
        .into_iter()
        .enumerate()
        .map(|(idx, name)| TabItem {
            name: name.into(),
            active: idx == active_index,
        })
        .collect();
    
    let model = Rc::new(slint::VecModel::from(tabs));
    ui.set_tabs(model.into());
    ui.set_active_tab_index(active_index as i32);
}

pub fn clear_tabs(ui: &MainWindow) {
    let model = Rc::new(slint::VecModel::from(Vec::<TabItem>::new()));
    ui.set_tabs(model.into());
    ui.set_active_tab_index(-1);
}
