use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Board {
    x_width: usize,
    y_width: usize,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
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
        let x_min = offset.min(board.x_width / 2);
        let y_min = offset.min(board.y_width / 2);

        let x = rng.gen_range(x_min..(board.x_width - x_min));
        let y = rng.gen_range(y_min..(board.y_width - y_min));

        Point { x, y }
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Snake {
    body: VecDeque<Point>,
    starting_length: usize,
}

impl Snake {
    pub fn new(board: &Board, starting_length: usize) -> Snake {
        let head = Point::get_random_with_offset(board, starting_length);
        let mut current_point = head.clone();
        let mut body: VecDeque<Point> = VecDeque::new();

        body.push_back(head);

        let mut move_function: fn(Point) -> Point = Point::move_right;
        if head.x > board.x_width / 2 {
            move_function = Point::move_left;
        }

        for _ in 0..starting_length {
            current_point = move_function(current_point);
            body.push_back(current_point);
        }

        Snake {
            body,
            starting_length,
        }
    }

    pub fn move_direction(&mut self, direction: Direction) {
        let head = self.body.front().unwrap().clone();
        let new_head: Point;

        match direction {
            Direction::Down => new_head = Point::move_down(head),
            Direction::Up => new_head = Point::move_up(head),
            Direction::Left => new_head = Point::move_left(head),
            Direction::Right => new_head = Point::move_right(head),
        }

        self.body
            .pop_back()
            .expect("Snake Disappeared... This is a Bug");
        self.body.push_front(new_head);
    }
}

fn main() -> () {}
