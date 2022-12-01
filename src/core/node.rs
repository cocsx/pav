use std::ops::{AddAssign, Add, Mul};
use super::direction::Direction;


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Node {
    row: i32,
    column: i32
}

impl Node {
    pub fn new(row: i32, column: i32) -> Node {
        Node { row: (row), column: (column) }
    }

    pub fn get_dir( curr: &Node, neigh: &Node) -> Direction {
        let x: i32 = neigh.column - curr.column;
        let y: i32 = neigh.row - curr.row;
        return Direction::new(x, y);
    }

    pub fn row(&self) -> i32 {
        return self.row
    }

    pub fn column(&self) -> i32 {
        return self.column
    }
}

impl Add<Direction> for Node {
    type Output = Self;

    fn add(self, other: Direction) -> Self::Output {
        Node { row: (self.row + other.y()), column: (self.column + other.x())}
    }
    
}

impl Mul<i32> for Node {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        Node { row: (self.row * other), column: (self.column * other)}
    }
}

impl AddAssign<Direction> for Node {
    fn add_assign(&mut self, other: Direction) {
        self.column += other.x();
        self.row    += other.y();
    }
}