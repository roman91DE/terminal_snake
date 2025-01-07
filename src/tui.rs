use crate::core::{Direction as CoreDirection, Game, Point};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::{self};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Terminal,
};

use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Style},
    text::{Span, Spans},
    widgets::Paragraph,
    Frame,
};

fn render_board<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    game: &Game, // Pass the game state to render
) {
    let mut rows = Vec::new();

    for y in 0..game.get_board_y_width() {
        let mut row = String::new();
        for x in 0..game.get_board_x_width() {
            let p = Point::new(x.try_into().unwrap(), y.try_into().unwrap());
            if game.snake.contains_point(p) {
                row.push('O'); // Snake body
            } else if game.fruit == p {
                row.push('F'); // Fruit
            } else {
                row.push(' '); // Empty cell
            }
        }
        rows.push(Span::raw(row));
    }

    let text = rows.into_iter().map(Spans::from).collect::<Vec<_>>();
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Game Area").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}

pub fn run_tui() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Game state
    let mut game = Game::new(20, 30, 3);

    while game.is_running() {
        // Render UI
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());

            // Render the game area
            render_board(f, chunks[0], &game);

            // Render controls
            let controls = Block::default().title("Controls").borders(Borders::ALL);
            f.render_widget(controls, chunks[1]);
        })?;

        // Handle user input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => game.stop(),
                    KeyCode::Up => game.progress(Some(CoreDirection::Up)),
                    KeyCode::Down => game.progress(Some(CoreDirection::Down)),
                    KeyCode::Left => game.progress(Some(CoreDirection::Left)),
                    KeyCode::Right => game.progress(Some(CoreDirection::Right)),
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
