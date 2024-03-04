use crossterm::event::{self, *};
use ratatui::{prelude::*, widgets::*};
use std::{
    cell::RefCell,
    io::{self, Stdout},
};

#[derive(Clone)]
pub enum CurrentlyBtn {
    Backup,
    Restore,
}

pub struct Menu {
    pub currently_btn: CurrentlyBtn,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            currently_btn: CurrentlyBtn::Backup,
        }
    }

    fn next_btn(&mut self) {
        match self.currently_btn {
            CurrentlyBtn::Backup => self.currently_btn = CurrentlyBtn::Restore,
            CurrentlyBtn::Restore => self.currently_btn = CurrentlyBtn::Backup,
        };
    }
}

pub fn start(
    terminal: &RefCell<Terminal<CrosstermBackend<Stdout>>>,
    mut select_fn: impl FnMut(CurrentlyBtn),
) -> io::Result<()> {
    let mut working = true;
    let mut menu = Menu::new();

    Ok(while working {
        terminal.borrow_mut().draw(|f| ui(f, &menu))?;
        let (_close, _result) = handle_evnets(&mut menu, &mut select_fn).unwrap();
        working = !_close;
    })
}

fn ui(frame: &mut Frame, menu: &Menu) {
    // Btns ==========================
    let mut backup_btn = Block::default()
        .title("BACKUP")
        .borders(Borders::NONE)
        .style(Style::default())
        .title_alignment(Alignment::Center);
    let mut restore_btn = Block::default()
        .title("RESTORE")
        .borders(Borders::NONE)
        .style(Style::default())
        .title_alignment(Alignment::Center);

    let active_style = Style::default().black().bg(Color::Gray).bold();

    match menu.currently_btn {
        CurrentlyBtn::Backup => backup_btn = backup_btn.style(active_style),
        CurrentlyBtn::Restore => restore_btn = restore_btn.style(active_style),
    };

    // Layouts ==========================
    let layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1), // 0 Header
            Constraint::Fill(1),   // 1 Spacer
            Constraint::Length(1), // 2 Btn Backup
            Constraint::Length(1), // 3 Btn Restore
            Constraint::Fill(1),   // 4 Spacer
            Constraint::Length(1), // 5 Action menu
        ],
    )
    .split(frame.size());

    let backup_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(30),
            Constraint::Fill(1),
        ])
        .split(layout[2]);
    let restore_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(30),
            Constraint::Fill(1),
        ])
        .split(layout[3]);

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

    //Spacer
    frame.render_widget(Block::default().borders(Borders::NONE), layout[1]);

    // Btn Backup
    frame.render_widget(Block::default().borders(Borders::NONE), restore_layout[0]);
    frame.render_widget(backup_btn, backup_layout[1]);
    frame.render_widget(Block::default().borders(Borders::NONE), backup_layout[2]);

    // Btn Restore
    frame.render_widget(Block::default().borders(Borders::NONE), restore_layout[0]);
    frame.render_widget(restore_btn, restore_layout[1]);
    frame.render_widget(Block::default().borders(Borders::NONE), restore_layout[2]);

    //Spacer
    frame.render_widget(Block::default().borders(Borders::NONE), layout[4]);

    // Action menu
    frame.render_widget(
        Paragraph::new("SELECT(ENTER) QUIT(Q)")
            .gray()
            .alignment(Alignment::Center),
        layout[5],
    );
}

fn handle_evnets(
    menu: &mut Menu,
    mut select_fn: impl FnMut(CurrentlyBtn),
) -> io::Result<(bool, io::Result<()>)> {
    if let event::Event::Key(key) = event::read()? {
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q')
            || key.code == KeyCode::Char('Q')
            || key.code == KeyCode::Char('й')
            || key.code == KeyCode::Char('Й')
        {
            return Ok((true, Ok(())));
        }

        if key.kind == KeyEventKind::Press && key.code == KeyCode::Up || key.code == KeyCode::Down {
            menu.next_btn();
        }

        if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
            select_fn(menu.currently_btn.clone());
        }
    }

    Ok((false, Ok(())))
}
