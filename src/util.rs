#![allow(dead_code)]

use std::{iter, ops};

#[derive(Clone, Copy)]
pub struct GridView<Input> {
    pub input: Input,
    pub shape: GridShape,
}

#[derive(Clone, Copy)]
pub struct GridShape {
    pub width:  u32,
    pub height: u32,
}

impl<Input: AsRef<[u8]>> GridView<Input> {
    pub fn new(input: Input) -> Self {
        let width = input.as_ref().iter().position(|&b| b == b'\n').unwrap() as u32 + 1;
        let height = (input.as_ref().len() as u32).div_ceil(width);
        Self { input, shape: GridShape { width, height } }
    }

    pub fn get(&self, loc: GridLoc) -> Option<u8> {
        self.input.as_ref().get(self.shape.loc_to_index(loc) as usize).copied()
    }
}

impl<Input: AsMut<[u8]>> GridView<Input> {
    pub fn get_mut(&mut self, loc: GridLoc) -> Option<&mut u8> {
        self.input.as_mut().get_mut(self.shape.loc_to_index(loc) as usize)
    }
}

impl GridShape {
    pub fn loc_to_index(&self, loc: GridLoc) -> u32 { loc.y * self.width + loc.x }

    pub fn index_to_loc(&self, index: usize) -> Option<GridLoc> {
        let y = index as u32 / self.width;
        let x = index as u32 % self.width;
        if x < self.width - 1 && y < self.height {
            Some(GridLoc { x, y })
        } else {
            None
        }
    }
}

impl<Input> From<GridView<Input>> for GridShape {
    fn from(grid: GridView<Input>) -> Self { grid.shape }
}

impl<'a, Input> From<&'a GridView<Input>> for GridShape {
    fn from(grid: &'a GridView<Input>) -> Self { grid.shape }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridLoc {
    pub x: u32,
    pub y: u32,
}

impl GridLoc {
    pub fn left(self) -> Option<Self> { Some(Self { x: self.x.checked_sub(1)?, y: self.y }) }
    pub fn right(self, grid: impl Into<GridShape>) -> Option<Self> {
        Some(Self {
            x: match self.x.checked_add(1)? {
                x if x < grid.into().width - 1 => x,
                _ => return None,
            },
            y: self.y,
        })
    }
    pub fn up(self) -> Option<Self> { Some(Self { x: self.x, y: self.y.checked_sub(1)? }) }
    pub fn down(self, grid: impl Into<GridShape>) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: match self.y.checked_add(1)? {
                y if y < grid.into().height => y,
                _ => return None,
            },
        })
    }

    pub fn direct(self, direct: impl Direct, grid: impl Into<GridShape> + Copy) -> Option<Self> {
        direct.apply(self, grid)
    }

    pub fn direct_iter<D: Direct, G: Into<GridShape> + Copy>(
        self,
        direct: D,
        grid: G,
    ) -> impl Iterator<Item = Self> + use<D, G> {
        let mut loc = Some(self);
        iter::from_fn(move || {
            let output = loc?;
            loc = direct.apply(output, grid);
            Some(output)
        })
    }

    #[must_use]
    pub fn add(mut self, vector: GridVector, grid: impl Into<GridShape>) -> Option<Self> {
        self.x = self.x.checked_add_signed(vector.x)?;
        self.y = self.y.checked_add_signed(vector.y)?;

        let shape: GridShape = grid.into();
        if self.x < shape.width - 1 && self.y < shape.height {
            Some(self)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridVector {
    pub x: i32,
    pub y: i32,
}

impl ops::Sub<GridLoc> for GridLoc {
    type Output = GridVector;

    fn sub(self, rhs: GridLoc) -> Self::Output {
        GridVector { x: self.x as i32 - rhs.x as i32, y: self.y as i32 - rhs.y as i32 }
    }
}

impl ops::Mul<i32> for GridVector {
    type Output = GridVector;

    fn mul(mut self, rhs: i32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

pub trait Direct: Copy + 'static {
    const ALL: &[Self];

    fn apply(self, loc: GridLoc, grid: impl Into<GridShape> + Copy) -> Option<GridLoc>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DirectTaxicab {
    Left,
    Right,
    Up,
    Down,
}

impl DirectTaxicab {
    pub fn clockwise(self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
        }
    }
}

impl Direct for DirectTaxicab {
    const ALL: &[Self] = &[Self::Left, Self::Right, Self::Up, Self::Down];

    fn apply(self, loc: GridLoc, grid: impl Into<GridShape>) -> Option<GridLoc> {
        match self {
            Self::Left => loc.left(),
            Self::Right => loc.right(grid),
            Self::Up => loc.up(),
            Self::Down => loc.down(grid),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DirectBoth {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl Direct for DirectBoth {
    const ALL: &[Self] = &[
        Self::Left,
        Self::Right,
        Self::Up,
        Self::Down,
        Self::LeftUp,
        Self::RightUp,
        Self::LeftDown,
        Self::RightDown,
    ];

    fn apply(self, loc: GridLoc, grid: impl Into<GridShape> + Copy) -> Option<GridLoc> {
        match self {
            Self::Left => loc.left(),
            Self::Right => loc.right(grid),
            Self::Up => loc.up(),
            Self::Down => loc.down(grid),
            Self::LeftUp => loc.left().and_then(GridLoc::up),
            Self::RightUp => loc.right(grid).and_then(GridLoc::up),
            Self::LeftDown => loc.left().and_then(|loc2| loc2.down(grid)),
            Self::RightDown => loc.right(grid).and_then(|loc2| loc2.down(grid)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DirectDiagonal {
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl Direct for DirectDiagonal {
    const ALL: &[Self] = &[Self::LeftUp, Self::RightUp, Self::LeftDown, Self::RightDown];

    fn apply(self, loc: GridLoc, grid: impl Into<GridShape> + Copy) -> Option<GridLoc> {
        match self {
            Self::LeftUp => loc.left().and_then(GridLoc::up),
            Self::RightUp => loc.right(grid).and_then(GridLoc::up),
            Self::LeftDown => loc.left().and_then(|loc2| loc2.down(grid)),
            Self::RightDown => loc.right(grid).and_then(|loc2| loc2.down(grid)),
        }
    }
}
