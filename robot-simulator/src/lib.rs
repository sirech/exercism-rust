// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        let new_d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self { d: new_d, ..self }
    }

    pub fn turn_left(self) -> Self {
        let new_d = match self.d {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        Self { d: new_d, ..self }
    }

    pub fn advance(self) -> Self {
        let (new_x, new_y) = match self.d {
            Direction::North => (self.x, self.y + 1),
            Direction::West => (self.x - 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::East => (self.x + 1, self.y),
        };
        Self {
            x: new_x,
            y: new_y,
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut r = self;
        for order in instructions.chars() {
            println!("insturction is {}", order);
            r = match order {
                'L' => r.turn_left(),
                'R' => r.turn_right(),
                'A' => r.advance(),
                _ => panic!("Instruction not in [L, R, A]"),
            }
        }
        r
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
