#![allow(dead_code)]

use std::iter;

#[derive(Clone, Copy)]
pub struct GridView<'a> {
    input:  &'a [u8],
    width:  u32,
    height: u32,
}

impl<'a> GridView<'a> {
    pub fn new(input: &'a impl AsRef<[u8]>) -> Self {
        let input = input.as_ref();
        let width = input.iter().position(|&b| b == b'\n').unwrap() as u32 + 1;
        Self { input, width, height: (input.len() as u32).div_ceil(width) }
    }

    pub fn get(&self, loc: GridLoc) -> Option<u8> {
        self.input.get((loc.y * self.width + loc.x) as usize).copied()
    }

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

#[derive(Debug, Clone, Copy)]
pub struct GridLoc {
    x: u32,
    y: u32,
}

impl GridLoc {
    pub fn left(self) -> Option<Self> { Some(Self { x: self.x.checked_sub(1)?, y: self.y }) }
    pub fn right(self, grid: GridView) -> Option<Self> {
        Some(Self {
            x: match self.x.checked_add(1)? {
                x if x < grid.width - 1 => x,
                _ => return None,
            },
            y: self.y,
        })
    }
    pub fn up(self) -> Option<Self> { Some(Self { x: self.x, y: self.y.checked_sub(1)? }) }
    pub fn down(self, grid: GridView) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: match self.y.checked_add(1)? {
                y if y < grid.height => y,
                _ => return None,
            },
        })
    }

    pub fn direct(self, direct: impl Direct, grid: GridView) -> Option<Self> {
        direct.apply(self, grid)
    }

    pub fn direct_iter<D: Direct>(
        self,
        direct: D,
        grid: GridView,
    ) -> impl Iterator<Item = Self> + use<'_, D> {
        let mut loc = Some(self);
        iter::from_fn(move || {
            let output = loc?;
            loc = direct.apply(output, grid);
            Some(output)
        })
    }
}

pub trait Direct: Copy + 'static {
    const ALL: &[Self];

    fn apply(self, loc: GridLoc, grid: GridView) -> Option<GridLoc>;
}

#[derive(Debug, Clone, Copy)]
pub enum DirectTaxicab {
    Left,
    Right,
    Up,
    Down,
}

impl Direct for DirectTaxicab {
    const ALL: &[Self] = &[Self::Left, Self::Right, Self::Up, Self::Down];

    fn apply(self, loc: GridLoc, grid: GridView) -> Option<GridLoc> {
        match self {
            Self::Left => loc.left(),
            Self::Right => loc.right(grid),
            Self::Up => loc.up(),
            Self::Down => loc.down(grid),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

    fn apply(self, loc: GridLoc, grid: GridView) -> Option<GridLoc> {
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

#[derive(Debug, Clone, Copy)]
pub enum DirectDiagonal {
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl Direct for DirectDiagonal {
    const ALL: &[Self] = &[Self::LeftUp, Self::RightUp, Self::LeftDown, Self::RightDown];

    fn apply(self, loc: GridLoc, grid: GridView) -> Option<GridLoc> {
        match self {
            Self::LeftUp => loc.left().and_then(GridLoc::up),
            Self::RightUp => loc.right(grid).and_then(GridLoc::up),
            Self::LeftDown => loc.left().and_then(|loc2| loc2.down(grid)),
            Self::RightDown => loc.right(grid).and_then(|loc2| loc2.down(grid)),
        }
    }
}
