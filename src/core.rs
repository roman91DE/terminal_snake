use rand::Rng;
use std::collections::VecDeque;
use std::io::{self, Write}; // for raw printing only

#[derive(Debug, Clone, Copy)]
struct Board {
    x_width: usize,
    y_width: usize,
}

impl Board {
    fn get_x_width(&self) -> usize {
        self.x_width
    }
    fn get_y_width(&self) -> usize {
        self.y_width
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn move_right(p: Point) -> Point {
        Point { x: p.x + 1, y: p.y }
    }
    pub fn move_left(p: Point) -> Point {
        Point { x: p.x - 1, y: p.y }
    }
    pub fn move_down(p: Point) -> Point {
        Point { x: p.x, y: p.y + 1 }
    }
    pub fn move_up(p: Point) -> Point {
        Point { x: p.x, y: p.y - 1 }
    }

    pub fn get_random_with_offset(board: &Board, offset: usize) -> Point {
        let mut rng = rand::thread_rng();

        // Ensure the offset doesn't exceed half the board's dimensions
        let x_min: usize = offset.min(board.x_width / 2);
        let y_min: usize = offset.min(board.y_width / 2);

        let x: i32 = rng.gen_range(x_min..(board.x_width - x_min)) as i32;
        let y: i32 = rng.gen_range(y_min..(board.y_width - y_min)) as i32;

        Point { x, y }
    }
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub struct Snake {
    body: VecDeque<Point>,
    starting_length: usize,
}

impl Snake {
    pub fn new(board: &Board, starting_length: usize) -> Snake {
        let head: Point = Point::get_random_with_offset(board, starting_length);
        let mut current_point: Point = head;
        let mut body: VecDeque<Point> = VecDeque::new();

        body.push_back(head);

        let mut move_function: fn(Point) -> Point = Point::move_right;
        if head.x > board.x_width as i32 / 2 {
            move_function = Point::move_left;
        }

        for _ in 1..starting_length {
            current_point = move_function(current_point);
            body.push_back(current_point);
        }

        Snake {
            body,
            starting_length,
        }
    }

    pub fn move_direction(&mut self, direction: Direction) {
        let head: Point = *self.body.front().unwrap();

        let new_head: Point = match direction {
            Direction::Down => Point::move_down(head),
            Direction::Up => Point::move_up(head),
            Direction::Left => Point::move_left(head),
            Direction::Right => Point::move_right(head),
        };

        self.body.push_front(new_head);
    }

    pub fn shrink_tail(&mut self) {
        self.body.pop_back().unwrap();
    }

    pub fn bit_itself(&self) -> bool {
        if let Some(head) = self.body.front() {
            self.body.iter().skip(1).any(|segment| segment == head)
        } else {
            false // If the body is empty, consider it not in a legal position
        }
    }

    pub fn hit_wall(&self, board: &Board) -> bool {
        if let Some(head) = self.body.front() {
            head.x < 0
                || head.x == board.x_width as i32
                || head.y < 0
                || head.y == board.y_width as i32
        } else {
            false
        }
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.body.contains(&point)
    }
}

pub struct Game {
    pub board: Board,
    pub snake: Snake,
    pub fruit: Point,
    pub running: bool,
}

impl Game {
    pub fn new(x_width: usize, y_width: usize, starting_length: usize) -> Game {
        let board: Board = Board { x_width, y_width };
        let snake: Snake = Snake::new(&board, starting_length);

        let fruit: Point = loop {
            let candidate: Point = Point::get_random_with_offset(&board, 0);
            if !snake.body.contains(&candidate) {
                break candidate;
            }
        };

        Game {
            board,
            snake,
            fruit,
            running: true,
        }
    }
    pub fn get_score(&self) -> u32 {
        (self.snake.body.len() - self.snake.starting_length) as u32
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn progress(&mut self, input: Option<Direction>) {
        if let Some(direction) = input {
            self.snake.move_direction(direction);
        }

        if self.snake.hit_wall(&self.board) || self.snake.bit_itself() {
            self.running = false
        }
        let head_pos = *self.snake.body.front().unwrap();

        if head_pos == self.fruit {
            self.fruit = loop {
                let new_fruit = Point::get_random_with_offset(&self.board, 0);
                if !self.snake.contains_point(new_fruit) {
                    break new_fruit;
                }
            }
        } else {
            self.snake.shrink_tail();
        }
    }

    pub fn get_board_x_width(&self) -> usize {
        self.board.get_x_width()
    }
    pub fn get_board_y_width(&self) -> usize {
        self.board.get_y_width()
    }

    pub fn draw_raw(&self) {
        for y in 0..self.board.y_width {
            for x in 0..self.board.x_width {
                let point = Point {
                    x: x as i32,
                    y: y as i32,
                };
                if self.snake.contains_point(point) {
                    print!("S"); // Snake body
                } else if point == self.fruit {
                    print!("F"); // Fruit
                } else {
                    print!("."); // Empty space
                }
            }
            println!();
        }
        println!();
    }
}

fn debug_run() {
    let mut game = Game::new(12, 16, 3);

    println!("Use W (Up), A (Left), S (Down), D (Right) to move. Press Enter after typing.");

    while game.running {
        println!("Current Score: {}", game.get_score());
        game.draw_raw();

        // Prompt for input
        print!("Enter direction: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        // Parse input to a Direction
        let input = input.trim(); // Remove extra whitespace
        let direction = match input {
            "w" | "W" => Some(Direction::Up),
            "a" | "A" => Some(Direction::Left),
            "s" | "S" => Some(Direction::Down),
            "d" | "D" => Some(Direction::Right),
            _ => None, // Invalid input
        };
        if direction.is_none() {
            println!("Invalid input! Use W, A, S, or D.");
        }

        // Progress the game with the parsed direction
        game.progress(direction);
    }

    println!("Game Over!");
}
