use std::{
    fmt::{Debug, Display},
    ops::Add,
    str::FromStr,
};

use anyhow::anyhow;
use enum_iterator::{all, Sequence};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where
    T: Default,
{
    pub fn origin() -> Self {
        Point::default()
    }
}

impl Point<i8> {
    pub fn manhattan_distance(&self, p: &Self) -> u8 {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }
}

impl Point<i16> {
    pub fn manhattan_distance(&self, p: &Self) -> u16 {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }
}

impl Point<i32> {
    pub fn manhattan_distance(&self, p: &Self) -> u32 {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }
}

impl<T> Debug for Point<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}

impl<T> FromStr for Point<T>
where
    T: FromStr<Err = std::num::ParseIntError>,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("no ',' in point '{}'", s))?;

        Ok(Point {
            x: l.parse()?,
            y: r.parse()?,
        })
    }
}

impl<T> Point<T> {
    pub fn towards(&self, dx: T, dy: T) -> Point<T>
    where
        T: Copy + Add<Output = T>,
    {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

impl Add<CardinalDirection> for Point<i8> {
    type Output = Self;

    fn add(self, rhs: CardinalDirection) -> Self::Output {
        let (dx, dy) = rhs.deltas();
        Point {
            x: self.x + dx as i8,
            y: self.y + dy as i8,
        }
    }
}

impl Add<CompassDirection> for Point<i8> {
    type Output = Self;

    fn add(self, rhs: CompassDirection) -> Self::Output {
        let (dx, dy) = rhs.deltas();
        Point {
            x: self.x + dx as i8,
            y: self.y + dy as i8,
        }
    }
}

impl Point<i8> {
    pub fn towards_compass_direction(&self, d: CompassDirection) -> Self {
        let (dx, dy) = d.deltas();
        Point {
            x: self.x + dx as i8,
            y: self.y + dy as i8,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Sequence)]
pub enum CompassDirection {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
}

impl CompassDirection {
    pub fn values() -> impl Iterator<Item = CompassDirection> {
        all::<CompassDirection>()
    }

    pub fn deltas(self) -> (i32, i32) {
        match self {
            CompassDirection::N => (0, -1),
            CompassDirection::S => (0, 1),
            CompassDirection::W => (-1, 0),
            CompassDirection::E => (1, 0),
            CompassDirection::NE => (1, -1),
            CompassDirection::NW => (-1, -1),
            CompassDirection::SE => (1, 1),
            CompassDirection::SW => (-1, 1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Sequence)]
pub enum CardinalDirection {
    N,
    S,
    W,
    E,
}

impl CardinalDirection {
    pub fn for_code(c: char) -> Self {
        match c {
            '^' => Self::N,
            'v' => Self::S,
            '<' => Self::W,
            '>' => Self::E,
            _ => panic!("unknown cardinal direction: '{c}'"),
        }
    }

    pub fn deltas(self) -> (i32, i32) {
        match self {
            CardinalDirection::N => (0, -1),
            CardinalDirection::S => (0, 1),
            CardinalDirection::W => (-1, 0),
            CardinalDirection::E => (1, 0),
        }
    }
}
