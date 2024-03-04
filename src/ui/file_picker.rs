use crossterm::event::{read, Event, KeyCode};
use ratatui::prelude::*;
use ratatui_explorer::{FileExplorer, Theme};
use std::{
    cell::RefCell,
    io::{Error, Stdout},
};

use crate::config::EElementType;

pub fn start(
    terminal: &RefCell<Terminal<CrosstermBackend<Stdout>>>,
    mut fn_select: impl FnMut(String, EElementType),
    requested_type: EElementType,
) -> Result<(), Error> {
    let theme = Theme::default().add_default_title();
    let mut file_explorer = FileExplorer::with_theme(theme)?;

    let mut working = true;

    Ok(while working {
        terminal.borrow_mut().draw(|f| {
            f.render_widget(&file_explorer.widget(), f.size());
        })?;

        let event = read()?;
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                break;
            }
            if key.code == KeyCode::Enter {
                let current_path = file_explorer.current().path();

                let mut element_type = EElementType::Anything;
                if current_path.is_file() {
                    element_type = EElementType::File
                } else if current_path.is_dir() {
                    element_type = EElementType::Folder
                }

                if requested_type != EElementType::Anything && requested_type != element_type {
                    continue;
                }

                fn_select(current_path.to_string_lossy().to_string(), element_type);
                working = false;
            }
        }

        file_explorer.handle(&event)?;
    })
}
