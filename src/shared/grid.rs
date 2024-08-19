use std::ops::{Index, IndexMut};

use super::point::Point;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let raw: Vec<_> = input.lines().map(str::as_bytes).collect();
        let width = i32::try_from(raw[0].len()).unwrap();
        let height = i32::try_from(raw.len()).unwrap();
        let mut bytes = Vec::with_capacity((width * height) as usize);
        raw.iter().for_each(|slice| bytes.extend_from_slice(slice));
        Self {
            width,
            height,
            bytes,
        }
    }
}

impl<T: Copy + PartialEq> Grid<T> {
    pub fn default_copy<U: Default + Copy>(&self) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![U::default(); (self.width * self.height) as usize],
        }
    }

    pub fn find(&self, needle: T) -> Option<Point> {
        let to_point = |index| {
            let x = i32::try_from(index).unwrap() % self.width;
            let y = i32::try_from(index).unwrap() / self.width;
            Point::new(x, y)
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }

    #[inline]
    pub const fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, point: Point) -> &Self::Output {
        &self.bytes[(self.width * point.y + point.x) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        &mut self.bytes[(self.width * point.y + point.x) as usize]
    }
}
