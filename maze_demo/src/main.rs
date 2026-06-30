use appcui::prelude::*;
use appcui::ui::appbar::{Button, MenuButton};
use appcui::{
    graphics::Surface,
    system::App,
    ui::{LayoutBuilder, Window, common::traits::OnPaint, desktop::events::DesktopEvents, window},
};
use maze_logic::build;
use maze_logic::generator::MazeGenerator;
use maze_logic::generator::generation_algorithms::cut::CuttingGenerator;
use maze_logic::generator::generation_algorithms::prim::PrimGenerator;
use maze_logic::generator::generation_algorithms::wilson::WilsonGenerator;
use maze_logic::grid::shapes::hexagon::HexagonalGrid;
use maze_logic::grid::shapes::rectangle::RectangularGrid;
use maze_logic::grid::shapes::triangle::TriangularGrid;

use crate::maze::maze_game::{ActiveMaze, MazeGame};

mod maze;

enum MazeSize {
    Small,
    Medium,
    Large,
}

impl MazeSize {
    fn dimensions(&self) -> (usize, usize) {
        match self {
            MazeSize::Small => (10, 5),
            MazeSize::Medium => (13, 7),
            MazeSize::Large => (18, 8),
        }
    }

    fn cell_size(&self) -> i32 {
        match self {
            MazeSize::Small => 7,
            MazeSize::Medium => 5,
            MazeSize::Large => 4,
        }
    }
}

enum Shape {
    Rectangular,
    Triangular,
    Hexagonal,
}

#[derive(Clone, Copy)]
enum Algorithm {
    Prim,
    Wilson,
    Cutting
}

impl MazeGenerator for Algorithm {
    fn generate<S: maze_logic::grid::Shape>(&self, maze: &mut maze_logic::maze::Maze<S>, rng: &mut rand::prelude::ThreadRng) {
        match self {
            Algorithm::Prim => PrimGenerator.generate(maze, rng),
            Algorithm::Wilson => WilsonGenerator.generate(maze, rng),
            Algorithm::Cutting => CuttingGenerator.generate(maze, rng),
        }
    }
}

#[Desktop(events = [MenuEvents, DesktopEvents, AppBarEvents],  
        commands = [RectangularShape, TriangularShape, HexagonalShape, PrimAlgorithm, SmallMaze, MediumMaze,LargeMaze, WilsonAlgorithm, CuttingAlgorithm]
    )
]
struct MyDesktop {
    shape: Shape,
    algorithm: Algorithm,
    size: MazeSize,
    menu_shape: Handle<MenuButton>,

    menu_algorithm: Handle<MenuButton>,
    generate_button: Handle<Button>,
    menu_size: Handle<MenuButton>,
    exit_button: Handle<Button>,
    game_window: Handle<Window>,
}

impl MyDesktop {
    fn new() -> Self {
        Self {
            base: Desktop::new(),
            shape: Shape::Rectangular,
            algorithm: Algorithm::Prim,
            size: MazeSize::Medium,
            menu_shape: Handle::None,
            menu_algorithm: Handle::None,
            generate_button: Handle::None,
            menu_size: Handle::None,
            exit_button: Handle::None,
            game_window: Handle::None,
        }
    }

    fn generate_maze(&mut self) {
        let (w, h) = self.size.dimensions();


        let active_maze = match self.shape {
            Shape::Rectangular => {
                ActiveMaze::Rectangular(build(RectangularGrid::new(w, h), self.algorithm))
            }
            Shape::Triangular => {
                ActiveMaze::Triangular(build(TriangularGrid::new(w, h), self.algorithm))
            }
            Shape::Hexagonal => {
                ActiveMaze::Hexagonal(build(HexagonalGrid::new(w, h), self.algorithm))
            }
        };

        let handle = self.game_window;
        if let Some(window) = self.window_mut(handle) {
            window.close();
        }

        let mut win = Window::new(
            "Maze",
            LayoutBuilder::new().dock(Dock::Fill).build(),
            window::Flags::Sizeable,
        );

        win.add(MazeGame::new(active_maze, self.size.cell_size()));
        self.game_window = self.add_window(win);
    }
}

impl DesktopEvents for MyDesktop {
    fn on_start(&mut self) {
        self.menu_shape = self.appbar().add(MenuButton::new(
            "Shape",
            menu!(
                "class:MyDesktop, items=[{Rectangular, selected:true, cmd: RectangularShape}, 
                {Triangular, selected: false, cmd: TriangularShape}, 
                {Hexagonal, selected:false, cmd: HexagonalShape}]"
            ),
            1,
            appbar::Side::Left,
        ));

        self.menu_algorithm = self.appbar().add(MenuButton::new(
            "Algorithm",
            menu!("class:MyDesktop, items=[{Prim, selected:true, cmd: PrimAlgorithm}, {Wilson, selected:false, cmd: WilsonAlgorithm}, 
            {Cutting, selected: false, cmd: CuttingAlgorithm}]"),
            0,
            appbar::Side::Left,
        ));

        self.menu_size = self.appbar().add(MenuButton::new(
            "Size",
            menu!(
                "class:MyDesktop, items=[{Small, selected:false, cmd: SmallMaze}, 
                                        {Medium, selected:true, cmd: MediumMaze}, 
                                        {Large, selected:false, cmd: LargeMaze}
                                        ]"
            ),
            2,
            appbar::Side::Left,
        ));

        self.generate_button = self
            .appbar()
            .add(Button::new("Generate ", 3, appbar::Side::Left));

        self.exit_button = self
            .appbar()
            .add(Button::new("Exit", 4, appbar::Side::Left));
        self.generate_maze();
    }
}

impl MenuEvents for MyDesktop {
    fn on_select(
        &mut self,
        _menu: Handle<Menu>,
        _item: Handle<menu::SingleChoice>,
        command: mydesktop::Commands,
    ) {
        match command {
            mydesktop::Commands::PrimAlgorithm => self.algorithm = Algorithm::Prim,
            mydesktop::Commands::WilsonAlgorithm => self.algorithm = Algorithm::Wilson,
            mydesktop::Commands::CuttingAlgorithm => self.algorithm = Algorithm::Cutting,
            mydesktop::Commands::RectangularShape => self.shape = Shape::Rectangular,
            mydesktop::Commands::TriangularShape => self.shape = Shape::Triangular,
            mydesktop::Commands::HexagonalShape => self.shape = Shape::Hexagonal,
            mydesktop::Commands::SmallMaze => self.size = MazeSize::Small,
            mydesktop::Commands::MediumMaze => self.size = MazeSize::Medium,
            mydesktop::Commands::LargeMaze => self.size = MazeSize::Large,
            
        }
    }
}

impl AppBarEvents for MyDesktop {
    fn on_update(&self, appbar: &mut AppBar) {
        appbar.show(self.menu_algorithm);
        appbar.show(self.generate_button);
        appbar.show(self.menu_size);
        appbar.show(self.menu_shape);
        appbar.show(self.exit_button);
    }

    fn on_button_click(&mut self, button: Handle<appbar::Button>) {
        if button == self.generate_button {
            self.generate_maze();
        } else if button == self.exit_button {
            self.close();
        }
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new()
        .desktop(MyDesktop::new())
        .app_bar()
        .build()?
        .run();
    Ok(())
}
