use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

/// Structure describing a rectanglar region
pub struct Region {
    top: u16,
    left: u16,
    bottom: u16,
    right: u16,
}

impl Region {
    /// Creates a new region
    pub fn new(top: u16, left: u16, bottom: u16, right: u16) -> Self {
        Region {
            top,
            left,
            bottom,
            right,
        }
    }

    /// Returns the left position of the region
    pub fn left(&self) -> u16 {
        self.left
    }

    /// Returns the top position of the region
    pub fn top(&self) -> u16 {
        self.top
    }

    /// Returns the width of the region
    pub fn width(&self) -> u16 {
        (self.right - self.left) + 1
    }

    /// Returns the height of the region
    pub fn height(&self) -> u16 {
        (self.bottom - self.top) + 1
    }

    /// Returns true if the y coordinate is contained in the region
    pub fn contains_y(&self, y: u16) -> bool {
        y >= self.top && y <= self.bottom
    }

    /// Returns a range for all x coordinates
    pub fn x_range(&self) -> RangeInclusive<usize> {
        (self.left as usize)..=(self.right as usize)
    }

    /// Initialises a region for the max region calculation
    pub fn max_init() -> Self {
        Self {
            top: u16::MAX,
            left: u16::MAX,
            bottom: 0,
            right: 0,
        }
    }

    /// Adds a coordinate in the max region calculation
    pub fn max_add(&mut self, x: u16, y: u16) {
        self.top = min(self.top, y);
        self.left = min(self.left, x);
        self.bottom = max(self.bottom, y);
        self.right = max(self.right, x);
    }

    /// Returns true if max has been calculated successfully
    pub fn max_valid(&self) -> bool {
        self.top != u16::MAX
    }
}
