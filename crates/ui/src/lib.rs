mod ui_layout;
mod tab_manager;
mod editor_manager;
mod file_manager;
mod event_handlers;

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use ide_core::AppState;
use slint::ComponentHandle;

pub use ui_layout::MainWindow;

pub fn run(state: AppState) -> Result<()> {
    let ui = MainWindow::new()?;
    let state = Rc::new(RefCell::new(state));

    tab_manager::init_command_list(&ui);
    
    event_handlers::setup_open_file(&ui, state.clone());
    event_handlers::setup_save_file(&ui, state.clone());
    event_handlers::setup_editor_changed(&ui, state.clone());
    event_handlers::setup_tab_selected(&ui, state.clone());
    event_handlers::setup_tab_closed(&ui, state.clone());
    event_handlers::setup_toggle_command_palette(&ui, state.clone());
    event_handlers::setup_close_command_palette(&ui, state.clone());
    event_handlers::setup_command_executed(&ui, state.clone());

    ui.run()?;
    Ok(())
}
