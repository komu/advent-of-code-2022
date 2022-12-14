use std::{
    fmt::{Debug, Display},
    ops::Add,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
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

impl Point<i16> {
    pub fn manhattan_distance(&self, p: &Self) -> u16 {
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

impl<T> Point<T>
where
    T: Copy + Add<Output = T>,
{
    pub fn towards(&self, dx: T, dy: T) -> Point<T> {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}
