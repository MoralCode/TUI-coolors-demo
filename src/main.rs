use tui::backend::CrosstermBackend;
use tui::style::{Color, Style};
use tui::symbols::Marker;
use tui::widgets::{Axis, Block, Chart, Dataset};
use tui::Terminal;

use crossterm::event::{self, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Data points
        // Example data points
    let red_points = vec![(0.0, 1.0), (1.0, 2.0)];
    let blue_points = vec![(2.0, 1.0), (3.0, 2.0)];
    let green_points = vec![(4.0, 1.0), (5.0, 2.0)];

    // Create datasets with different colors
    let datasets = vec![
        Dataset::default()
            .name("Red Points")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Red))
            .data(&red_points),
        Dataset::default()
            .name("Blue Points")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Blue))
            .data(&blue_points),
        Dataset::default()
            .name("Green Points")
            .marker(Marker::Braille)
            .style(Style::default().fg(Color::Green))
            .data(&green_points),
    ];

    // Main rendering loop
    loop {
        terminal.draw(|f| {
            let size = f.size();

            let chart = Chart::new(datasets.clone())
                .block(Block::default().title("Multi-Colored Points"))
                .x_axis(Axis::default().title("X Axis").bounds([0.0, 6.0]))
                .y_axis(Axis::default().title("Y Axis").bounds([0.0, 6.0]));

            f.render_widget(chart, size);
        })?;

        // Exit on 'q'
        if let event::Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
