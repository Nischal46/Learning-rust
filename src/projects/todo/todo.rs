use std::{error::Error, io};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::{
    Terminal,
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

#[derive(Debug)]
struct App {
    show_popup: bool,
    input: String,
}

impl App {
    fn new() -> Self {
        Self {
            show_popup: true,
            input: String::new(),
        }
    }
}

pub fn main_todo() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => return Ok(()),

                    KeyCode::Char(c) => {
                        if app.show_popup {
                            app.input.push(c);
                        }
                    }

                    KeyCode::Backspace => {
                        if app.show_popup {
                            app.input.pop();
                        }
                    }

                    KeyCode::Enter => {
                        if app.show_popup {
                            app.show_popup = false;
                        }
                    }

                    KeyCode::Char('p') => {
                        app.show_popup = true;
                    }

                    _ => {}
                }
            }
        }
    }
}

fn ui(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let main = Paragraph::new(format!(
        "Saved Text: {}\n\nPress P to open popup\nPress ESC to quit",
        app.input
    ))
    .block(Block::default().title("Main Screen").borders(Borders::ALL));

    frame.render_widget(main, size);

    if app.show_popup {
        let popup_area = centered_rect(60, 20, size);

        frame.render_widget(Clear, popup_area);

        let popup = Paragraph::new(app.input.as_str()).block(
            Block::default()
                .title("Type Something")
                .borders(Borders::ALL),
        );

        frame.render_widget(popup, popup_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}
