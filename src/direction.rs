use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn isOpposite(&self, other: &Direction) -> bool {
        self == &Direction::Up && other == &Direction::Down
            || self == &Direction::Down && other == &Direction::Up
            || self == &Direction::Right && other == &Direction::Left
            || self == &Direction::Left && other == &Direction::Right
    }
}

impl TryFrom<u8> for Direction {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'w' => Ok(Direction::Up),
            b's' => Ok(Direction::Down),
            b'd' => Ok(Direction::Right),
            b'a' => Ok(Direction::Left),
            _ => Err("Can't convert".into()),
        }
    }
}
