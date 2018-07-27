#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Pos,
    Neg,
}

impl Direction {
    fn apply(&self, mov: i32) -> i32 {
        if *self == Direction::Pos {
            mov.abs()
        } else {
            -1 * mov.abs()
        }
    }
}

impl From<bool> for Direction {
    fn from(a: bool) -> Direction {
        if a {
            Direction::Pos
        } else {
            Direction::Neg
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Axis {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl From<u8> for Axis {
    fn from(a: u8) -> Axis {
        match a {
            0 => Axis::Zero,
            1 => Axis::One,
            2 => Axis::Two,
            3 => Axis::Three,
            4 => Axis::Four,
            5 => Axis::Five,
            6 => Axis::Six,
            7 => Axis::Seven,
            8 => Axis::Eight,
            9 => Axis::Nine,
            _ => panic!("Number converted to axis is not in [0, 10)"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Move(Direction, Axis),
    Duplicate(Direction, Axis),
    Undo,
    Kill,
    PlacePortal,
    JumpPortal,
    Inc,
    Dec,
    NoOp,
    Read,
    Write,
    Loop(Vec<Instruction>),
}

pub type Program = Vec<Instruction>;
