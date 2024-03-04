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

use tui_textarea::TextArea;

use crate::config::*;

use super::file_picker;

pub struct SBackupUI {
    pub folder_name: String,
    pub folder_path: String,
}

impl SBackupUI {
    fn new() -> SBackupUI {
        SBackupUI {
            folder_name: "backup".to_string(),
            folder_path: "".to_string(),
        }
    }
}

pub fn start(
    terminal: &RefCell<Terminal<CrosstermBackend<Stdout>>>,
    config: &SBackupConfig,
    mut start_backup: impl FnMut(&SBackupConfig, &SBackupUI),
) -> Result<(), Error> {
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title_bottom("CANCEL(ESC) SELECT(ENTER)")
            .title_alignment(Alignment::Center)
            .title("Enter backup folder name: "),
    );

    textarea.set_style(Style::default().fg(Color::Yellow));
    textarea.set_placeholder_style(Style::default());
    textarea.set_placeholder_text("backup");

    let mut working = true;
    let mut enter_text = false;

    let mut show_error = false;

    let mut backup = SBackupUI::new();

    Ok(while working {
        terminal.borrow_mut().draw(|f| {
            ui(f, &backup);
            if enter_text {
                let (_end, _save) = enter_folder_name(f, &mut textarea);
                if _end {
                    enter_text = false;
                }

                if _save {
                    backup.folder_name = textarea.lines()[0].clone();
                }
            } else {
                if show_error {
                    if ui_error(
                        f,
                        "Не указано имя папки, папка или в конфиге нету элементов".to_string(),
                        "Close(ESC)".to_string(),
                    ) {
                        show_error = false
                    }
                }
            }
        })?;
        if !enter_text && !show_error {
            let callback = |key: KeyCode| {
                if key == KeyCode::Char('q')
                    || key == KeyCode::Char('Q')
                    || key == KeyCode::Char('й')
                    || key == KeyCode::Char('Й')
                {
                    working = false;
                }
                if key == KeyCode::Char('n')
                    || key == KeyCode::Char('N')
                    || key == KeyCode::Char('т')
                    || key == KeyCode::Char('Т')
                {
                    enter_text = true;
                }

                if key == KeyCode::Char('F')
                    || key == KeyCode::Char('f')
                    || key == KeyCode::Char('а')
                    || key == KeyCode::Char('А')
                {
                    let callback =
                        |path: String, _element_type: EElementType| backup.folder_path = path;
                    file_picker::start(terminal, callback, EElementType::Folder).unwrap();
                }

                if key == KeyCode::Char('Ы')
                    || key == KeyCode::Char('ы')
                    || key == KeyCode::Char('S')
                    || key == KeyCode::Char('s')
                {
                    if backup.folder_name.is_empty()
                        || backup.folder_path.is_empty()
                        || config.elements.is_empty()
                    {
                        show_error = true;
                    } else {
                        start_backup(&config, &backup);
                    }
                }
            };

            handle_evnets(callback);
        }
    })
}

fn ui(frame: &mut Frame, backup: &SBackupUI) {
    // Layouts ==========================
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1), // 0 Header
            Constraint::Length(1), // 1 Folder name
            Constraint::Length(1), // 2 Backup folder
            Constraint::Fill(1),   // 3 Spacer
            Constraint::Length(1), // 4 Action menu
        ],
    )
    .split(frame.size());

    let backup_folder_name_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(32), Constraint::Length(32)])
        .split(layout[1]);

    let backup_folder_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(32), Constraint::Length(32)])
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

    frame.render_widget(
        Paragraph::new("Backup folder name(N): ").white(),
        backup_folder_name_layout[0],
    );

    frame.render_widget(
        Paragraph::new(backup.folder_name.clone()).gray(),
        backup_folder_name_layout[1],
    );

    frame.render_widget(
        Paragraph::new("Backup folder(F): ").white(),
        backup_folder_layout[0],
    );

    frame.render_widget(
        Paragraph::new(backup.folder_path.clone()).gray(),
        backup_folder_layout[1],
    );

    frame.render_widget(Block::default().borders(Borders::NONE), layout[3]);

    frame.render_widget(Paragraph::new("START BACKUP(S)  QUIT(Q)").gray(), layout[4]);
}

fn handle_evnets(mut callback: impl FnMut(KeyCode)) {
    if let event::Event::Key(key) = event::read().unwrap() {
        if key.kind == KeyEventKind::Press {
            callback(key.code)
        }
    }
}

fn enter_folder_name(frame: &mut Frame, text_area: &mut TextArea<'_>) -> (bool, bool) {
    // close: bool, save: bool
    let area = Rect {
        width: 40,
        height: 3,
        x: (frame.size().width / 2) - 20,
        y: (frame.size().height / 2) - 2,
    };

    frame.render_widget(text_area.widget(), area);

    if event::poll(std::time::Duration::from_millis(16)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                return (true, false);
            }
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
                return (true, true);
            }

            text_area.input(key);
        }
    }

    return (false, false);
}

fn ui_error(frame: &mut Frame, text1: String, text2: String) -> bool {
    let width: u16 = 120;
    let height: u16 = 4;
    let area = Rect {
        width: width,
        height: height,
        x: (frame.size().width / 2) - width / 2,
        y: (frame.size().height / 2) - height / 2,
    };

    frame.render_widget(
        Block::new()
            .title(" ERROR! ")
            .borders(Borders::all())
            .border_style(Style::default().red()),
        area,
    );

    let layout = Layout::new(
        Direction::Vertical,
        [Constraint::Length(1), Constraint::Length(1)],
    )
    .margin(1)
    .split(area);

    frame.render_widget(Paragraph::new(text1).gray(), layout[0]);
    frame.render_widget(
        Paragraph::new(text2).white().alignment(Alignment::Right),
        layout[1],
    );

    if let event::Event::Key(key) = event::read().unwrap() {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
            return true;
        }
    }

    return false;
}
