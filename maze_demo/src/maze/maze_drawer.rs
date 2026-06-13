use appcui::graphics::{Color, Surface};

pub fn get_key_door_color(id: u8) -> Color {
    match id {
        0 => Color::Yellow,
        1 => Color::Aqua,
        2 => Color::Pink,
        3 => Color::Teal,
        4 => Color::Magenta,
        5 => Color::Blue,
        6 => Color::Olive,
        7 => Color::Green,
        8 => Color::Red,
        _ => Color::DarkRed,
    }
}

pub trait MazeDrawer {
    fn draw(&self, surface: &mut Surface, cell_size: i32, off_x: i32, off_y: i32);
}
