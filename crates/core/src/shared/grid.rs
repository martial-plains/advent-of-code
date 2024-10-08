use std::ops::{Index, IndexMut};

use super::point::Point;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i32,
    pub height: i32,
    pub bytes: Vec<T>,
}

impl Grid<u8> {
    /// # Panics
    ///
    /// This function will panic if:
    /// - The input string is empty or does not contain any lines.
    /// - The length of the first line in the input string cannot be converted to an `i32`.
    /// - The number of lines in the input string cannot be converted to an `i32`.
    /// - The input string contains lines with different lengths, as the function assumes a uniform width for all lines.
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
    #[must_use]
    pub fn default_copy<U: Default + Copy>(&self) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            bytes: vec![U::default(); (self.width * self.height) as usize],
        }
    }

    /// # Panics
    ///
    /// This function will panic if:
    /// - The index of the found element cannot be converted to an `i32`.
    /// - The width of the structure (`self.width`) is zero, as division by zero will occur when calculating the `y` coordinate.
    pub fn find(&self, needle: T) -> Option<Point> {
        let to_point = |index| {
            let x = i32::try_from(index).unwrap() % self.width;
            let y = i32::try_from(index).unwrap() / self.width;
            Point::new(x, y)
        };
        self.bytes.iter().position(|&h| h == needle).map(to_point)
    }

    #[inline]
    #[must_use]
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
