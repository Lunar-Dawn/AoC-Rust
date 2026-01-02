use crate::util::point2i::Point2i;

pub trait Grid<T> {
    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn get(&self, pos: &Point2i) -> Option<&T>;
    fn get_mut(&mut self, pos: &Point2i) -> Option<&mut T>;

    fn in_bounds(&self, pos: &Point2i) -> bool {
        0 <= pos.x && pos.x < self.width() as i64 && 0 <= pos.y && pos.y < self.height() as i64
    }

    fn pos_iter(&self) -> impl Iterator<Item = Point2i> {
        (0..self.height())
            .flat_map(|y| (0..self.width()).map(move |x| Point2i::new(x as i64, y as i64)))
    }
}

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
        pos.y as usize * self.width + pos.x as usize
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
}
