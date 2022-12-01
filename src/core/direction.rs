use std::{ops, panic};

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub struct Direction {
    y: i32,
    x: i32
}

impl ops::Neg for Direction {
   type Output = Direction;
    fn neg(self) -> Self::Output {
        return Direction { x: -self.x, y: -self.y};
    }
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Direction {
        Direction { x: (x), y: (y) }
    }

    pub fn x(&self) -> i32 {
        return self.x;
    }

    pub fn y(&self) -> i32 {
        return self.y;
    }

    pub fn index(ind: i32) -> Direction {
        match ind {
            0  => Direction { x: 1, y: 0},  // 0 right
            1  => Direction { x: -1, y: 0}, // 1 left
            2  => Direction { x: 0, y: 1},  // 2 down
            3  => Direction { x: 0, y: -1}, // 3 top
            _ => panic!("[FATAL] NOT A VALID DIRECTION!")
        }
    }
    
    #[allow(dead_code)]
    pub fn is_left(&self) -> bool {
        return self == &Direction { x: -1, y: 0}
    }

    pub fn is_right(&self) -> bool {
        return self == &Direction { x: 1, y: 0}
    }

    pub fn is_top(&self) -> bool {
        return self == &Direction { x: 0, y: -1}
    }
    
    pub fn is_down(&self) -> bool {
        return self == &Direction { x: 0, y: 1}
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}