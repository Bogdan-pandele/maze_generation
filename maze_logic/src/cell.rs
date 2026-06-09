#[derive(Clone, PartialEq, Eq, Copy, Debug)]

pub enum WallState {
    Solid,
    Door(u8),
    Open,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum CellType {
    Key(u8),
    #[default]
    Normal,
}
