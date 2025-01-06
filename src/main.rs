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

struct Snake {
    body: VecDeque<Point>
}

impl Snake {
    
}
