use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use rand::Rng;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::{
    collections::VecDeque,
    io,
    time::{Duration, Instant},
};

// --- 1. THE DATA STRUCTURES ---

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
struct Position {
    x: u16,
    y: u16,
}

struct GameState {
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    score: u32,
    game_over: bool,
    width: u16,
    height: u16,
}

impl GameState {
    fn new(width: u16, height: u16) -> Self {
        let mut snake = VecDeque::new();
        // Start snake in the middle
        snake.push_back(Position {
            x: width / 2,
            y: height / 2,
        });

        Self {
            snake,
            direction: Direction::Right,
            food: Position { x: 5, y: 5 }, // Initial food spot
            score: 0,
            game_over: false,
            width,
            height,
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            // Keep food within the playable board borders
            let new_food = Position {
                x: rng.gen_range(1..self.width - 1),
                y: rng.gen_range(1..self.height - 1),
            };
            // Don't spawn food inside the snake's body
            if !self.snake.contains(&new_food) {
                self.food = new_food;
                break;
            }
        }
    }

    // --- 2. GAME LOGIC (UPDATE STATE) ---
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Calculate where the head will be next
        let head = self.snake.front().unwrap();
        let mut new_head = *head;

        match self.direction {
            Direction::Up => new_head.y = new_head.y.saturating_sub(1),
            Direction::Down => new_head.y += 1,
            Direction::Left => new_head.x = new_head.x.saturating_sub(1),
            Direction::Right => new_head.x += 1,
        }

        // Hit a wall? (Assuming game box is drawn at borders)
        if new_head.x == 0
            || new_head.x >= self.width - 1
            || new_head.y == 0
            || new_head.y >= self.height - 1
        {
            self.game_over = true;
            return;
        }

        // Hit self?
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        // Move head forward
        self.snake.push_front(new_head);

        // Check food collision
        if new_head == self.food {
            self.score += 10;
            self.spawn_food();
        } else {
            // If no food eaten, remove the tail to simulate moving forward
            self.snake.pop_back();
        }
    }
}

// --- 3. THE MAIN ENGINE & LOOP ---
fn main() -> Result<(), io::Error> {
    // Terminal Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Game configurations
    let board_width = 30;
    let board_height = 20;
    let mut game_state = GameState::new(board_width, board_height);
    game_state.spawn_food(); // Place the first piece of food

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(150); // Game Speed

    loop {
        // --- DRAW SCREEN ---
        terminal.draw(|f| {
            // Layout splitting UI into Canvas and Score area
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .constraints([Constraint::Length(board_height), Constraint::Length(3)])
                .split(f.size());

            let game_area = chunks[0];
            let score_area = chunks[1];

            // Render Board Border
            let board_block = Block::default()
                .title(" Rust Snake ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            f.render_widget(board_block, game_area);

            // Render Score
            let score_text = format!(" Score: {} | Press 'Q' to Quit", game_state.score);
            let score_para = Paragraph::new(score_text)
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(score_para, score_area);

            // Render Snake & Food inside the bounds
            if game_state.game_over {
                let game_over_para = Paragraph::new(" GAME OVER! Press 'Q' to Exit ")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
                f.render_widget(game_over_para, game_area);
            } else {
                // Draw Food
                let food_rect = Rect::new(
                    game_area.x + game_state.food.x,
                    game_area.y + game_state.food.y,
                    1,
                    1,
                );
                f.render_widget(
                    Paragraph::new("●").style(Style::default().fg(Color::Red)),
                    food_rect,
                );

                // Draw Snake
                for pos in &game_state.snake {
                    let snake_rect = Rect::new(game_area.x + pos.x, game_area.y + pos.y, 1, 1);
                    f.render_widget(
                        Paragraph::new("█").style(Style::default().fg(Color::Green)),
                        snake_rect,
                    );
                }
            }
        })?;

        // --- READ INPUT (With Timeout) ---
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Up | KeyCode::Char('w') => {
                            if game_state.direction != Direction::Down {
                                game_state.direction = Direction::Up;
                            }
                        }
                        KeyCode::Down | KeyCode::Char('s') => {
                            if game_state.direction != Direction::Up {
                                game_state.direction = Direction::Down;
                            }
                        }
                        KeyCode::Left | KeyCode::Char('a') => {
                            if game_state.direction != Direction::Right {
                                game_state.direction = Direction::Left;
                            }
                        }
                        KeyCode::Right | KeyCode::Char('d') => {
                            if game_state.direction != Direction::Left {
                                game_state.direction = Direction::Right;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        // --- TICK UPDATE ---
        if last_tick.elapsed() >= tick_rate {
            game_state.update();
            last_tick = Instant::now();
        }
    }

    // Terminal Cleanup (Restores your standard command line state)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
