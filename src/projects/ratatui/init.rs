use color_eyre::Result;
use color_eyre::owo_colors::colors::Blue;
use crossterm::event;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Paragraph};

pub fn init() {
    println!("This is the concept of ratatui....");

    color_eyre::install();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(render);
        }
    })
}

fn render(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(33); 3]).spacing(1);

    let [top, main] = frame.area().layout(&vertical);
    let [left, middle, right] = main.layout(&horizontal);

    let title = Line::from_iter([
        Span::from("Block Create\n").bold(),
        Span::from(" (Press q or Esc to break) "),
    ]);

    frame.render_widget(title.centered(), top);

    render_left_block(frame, left);
    render_middle_block(frame, middle);

    render_paragraph(frame, middle);
}

fn render_left_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .style(Style::new().on_magenta())
        .title("Left Block");
    frame.render_widget(block, area);
}

fn render_middle_block(frame: &mut Frame, area: Rect) {
    let block = Block::bordered()
        .style(Style::new().blue().on_black().bold().italic())
        .title("Middle Block");
    frame.render_widget(block, area);
}

fn render_paragraph(frame: &mut Frame, area: Rect) {
    let text = "\nI am Nischal Baniya";
    let display_text = Paragraph::new(text)
        .style(Color::White)
        .alignment(Alignment::Center);

    frame.render_widget(display_text, area);
}
