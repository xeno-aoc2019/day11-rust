use std::collections::HashMap;
use std::collections::HashSet;

pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

pub enum Color {
    BLACK,
    WHITE,
}

pub struct Panel {
    painted_squares: HashSet<(i64, i64)>,
    white_squares: HashSet<(i64, i64)>,
    position: (i64, i64),
    direction: Direction,
}

impl Panel {
    pub fn new() -> Panel {
        Panel { painted_squares: HashSet::new(), white_squares: HashSet::new(), position: (0, 0), direction: Direction::NORTH }
    }

    pub fn paint(&mut self, color: Color) {
        match color {
            Color::WHITE => {
                self.white_squares.insert(self.position.clone());
            }
            Color::BLACK => {
                self.white_squares.remove(&self.position);
            }
        }
        self.painted_squares.insert(self.position.clone());
    }

    pub fn turn_left(&mut self) {
        let direction = match self.direction {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH
        };
        self.direction = direction;
    }

    pub fn turn_right(&mut self) {
        let direction = match self.direction {
            Direction::NORTH => Direction::EAST,
            Direction::WEST => Direction::NORTH,
            Direction::SOUTH => Direction::WEST,
            Direction::EAST => Direction::SOUTH
        };
        self.direction = direction;
    }

    pub fn step(&mut self) {
        let x = self.position.0;
        let y = self.position.1;
        let new_pos = match self.direction {
            Direction::NORTH => (x, y + 1),
            Direction::WEST => (x - 1, y),
            Direction::SOUTH => (x, y - 1),
            Direction::EAST => (x + 1, y),
        };
        self.position = new_pos;
    }

    pub fn white_count(&self) -> usize {
        self.white_squares.len()
    }

    pub fn paint_count(&self) -> usize {
        self.painted_squares.len()
    }
    pub fn pos(& self ) -> (i64, i64) {self.position}
}

