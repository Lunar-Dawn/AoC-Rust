use crate::{impl_neg, impl_operators};
use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Vec2i {
    pub x: i64,
    pub y: i64,
}

impl Vec2i {
    pub const fn new(x: i64, y: i64) -> Vec2i {
        Vec2i { x, y }
    }
}
impl Vec2i {
    pub const UP: Vec2i = Vec2i::new(0, -1);
    pub const UP_RIGHT: Vec2i = Vec2i::new(1, -1);
    pub const RIGHT: Vec2i = Vec2i::new(1, 0);
    pub const DOWN_RIGHT: Vec2i = Vec2i::new(1, 1);
    pub const DOWN: Vec2i = Vec2i::new(0, 1);
    pub const DOWN_LEFT: Vec2i = Vec2i::new(-1, 1);
    pub const LEFT: Vec2i = Vec2i::new(-1, 0);
    pub const UP_LEFT: Vec2i = Vec2i::new(-1, -1);
    pub const DIRECTIONS_CARDINAL: [Vec2i; 4] = [Vec2i::UP, Vec2i::RIGHT, Vec2i::DOWN, Vec2i::LEFT];
    pub const DIRECTIONS_ORDINAL: [Vec2i; 4] = [
        Vec2i::UP_RIGHT,
        Vec2i::DOWN_RIGHT,
        Vec2i::DOWN_LEFT,
        Vec2i::UP_LEFT,
    ];
    pub const DIRECTIONS: [Vec2i; 8] = [
        Vec2i::UP,
        Vec2i::UP_RIGHT,
        Vec2i::RIGHT,
        Vec2i::DOWN_RIGHT,
        Vec2i::DOWN,
        Vec2i::DOWN_LEFT,
        Vec2i::LEFT,
        Vec2i::UP_LEFT,
    ];
}

impl Display for Vec2i {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('<')?;
        Display::fmt(&self.x, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.y, f)?;
        f.write_char('>')
    }
}

impl_operators!(+ Vec2i; (self, rhs) {
    self.x += rhs.x;
    self.y += rhs.y;
});
impl_operators!(- Vec2i; (self, rhs) {
    self.x -= rhs.x;
    self.y -= rhs.y;
});
impl_operators!(* Vec2i, i64; (self, rhs) {
    self.x *= rhs;
    self.y *= rhs;
});
impl_operators!(/ Vec2i, i64; (self, rhs) {
    self.x /= rhs;
    self.y /= rhs;
});
impl_neg!(Vec2i; (self) {
    Vec2i {
        x: -self.x,
        y: -self.y
    }
});
