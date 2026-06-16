use std::io;

use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use ratatui::{Terminal, prelude::CrosstermBackend};

#[derive(Debug, Clone)]
struct UserDetails<'a> {
    name_input: &'a str,
    email_input: &'a str,
    contact: &'a str,
}

#[derive(Debug)]
struct LinearQueue<'a> {
    data: Vec<Option<UserDetails<'a>>>,
    count: usize,
    front: usize,
    rear: usize,
    capacity: usize,
}

impl<'a> LinearQueue<'a> {
    fn new(user_inp_size: u8) -> Self {
        let alloc_size = user_inp_size as usize;
        LinearQueue {
            data: vec![None; alloc_size],
            front: 0,
            rear: 0,
            count: 0,
            capacity: alloc_size,
        }
    }

    fn is_full(&self) -> bool {
        if self.capacity == self.rear {
            return true;
        } else {
            return false;
        }
    }

    fn is_empty(&self) -> bool {
        if self.count == 0 {
            return true;
        } else {
            return false;
        }
    }

    fn add_user_in_queue(&mut self, data: UserDetails<'a>) {
        if self.is_full() {
            println!("Queue is full............");
            return;
        }

        self.data[self.rear] = Some(data);
        self.rear += 1;
        self.count += 1;
    }

    fn remove_user_from_queue(&mut self) {
        if self.is_empty() {
            println!("Queue is empty..........");
            return;
        }

        self.data[self.front] = None;
        self.front += 1;
    }
}

pub fn main_todo() -> Result<()> {
    // NOTE: This is for seting up of terminal
    enable_raw_mode();

    let mut stdot = io::stdout();

    // NOTE: to capture mouse clicke event

    execute!(stdot, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdot);

    let mut terminal = Terminal::new(backend)?;
}

