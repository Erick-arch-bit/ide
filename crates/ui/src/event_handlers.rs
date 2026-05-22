use std::cell::RefCell;
use std::rc::Rc;

use ide_core::AppState;
use slint::ComponentHandle;
use crate::ui_layout::MainWindow;
use crate::tab_manager;
use crate::editor_manager;
use crate::file_manager;

pub fn setup_open_file(ui: &MainWindow, state: Rc<RefCell<AppState>>) {
    let ui_weak = ui.as_weak();
    ui.on_open_file(move |path| {
        let ui = ui_weak.unwrap();
        let mut s = state.borrow_mut();
        if path.is_empty() {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                s.open_folder(folder);
                sync_state(&ui, &s);
            }
        } else if let Err(e) = s.open_file_relative(path.as_str()) {
            eprintln!("mi-ide: error al abrir archivo: {e}");
        } else {
            sync_state(&ui, &s);
        }
    });
}

pub fn setup_save_file(ui: &MainWindow, state: Rc<RefCell<AppState>>) {
    let ui_weak = ui.as_weak();
    ui.on_save_file(move || {
        let ui = ui_weak.unwrap();
        let mut s = state.borrow_mut();
        if let Err(e) = s.save_current() {
            eprintln!("mi-ide: error al guardar: {e}");
        }
        sync_state(&ui, &s);
    });
}

pub fn setup_editor_changed(ui: &MainWindow, state: Rc<RefCell<AppState>>) {
    let ui_weak = ui.as_weak();
    ui.on_editor_changed(move |text| {
        let ui = ui_weak.unwrap();
        let mut s = state.borrow_mut();
        if let Some(buf) = s.active_buffer_mut() {
            if buf.text != text.as_str() {
                buf.text = text.to_string();
                buf.dirty = true;
            }
        }
        sync_state(&ui, &s);
    });
}

pub fn setup_tab_selected(ui: &MainWindow, state: Rc<RefCell<AppState>>) {
    let ui_weak = ui.as_weak();
    ui.on_tab_selected(move |idx| {
        let ui = ui_weak.unwrap();
        let mut s = state.borrow_mut();
        s.switch_to(idx as usize);
        sync_state(&ui, &s);
    });
}

pub fn setup_tab_closed(ui: &MainWindow, state: Rc<RefCell<AppState>>) {
    let ui_weak = ui.as_weak();
    ui.on_tab_closed(move |idx| {
        let ui = ui_weak.unwrap();
        let mut s = state.borrow_mut();
        if (idx as usize) < s.buffers.len() {
            s.close_active();
        }
        sync_state(&ui, &s);
    });
}

pub fn setup_toggle_command_palette(ui: &MainWindow, state: Rc<RefCell<AppState>>) {
    let ui_weak = ui.as_weak();
    ui.on_toggle_command_palette(move || {
        let ui = ui_weak.unwrap();
        let mut s = state.borrow_mut();
        s.command_palette_open = !s.command_palette_open;
        ui.set_command_palette_open(s.command_palette_open);
    });
}

pub fn setup_close_command_palette(ui: &MainWindow, state: Rc<RefCell<AppState>>) {
    let ui_weak = ui.as_weak();
    ui.on_close_command_palette(move || {
        let ui = ui_weak.unwrap();
        let mut s = state.borrow_mut();
        s.command_palette_open = false;
        ui.set_command_palette_open(false);
    });
}

pub fn setup_command_executed(ui: &MainWindow, state: Rc<RefCell<AppState>>) {
    let ui_weak = ui.as_weak();
    ui.on_command_executed(move |cmd| {
        let ui = ui_weak.unwrap();
        let mut s = state.borrow_mut();
        s.command_palette_open = false;
        ui.set_command_palette_open(false);

        match cmd.as_str() {
            "Abrir carpeta..." => {
                drop(s);
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    let mut s = state.borrow_mut();
                    s.open_folder(folder);
                    sync_state(&ui, &s);
                }
            }
            "Guardar archivo" => {
                if let Err(e) = s.save_current() {
                    eprintln!("mi-ide: error al guardar: {e}");
                }
                sync_state(&ui, &s);
            }
            "Cerrar archivo" => {
                s.close_active();
                sync_state(&ui, &s);
            }
            "Siguiente tab" => {
                s.switch_to_next();
                sync_state(&ui, &s);
            }
            "Tab anterior" => {
                s.switch_to_prev();
                sync_state(&ui, &s);
            }
            _ => {}
        }
    });
}

fn sync_state(ui: &MainWindow, state: &AppState) {
    // Update tabs
    let titles = state.tab_titles();
    if titles.is_empty() {
        tab_manager::clear_tabs(ui);
    } else {
        tab_manager::update_tabs(ui, titles, state.active_index);
    }

    // Update file list
    let files = state.file_list();
    if files.is_empty() {
        file_manager::clear_file_list(ui);
    } else {
        file_manager::update_file_list(ui, files);
    }

    // Update editor text
    if let Some(buf) = state.active_buffer() {
        editor_manager::update_editor_text(ui, buf.text.clone());
    } else {
        editor_manager::clear_editor(ui);
    }

    // Update window title
    ui.set_window_title(state.window_title().into());
}
