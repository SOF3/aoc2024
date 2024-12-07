#![allow(dead_code)]

use std::iter;

#[derive(Clone, Copy)]
pub struct GridView<'a> {
    input: &'a [u8],
    width: u32,
    height: u32,
}

impl<'a> GridView<'a> {
    pub fn new(input: &'a impl AsRef<[u8]>) -> Self {
        let input = input.as_ref();
            let width= input.iter().position(|&b| b == b'\n').unwrap() as u32+1;
        Self {
            input,
            width,
            height: (input.len() as u32).div_ceil(width),
        }
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
    pub fn left(self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }
    pub fn right(self, grid: GridView) -> Option<Self> {
        Some(Self {
            x: match self.x.checked_add(1)? {
                x if x < grid.width - 1 => x,
                _ => return None,
            },
            y: self.y,
        })
    }
    pub fn up(self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }
    pub fn down(self, grid: GridView) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: match self.y.checked_add(1)? {
                y if y < grid.height - 1 => y,
                _ => return None,
            },
        })
    }

    pub fn direct(self, direct: Direct, grid: GridView) -> Option<Self> {
        match direct {
            Direct::Left => self.left(),
            Direct::Right => self.right(grid),
            Direct::Up => self.up(),
            Direct::Down => self.down(grid),
        }
    }

    pub fn direct_diagonal(self, direct: DirectDiagonal, grid: GridView) -> Option<Self> {
        match direct {
            DirectDiagonal::Left => self.left(),
            DirectDiagonal::Right => self.right(grid),
            DirectDiagonal::Up => self.up(),
            DirectDiagonal::Down => self.down(grid),
            DirectDiagonal::LeftUp => self.left().and_then(Self::up),
            DirectDiagonal::RightUp => self.right(grid).and_then(Self::up),
            DirectDiagonal::LeftDown => self.left().and_then(|loc| loc.down(grid)),
            DirectDiagonal::RightDown => self.right(grid).and_then(|loc| loc.down(grid)),
        }
    }

    fn direct_any_iter<F>(self, mutate: F, grid: GridView) -> impl Iterator<Item = Self> + use<'_, F>
    where F: Fn(GridLoc, GridView) -> Option<GridLoc>{
        let mut loc = Some(self);
        iter::from_fn(move || {
            let output = loc?;
            loc = mutate(output, grid);
            Some(output)
        })
    }

    pub fn direct_iter(self, direct: Direct, grid: GridView) -> impl Iterator<Item = Self> + use<'_> {
        self.direct_any_iter(move |loc, grid| loc.direct(direct, grid), grid)
    }

    pub fn direct_diagonal_iter(self, direct: DirectDiagonal, grid: GridView) -> impl Iterator<Item = Self> + use<'_> {
        self.direct_any_iter(move |loc, grid| loc.direct_diagonal(direct, grid), grid)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direct {
    Left,
    Right,
    Up,
    Down,
}

impl Direct {
    pub const ALL: [Self; 4] = [Self::Left, Self::Right, Self::Up, Self::Down];
}

#[derive(Debug, Clone, Copy)]
pub enum DirectDiagonal {
    Left,
    Right,
    Up,
    Down,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
}

impl DirectDiagonal {
    pub const ALL: [Self; 8] = [
        Self::Left, Self::Right, Self::Up, Self::Down,
    Self::LeftUp, Self::RightUp, Self::LeftDown, Self::RightDown,
    ];
}
