use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseEventKind,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{error::Error, io};

// --- 1. DEFINE OUR APP STATE ---
// This enum tracks exactly what the user is currently interacting with
#[derive(PartialEq)]
enum ActiveField {
    Name,
    Email,
    SubmitButton,
}

// This struct holds all the memory for our application
struct App {
    name_input: String,
    email_input: String,
    users: Vec<String>,  // Stores the submitted data
    active: ActiveField, // Which box is highlighted right now?
}

impl App {
    fn new() -> App {
        App {
            name_input: String::new(),
            email_input: String::new(),
            users: Vec::new(),
            active: ActiveField::Name, // Start with Name focused
        }
    }

    // Function to handle submitting the form
    fn submit(&mut self) {
        if !self.name_input.is_empty() && !self.email_input.is_empty() {
            let record = format!("{} ({})", self.name_input, self.email_input);
            self.users.push(record);
            self.name_input.clear();
            self.email_input.clear();
            self.active = ActiveField::Name; // Reset focus
        }
    }
}

pub fn init() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // EnableMouseCapture is required to read mouse clicks!
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create our application state
    let mut app = App::new();

    // Run the main loop
    let res = run_app(&mut terminal, &mut app);

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app));

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => return Ok(()), // Exit

                    // Navigate fields with Tab
                    KeyCode::Tab => {
                        app.active = match app.active {
                            ActiveField::Name => ActiveField::Email,
                            ActiveField::Email => ActiveField::SubmitButton,
                            ActiveField::SubmitButton => ActiveField::Name,
                        }
                    }

                    // Handle Enter key (Submit form)
                    KeyCode::Enter => {
                        if app.active == ActiveField::SubmitButton {
                            app.submit();
                        }
                    }

                    // Handle Backspace
                    KeyCode::Backspace => match app.active {
                        ActiveField::Name => {
                            app.name_input.pop();
                        }
                        ActiveField::Email => {
                            app.email_input.pop();
                        }
                        _ => {}
                    },

                    // Handle Typing text
                    KeyCode::Char(c) => match app.active {
                        ActiveField::Name => app.name_input.push(c),
                        ActiveField::Email => app.email_input.push(c),
                        ActiveField::SubmitButton => {
                            if c == ' ' {
                                app.submit();
                            } // Spacebar submits if button is focused
                        }
                    },
                    _ => {}
                }
            }
        } else if let Event::Mouse(mouse) = event::read()? {
            // --- HANDLE MOUSE CLICKS ---
            if mouse.kind == MouseEventKind::Down(crossterm::event::MouseButton::Left) {
                // We do a simple check based on the Y coordinate (row) of the click
                // This corresponds to the heights we set in our layout
                if mouse.row >= 0 && mouse.row <= 2 {
                    app.active = ActiveField::Name;
                } else if mouse.row >= 3 && mouse.row <= 5 {
                    app.active = ActiveField::Email;
                } else if mouse.row >= 6 && mouse.row <= 8 {
                    app.active = ActiveField::SubmitButton;
                }
            }
        }
    }
}

// --- 3. DRAW THE UI ---
fn ui(f: &mut Frame, app: &App) {
    // Split the screen into two equal horizontal columns (50% left, 50% right)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.area());

    let left_column = chunks[0];
    let right_column = chunks[1];

    // Split the left column vertically into 3 pieces: Name, Email, and Button
    let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        // 3 rows high for name, 3 for email, 3 for button, and the rest is blank
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(left_column);

    // Helper function to color the active box Green and inactive boxes White
    let get_style = |field: ActiveField, current: &ActiveField| {
        if *current == field {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        }
    };

    // 1. Name Input Box
    let name_widget = Paragraph::new(app.name_input.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Name ")
            .style(get_style(ActiveField::Name, &app.active)),
    );
    f.render_widget(name_widget, form_chunks[0]);

    // 2. Email Input Box
    let email_widget = Paragraph::new(app.email_input.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Email ")
            .style(get_style(ActiveField::Email, &app.active)),
    );
    f.render_widget(email_widget, form_chunks[1]);

    // 3. Submit Button
    let button_text = if app.active == ActiveField::SubmitButton {
        "[ PRESS ENTER TO ADD ]"
    } else {
        "[ ADD ]"
    };
    let button_widget = Paragraph::new(button_text).block(
        Block::default()
            .borders(Borders::ALL)
            .style(get_style(ActiveField::SubmitButton, &app.active)),
    );
    f.render_widget(button_widget, form_chunks[2]);

    // 4. Right Column: The Data List
    let items: Vec<ListItem> = app
        .users
        .iter()
        .map(|u| ListItem::new(Line::from(Span::raw(u))))
        .collect();

    let list_widget = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Recently Added "),
    );
    f.render_widget(list_widget, right_column);
}

