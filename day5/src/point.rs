use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl Point {
    pub fn normalize(self) -> Self {
        Self {
            x: if self.x != 0 { self.x/self.x.abs() } else { 0 },
            y: if self.y != 0 { self.y/self.y.abs() } else { 0 },
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(",").unwrap();
        Ok(Point {
            x: x_str.parse().unwrap(),
            y: y_str.parse().unwrap(),
        })
    }
}
