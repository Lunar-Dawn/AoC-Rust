use crate::impl_operators;
use crate::util::vec2i::Vec2i;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Point2i {
    pub x: i64,
    pub y: i64,
}

impl Point2i {
    pub const fn new(x: i64, y: i64) -> Point2i {
        Point2i { x, y }
    }

    pub fn vec_to(&self, other: &Self) -> Vec2i {
        Vec2i::new(other.x - self.x, other.y - self.y)
    }
    pub fn distance_sq(&self, other: &Self) -> i64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx * dx + dy * dy
    }

    pub fn distance_manhattan(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbours(&self) -> impl Iterator<Item = Point2i> + use<'_> {
        Vec2i::DIRECTIONS.iter().map(move |d| self + d)
    }
    pub fn neighbours_cardinal(&self) -> impl Iterator<Item = Point2i> + use<'_> {
        Vec2i::DIRECTIONS_CARDINAL.iter().map(move |d| self + d)
    }
    pub fn neighbours_ordinal(&self) -> impl Iterator<Item = Point2i> + use<'_> {
        Vec2i::DIRECTIONS_ORDINAL.iter().map(move |d| self + d)
    }
}
impl Point2i {
    pub const ORIGIN: Point2i = Point2i::new(0, 0);
}

impl Display for Point2i {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;
        Display::fmt(&self.x, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.y, f)?;
        f.write_char(')')
    }
}

impl_operators!(+ Point2i, Vec2i; (self, rhs) {
    self.x += rhs.x;
    self.y += rhs.y;
});
impl_operators!(- Point2i, Vec2i; (self, rhs) {
    self.x -= rhs.x;
    self.y -= rhs.y;
});
impl_operators!(* Point2i, i64; (self, rhs) {
    self.x *= rhs;
    self.y *= rhs;
});
impl_operators!(/ Point2i, i64; (self, rhs) {
    self.x /= rhs;
    self.y /= rhs;
});
