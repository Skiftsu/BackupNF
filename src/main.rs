mod backup_service;
mod config;
mod tui;
mod ui;

use std::{
    cell::RefCell,
    io::{self, Stdout},
};

use config::SBackupConfig;
use ratatui::{backend::CrosstermBackend, terminal::Terminal};
use ui::{backup::SBackupUI, menu::CurrentlyBtn, recovery::SRecoveryPanel};

struct App {
    terminal: RefCell<Terminal<CrosstermBackend<Stdout>>>,
}

impl App {
    fn select_panel(&self, btn: CurrentlyBtn) {
        let _ = match btn {
            CurrentlyBtn::Backup => {
                let callback = |config: &SBackupConfig| self.backup_panel(config);
                ui::backup_config::start(&self.terminal, callback)
            }
            CurrentlyBtn::Restore => {
                let callback = |config: &SRecoveryPanel| backup_service::recovery(config);
                ui::recovery::start(&self.terminal, callback)
            }
        };
    }

    fn backup_panel(&self, config: &SBackupConfig) {
        let callback =
            |config: &SBackupConfig, details: &SBackupUI| backup_service::backup(config, details);
        ui::backup::start(&self.terminal, config, callback).unwrap();
    }

    fn new() -> App {
        let app = App {
            terminal: RefCell::new(tui::init().unwrap()),
        };

        let callback = |btn: CurrentlyBtn| app.select_panel(btn);
        ui::menu::start(&app.terminal, callback).unwrap();

        app
    }
}

impl Drop for App {
    fn drop(&mut self) {
        tui::restore().unwrap();
    }
}

fn main() -> io::Result<()> {
    let app = App::new();

    Ok(())
}
