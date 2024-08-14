mod tui;
mod app;

use std::io::{Result};


fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    terminal.clear()?;

    let app = app::App::default().run(&mut terminal);

    tui::restore()?;
    app
}
