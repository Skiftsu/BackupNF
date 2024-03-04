use std::{
    cell::RefCell,
    io::{Error, Stdout},
};

use crossterm::event::{self, *};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    *,
};

use super::file_picker;
use crate::config::*;

struct SBackupConfigUI {
    backup_config: SBackupConfig,
    current_element: u16,
    top_element: u16,
}

impl SBackupConfigUI {
    fn new() -> SBackupConfigUI {
        SBackupConfigUI {
            backup_config: SBackupConfig::new(String::new()),
            current_element: 0,
            top_element: 0,
        }
    }

    fn next(&mut self) {
        if self.current_element >= self.backup_config.elements.len() as u16 - 1 {
            return;
        }
        self.current_element += 1;
    }

    fn previous(&mut self) {
        if self.current_element == 0 {
            return;
        }
        self.current_element -= 1;
    }

    fn remove_current(&mut self) {
        let elements_len = self.backup_config.elements.len();

        if elements_len == 0 || self.current_element >= elements_len as u16 {
            return;
        }

        self.backup_config
            .elements
            .remove(self.current_element as usize);

        if !(self.current_element == 0) {
            self.current_element -= 1;
        }
    }

    fn add_new(&mut self, element: SConfigElement) {
        self.backup_config.elements.push(element);
    }

    fn clear_all(&mut self) {
        self.backup_config.elements.clear();
    }
}

pub fn start(
    terminal: &RefCell<Terminal<CrosstermBackend<Stdout>>>,
    mut start_backup: impl FnMut(&SBackupConfig),
) -> Result<(), Error> {
    let backup_config = RefCell::new(SBackupConfigUI::new());

    let mut working = true;

    Ok(while working {
        terminal
            .borrow_mut()
            .draw(|f| ui(f, &backup_config))
            .unwrap();
        let mut callback = |key: KeyCode| {
            if key == KeyCode::Char('q')
                || key == KeyCode::Char('Q')
                || key == KeyCode::Char('й')
                || key == KeyCode::Char('Й')
            {
                working = false;
            }

            if key == KeyCode::Char('b')
                || key == KeyCode::Char('B')
                || key == KeyCode::Char('И')
                || key == KeyCode::Char('и')
            {
                start_backup(&backup_config.borrow().backup_config)
            }

            if key == KeyCode::Down {
                backup_config.borrow_mut().next()
            }

            if key == KeyCode::Up {
                backup_config.borrow_mut().previous()
            }

            if key == KeyCode::Char('r')
                || key == KeyCode::Char('R')
                || key == KeyCode::Char('К')
                || key == KeyCode::Char('к')
            {
                backup_config.borrow_mut().remove_current();
            }

            if key == KeyCode::Char('C')
                || key == KeyCode::Char('c')
                || key == KeyCode::Char('с')
                || key == KeyCode::Char('С')
            {
                backup_config.borrow_mut().clear_all();
            }

            if key == KeyCode::Char('ф')
                || key == KeyCode::Char('Ф')
                || key == KeyCode::Char('A')
                || key == KeyCode::Char('a')
            {
                let callback = |path: String, element_type: EElementType| {
                    backup_config.borrow_mut().add_new(SConfigElement {
                        path: path,
                        content_type: element_type,
                    })
                };
                let _ = file_picker::start(terminal, callback, EElementType::Anything);
            }

            if key == KeyCode::Char('L')
                || key == KeyCode::Char('l')
                || key == KeyCode::Char('д')
                || key == KeyCode::Char('Д')
            {
                let callback = |path: String, _element_type: EElementType| {
                    backup_config.borrow_mut().backup_config.path = path.clone();
                    backup_config.borrow_mut().backup_config.load_config(path);
                };
                let _ = file_picker::start(terminal, callback, EElementType::File);
            }

            if key == KeyCode::Char('ы')
                || key == KeyCode::Char('Ы')
                || key == KeyCode::Char('s')
                || key == KeyCode::Char('S')
            {
                if backup_config.borrow().backup_config.path.is_empty() {
                    let callback = |path: String, _element_type: EElementType| {
                        backup_config.borrow_mut().backup_config.auto_save(path);
                    };
                    let _ = file_picker::start(terminal, callback, EElementType::Folder);
                } else {
                    backup_config
                        .borrow_mut()
                        .backup_config
                        .auto_save(String::new());
                }
            }
        };
        handle_evnets(&mut callback);
    })
}

fn ui(frame: &mut Frame, backup_config: &RefCell<SBackupConfigUI>) {
    // Layouts ==========================
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1), // 0 Header
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());

    let config_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(16), // Text "CurrentConfig"
            Constraint::Fill(1),    // Config file path
            Constraint::Fill(1),    // Spacer
            Constraint::Fill(1),    // Action menu
        ])
        .split(layout[1]);

    let content = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Fill(1)])
        .split(layout[2]);

    // Render ==========================
    // Header
    frame.render_widget(
        Block::new()
            .title("BackupNF")
            .borders(Borders::TOP)
            .border_style(Style::default().green())
            .title_alignment(Alignment::Center),
        layout[0],
    );

    // Content block
    frame.render_widget(
        Block::new()
            .title_bottom("─── CLEAR ALL(C) ─── ADD FILE/FOLDER(A) ─── REMOVE(R) ")
            .borders(Borders::ALL),
        layout[2],
    );

    // Action menu
    frame.render_widget(
        Paragraph::new("START BACKUP(B) QUIT(Q)")
            .gray()
            .alignment(Alignment::Center),
        layout[3],
    );

    config_content_ui(frame, &content[0], backup_config);

    frame.render_widget(Paragraph::new("Current config: ").red(), config_layout[0]);
    frame.render_widget(
        Paragraph::new(backup_config.borrow().backup_config.path.clone()).gray(),
        config_layout[1],
    );

    frame.render_widget(Block::default().borders(Borders::NONE), config_layout[2]);

    frame.render_widget(
        Paragraph::new("LOAD CONFIG(L)  SAVE CONFIG(S)")
            .gray()
            .alignment(Alignment::Right),
        config_layout[3],
    );
}

fn handle_evnets(mut callback: impl FnMut(KeyCode)) {
    if let event::Event::Key(key) = event::read().unwrap() {
        if key.kind == KeyEventKind::Press {
            callback(key.code)
        }
    }
}

fn config_content_ui(frame: &mut Frame, area: &Rect, backup_config: &RefCell<SBackupConfigUI>) {
    let size = area.height;

    let mut array: Vec<Constraint> = Vec::new();
    for _i in 0..size {
        array.push(Constraint::Length(1))
    }
    let layout = Layout::new(Direction::Vertical, array).split(*area);

    for i in 0..size {
        let length = backup_config.borrow().backup_config.elements.len()
            - backup_config.borrow().top_element as usize;
        if i >= length as u16 {
            break;
        }

        let config_element = &backup_config.borrow().backup_config.elements
            [(backup_config.borrow().top_element + i) as usize];

        let is_element_selectd =
            (backup_config.borrow().current_element - backup_config.borrow().top_element) == i;
        content_unit_ui(
            frame,
            &layout[i as usize],
            &config_element,
            is_element_selectd,
        );
    }
}

fn content_unit_ui(frame: &mut Frame, area: &Rect, unit: &SConfigElement, selected: bool) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(6)])
        .split(*area);

    if selected {
        frame.render_widget(Block::new().borders(Borders::NONE).bg(Color::Gray), *area);
    }

    let mut path_widget = Paragraph::new(unit.path.clone()).gray();
    if selected {
        path_widget = path_widget.black();
    }
    frame.render_widget(path_widget, layout[0]);

    let type_text: String;
    match unit.content_type {
        EElementType::File => type_text = "File".to_string(),
        EElementType::Folder => type_text = "Folder".to_string(),
        EElementType::Anything => type_text = "ERROR".to_string(),
    }

    let mut type_text_widget = Paragraph::new(type_text).bold().blue();
    if selected {
        type_text_widget = type_text_widget.bg(Color::Blue).black();
    }
    frame.render_widget(type_text_widget, layout[1]);
}
