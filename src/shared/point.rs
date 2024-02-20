use std::{
    hash::Hasher,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const ORIGIN: Self = Self::new(0, 0);
    pub const UP: Self = Self::new(0, -1);
    pub const DOWN: Self = Self::new(0, 1);
    pub const LEFT: Self = Self::new(-1, 0);
    pub const RIGHT: Self = Self::new(1, 0);
    pub const ORTHOGONAL: [Self; 4] = [Self::UP, Self::DOWN, Self::LEFT, Self::RIGHT];

    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn clockwise(self) -> Self {
        Self::new(-self.y, self.x)
    }

    #[inline]
    #[must_use]
    pub const fn counter_clockwise(self) -> Self {
        Self::new(self.y, -self.x)
    }

    #[inline]
    #[must_use]
    pub const fn manhattan(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    #[must_use]
    pub const fn signum(self, other: Self) -> Self {
        Self::new((self.x - other.x).signum(), (self.y - other.y).signum())
    }
}

impl From<u8> for Point {
    #[inline]
    #[must_use]
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => Self::UP,
            b'v' | b'D' => Self::DOWN,
            b'<' | b'L' => Self::LEFT,
            b'>' | b'R' => Self::RIGHT,
            _ => unreachable!(),
        }
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
