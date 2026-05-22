use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();

    ide_core::init();
    language::init();
    terminal::init();
    extensions::init();

    let state = ide_core::AppState::default();
    ui::run(state)?;

    Ok(())
}