use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Bar, BarChart, Block, BorderType};

use color_eyre::Result;
use crossterm::event;

pub fn init() {
    color_eyre::install();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(render);
        }
    })
}

fn render(frame: &mut Frame) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Length(28), Constraint::Fill((1))]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [left, right] = main.layout(&horizontal);

    let title = Line::from_iter([
        Span::from("Nischal's Barchart\n").bold(),
        Span::from("Press q to Quit"),
    ]);

    frame.render_widget(title.centered(), top);
    render_vertical_barchart(frame, left);
}

fn render_vertical_barchart(frame: &mut Frame, area: Rect) {
    let bars = vec![
        Bar::with_label("Red", 30).red(),
        Bar::with_label("Blue", 20).blue(),
        Bar::with_label("Green", 15).green(),
        Bar::with_label("Yellow", 10).yellow(),
    ];
    let chart = BarChart::vertical(bars).bar_width(6);
    frame.render_widget(chart, area);
}
