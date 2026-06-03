use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Walls: u8 {
        const NORTH = 0b00000001;
        const WEST = 0b00000010;
        const EAST = 0b00000100;
        const SOUTH = 0b00001000;
        const ALL = Self::NORTH.bits() | Self::WEST.bits() | Self::EAST.bits() | Self::SOUTH.bits();
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CellType {
    Door(u8),
    Key(u8),
    #[default]
    Normal,
}

#[derive(Clone, Copy)]
pub struct Cell {
    pub walls: Walls,
    pub cell_type: CellType,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            walls: Walls::ALL,
            cell_type: CellType::default(),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}
