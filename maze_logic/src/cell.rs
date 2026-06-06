#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum CellType {
    Key(u8),
    #[default]
    Normal,
    Void,
}
