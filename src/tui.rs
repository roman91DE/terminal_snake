use crate::core::{Direction as CoreDirection, Game, Point, Config};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::{self};

use tui::{
    backend::CrosstermBackend,
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
                row.push('x'); // Snake body
            } else if game.fruit == p {
                row.push('o'); // Fruit
            } else {
                row.push(' '); // Empty cell
            }
        }
        rows.push(Span::raw(row));
    }

    let text = rows.into_iter().map(Spans::from).collect::<Vec<_>>();
    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Terminal-Snake").borders(Borders::ALL))
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, area);
}

pub fn run_tui(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get terminal dimensions
    let (width, height) = crossterm::terminal::size()?;
    let board_width = (width-2) as usize; // - 1 since core uses 0/0 instead of 1/1 for upper left origin
    let board_height = (height-2) as usize; 

    // Ensure the board is large enough to play
    if board_width < 10 || board_height < 10 {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        return Err("Terminal size is too small to play!".into());
    }

    // Initialize game with dynamic board size
    let mut game = Game::new(board_width, board_height, config.snake_starting_length);
    let mut current_direction = game.get_initial_direction();

    while game.is_running() {
        // Render the game area
        terminal.draw(|f| {
            let area = f.size(); // Use the full terminal area
            render_board(f, area, &game);
        })?;

        let milis: u64 =  config.max_refresh_in_ms.max(config.start_refresh_in_ms - game.get_score() as u64);

        if event::poll(std::time::Duration::from_millis(milis))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => game.stop(),
                    KeyCode::Up if !current_direction.is_opposite(CoreDirection::Up) => {
                        current_direction = CoreDirection::Up;
                    }
                    KeyCode::Down if !current_direction.is_opposite(CoreDirection::Down) => {
                        current_direction = CoreDirection::Down;
                    }
                    KeyCode::Left if !current_direction.is_opposite(CoreDirection::Left) => {
                        current_direction = CoreDirection::Left;
                    }
                    KeyCode::Right if !current_direction.is_opposite(CoreDirection::Right) => {
                        current_direction = CoreDirection::Right;
                    }
                    _ => {}
                }
            }
        }

        // Progress the game in the current direction
        game.progress(Some(current_direction));
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    println!("Snake died :-(");
    println!("Score: {}", game.get_score());
    terminal.show_cursor()?;
    Ok(())
}
