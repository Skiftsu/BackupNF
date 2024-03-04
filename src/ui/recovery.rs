use std::{
    cell::RefCell,
    io::{self, Error, Stdout},
};

use crossterm::event::{self, *};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    *,
};

use crate::config::*;

use super::file_picker;

#[derive(PartialEq)]
pub enum EFileAction {
    Copied,
    Moved,
}

pub struct SRecoveryPanel {
    pub file_action: EFileAction,
    pub backup_folder: String,
    show_error: bool,
}

impl SRecoveryPanel {
    fn new() -> SRecoveryPanel {
        SRecoveryPanel {
            file_action: EFileAction::Copied,
            backup_folder: "".to_string(),
            show_error: false,
        }
    }
}

pub fn start(
    terminal: &RefCell<Terminal<CrosstermBackend<Stdout>>>,
    mut start_recovery: impl FnMut(&SRecoveryPanel),
) -> Result<(), Error> {
    let mut recovery = SRecoveryPanel::new();
    let mut working = true;

    Ok(while working {
        terminal.borrow_mut().draw(|f| ui(f, &recovery))?;
        let (_close, _result) =
            handle_events(&mut start_recovery, &mut recovery, terminal).unwrap();
        working = !_close;
    })
}

fn ui(frame: &mut Frame, recovery: &SRecoveryPanel) {
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

    let files_will_be_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(32),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .split(layout[1]);

    let backup_folder_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(32), Constraint::Fill(1)])
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
        Paragraph::new("1. Files wiil be: ").white(),
        files_will_be_layout[0],
    );

    let mut copied_btn = Paragraph::new("Copied(C)").gray();
    let mut move_btn = Paragraph::new("Moved(M)").gray();

    match recovery.file_action {
        EFileAction::Copied => copied_btn = copied_btn.green(),
        EFileAction::Moved => move_btn = move_btn.green(),
    };

    frame.render_widget(copied_btn, files_will_be_layout[1]);
    frame.render_widget(move_btn, files_will_be_layout[2]);

    frame.render_widget(
        Paragraph::new("2. Select backup folder(F): ").white(),
        backup_folder_layout[0],
    );

    frame.render_widget(
        Paragraph::new(recovery.backup_folder.clone()).gray(),
        backup_folder_layout[1],
    );

    frame.render_widget(Block::default().borders(Borders::NONE), layout[3]);

    frame.render_widget(
        Paragraph::new("START RECOVERY(S)  QUIT(Q)").gray(),
        layout[4],
    );

    if recovery.show_error {
        ui_error(frame);
    }
}

fn handle_events(
    mut start_recovery: impl FnMut(&SRecoveryPanel),
    recovery: &mut SRecoveryPanel,
    terminal: &RefCell<Terminal<CrosstermBackend<Stdout>>>,
) -> io::Result<(bool, io::Result<()>)> {
    if event::poll(std::time::Duration::from_millis(16))? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')
                || key.code == KeyCode::Char('Q')
                || key.code == KeyCode::Char('й')
                || key.code == KeyCode::Char('Й')
            {
                return Ok((true, Ok(())));
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('S')
                || key.code == KeyCode::Char('s')
                || key.code == KeyCode::Char('ы')
                || key.code == KeyCode::Char('Ы')
            {
                start_recovery(&recovery)
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('c')
                || key.code == KeyCode::Char('C')
                || key.code == KeyCode::Char('С')
                || key.code == KeyCode::Char('с')
            {
                recovery.file_action = EFileAction::Copied
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('Ь')
                || key.code == KeyCode::Char('ь')
                || key.code == KeyCode::Char('m')
                || key.code == KeyCode::Char('M')
            {
                recovery.file_action = EFileAction::Moved
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('А')
                || key.code == KeyCode::Char('а')
                || key.code == KeyCode::Char('F')
                || key.code == KeyCode::Char('f')
            {
                let callback =
                    |path: String, _element_type: EElementType| recovery.backup_folder = path;
                let _ = file_picker::start(terminal, callback, EElementType::Folder);
            }

            if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
                //select_fn(menu.currently_btn.clone());
                return Ok((true, Ok(())));
            }
        }
    }
    Ok((false, Ok(())))
}

fn ui_error(frame: &mut Frame) {
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

    frame.render_widget(
        Paragraph::new("There are matches between your files and the backup files!").gray(),
        layout[0],
    );
    frame.render_widget(
        Paragraph::new("Replace files(R) Skip files(S) Cancel(C)")
            .white()
            .alignment(Alignment::Right),
        layout[1],
    );
}
