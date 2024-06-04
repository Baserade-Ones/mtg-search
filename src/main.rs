use collection::{get_collections, Collection, User};
use colors::*;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    terminal::{Frame, Terminal},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
};
use std::{collections::HashMap, io};

mod collection;
mod colors;

fn main() -> anyhow::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if matches!(
                key,
                KeyEvent {
                    code: KeyCode::Esc,
                    ..
                } | KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }
            ) {
                break;
            }

            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }
                    _ => {}
                }
            }
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

struct App {
    state: TableState,
    data: HashMap<User, Collection>,
    search: String,
    character_index: usize,
}

impl App {
    fn new() -> Self {
        Self {
            state: TableState::default(),
            data: get_collections().expect("Failed to fetch collections"),
            search: String::new(),
            character_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.search.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.search
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.search.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.search.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.search.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.search = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.search.chars().count())
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(f.size());

    render_table(f, app, rects[0]);

    let input = Paragraph::new(app.search.as_str()).block(Block::bordered().title("Search"));
    f.render_widget(input, rects[1]);
    f.set_cursor(
        // Draw the cursor at the current position in the input field.
        // This position is can be controlled via the left and right arrow key
        rects[1].x + app.character_index as u16 + 1,
        // Move one line down, from the border to the input line
        rects[1].y + 1,
    );
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED);

    let header = [
        "Owner",
        "",
        "Name",
        "Set",
        "Multiverse",
        "Scryfall",
        "Price",
    ]
    .into_iter()
    .map(Cell::from)
    .collect::<Row>()
    .style(header_style)
    .height(1);
    let rows = app
        .data
        .iter()
        .flat_map(|(owner, data)| data.iter().map(move |data| (owner, data)))
        .filter(|(owner, data)| data.matches(app.search.clone()))
        .map(|(owner, data)| data.as_row(*owner).style(Style::new().fg(ROW_FG)).height(1));
    let t = Table::new(
        rows,
        [
            // + 1 is for padding.
            Constraint::Max(7),
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(4),
            Constraint::Max(10),
            Constraint::Max(40),
            Constraint::Max(6),
        ],
    )
    .header(header)
    .highlight_spacing(HighlightSpacing::Always)
    .block(Block::bordered());
    f.render_stateful_widget(t, area, &mut app.state);
}
