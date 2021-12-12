use std::str::FromStr;
use crate::input::InputFileError;

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Ord, Eq, Default, Hash)]
pub struct Point {
    pub x : usize,
    pub y : usize,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = InputFileError;
    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let tokens : Vec<&str> = s.split(",").collect();

        if tokens.len() == 2 {
            Ok(Point {
                x: tokens[0].parse()?,
                y: tokens[1].parse()?,
            })
        } else {
            Err(InputFileError::GeneralError(format!("unable to parse {}", s)))
        }
    }
}

impl Point {
    pub fn from(x : usize, y : usize) -> Self {
        Point {
            x,
            y,
        }
    }

    pub fn add_x(&self, x : usize) -> Self {
        Point {
            x: self.x + x,
            y: self.y,
        }
    }

    pub fn minus_x(&self, x : usize) -> Self {
        Point {
            x: self.x - x,
            y: self.y,
        }
    }

    pub fn add_y(&self, y : usize) -> Self {
        Point {
            x: self.x,
            y: self.y + y,
        }
    }

    pub fn minus_y(&self, y : usize) -> Self {
        Point {
            x: self.x,
            y: self.y - y,
        }
    }
}
