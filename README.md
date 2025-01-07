# Terminal Snake Game

A classic Snake game written in **Rust**, designed to run directly in your terminal! Test your reflexes and have fun controlling the snake as it navigates the board, collects fruit, and grows longer. Avoid hitting walls or yourself to keep the game going!

## Features

- **Randomized Board and Snake Placement**: Start with a unique layout every time.
- **Adjustable Settings**: Customize the game speed and snake length via a configuration file.
- **Collision Detection**: Lose the game if you hit the wall or the snake bites itself.
- **Real-Time Gameplay**: Play the game directly in your terminal using keyboard controls.

## How to Play

- Use the arrow keys to control the snake's direction:
  - `↑` Up
  - `↓` Down
  - `←` Left
  - `→` Right
- Collect fruit to score points and grow the snake.
- Avoid crashing into the walls or the snake's body.

## Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/roman91DE/terminal_snake.git
   cd terminal-snake
   ```

2. **Build the game**:
   Ensure you have Rust installed. If not, [install Rust](https://www.rust-lang.org/tools/install).
   ```bash
   cargo build --release
   ```

   Alternativley, you can use `make`
   ```bash
   make release
   ```

4. **Run the game**:
   ```bash
   cargo run --release
   ```

## Configuration

Customize the game settings using a `.config/config.toml` file:

```toml
snake_starting_length = 3
start_refresh_in_ms = 100
max_refresh_in_ms = 50
```

- `snake_starting_length`: Initial length of the snake.
- `start_refresh_in_ms`: Initial speed of the game (lower = faster).
- `max_refresh_in_ms`: Maximum speed as the game progresses.

If the file is not present, default settings will be used.

## Code Structure

- **`core.rs`**: Contains the core logic for the game, including the snake, board, and gameplay mechanics.
- **`main.rs`**: Entry point for running the game.
- **`tui.rs`**: Handles I/O from the Terminal


## Dependencies

- [rand](https://crates.io/crates/rand): For generating random positions for the snake and fruit.
- [serde](https://crates.io/crates/serde): For configuration parsing.
- [toml](https://crates.io/crates/toml): For reading and interpreting the configuration file.


## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve the game.

## License

This project is licensed under the [MIT License](LICENSE).

