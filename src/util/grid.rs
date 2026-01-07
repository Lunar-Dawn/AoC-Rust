use crate::util::point2i::Point2i;
use std::fmt::Display;

pub trait Grid<T> {
    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn get(&self, pos: &Point2i) -> Option<&T>;
    fn get_mut(&mut self, pos: &Point2i) -> Option<&mut T>;

    fn get_xy(&self, x: i64, y: i64) -> Option<&T>;
    fn get_xy_mut(&mut self, x: i64, y: i64) -> Option<&mut T>;

    fn in_bounds(&self, pos: &Point2i) -> bool {
        self.in_bounds_xy(pos.x, pos.y)
    }
    fn in_bounds_xy(&self, x: i64, y: i64) -> bool {
        0 <= x && x < self.width() as i64 && 0 <= y && y < self.height() as i64
    }

    fn pos_iter(&self) -> impl Iterator<Item = Point2i> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| Point2i::new(x as i64, y as i64)))
    }

    fn swap(&mut self, pos1: &Point2i, pos2: &Point2i);
}

#[derive(Clone)]
pub struct VectorGrid<T> {
    width: usize,
    height: usize,

    data: Vec<T>,
}
impl<T> VectorGrid<T> {
    pub fn from(width: usize, height: usize, data: Vec<T>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }
    fn index(&self, pos: &Point2i) -> usize {
        self.index_xy(pos.x, pos.y)
    }
    fn index_xy(&self, x: i64, y: i64) -> usize {
        y as usize * self.width + x as usize
    }
}
impl<T> VectorGrid<T>
where
    T: Copy,
{
    pub fn new(width: usize, height: usize, default: T) -> Self {
        VectorGrid {
            width,
            height,
            data: vec![default; width * height],
        }
    }
}
impl<T> Grid<T> for VectorGrid<T> {
    fn height(&self) -> usize {
        self.height
    }
    fn width(&self) -> usize {
        self.width
    }

    fn get(&self, pos: &Point2i) -> Option<&T> {
        if !self.in_bounds(pos) {
            None
        } else {
            self.data.get(self.index(pos))
        }
    }
    fn get_mut(&mut self, pos: &Point2i) -> Option<&mut T> {
        if !self.in_bounds(pos) {
            None
        } else {
            let index = self.index(pos);
            self.data.get_mut(index)
        }
    }

    fn get_xy(&self, x: i64, y: i64) -> Option<&T> {
        if !self.in_bounds_xy(x, y) {
            None
        } else {
            self.data.get(self.index_xy(x, y))
        }
    }
    fn get_xy_mut(&mut self, x: i64, y: i64) -> Option<&mut T> {
        if !self.in_bounds_xy(x, y) {
            None
        } else {
            let index = self.index_xy(x, y);
            self.data.get_mut(index)
        }
    }

    fn swap(&mut self, pos1: &Point2i, pos2: &Point2i) {
        let i1 = self.index(pos1);
        let i2 = self.index(pos2);
        self.data.swap(i1, i2);
    }
}
impl<T> Display for VectorGrid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() as i64 {
            for x in 0..self.width() as i64 {
                write!(f, "{}", self.get_xy(x, y).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
