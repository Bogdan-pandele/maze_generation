use maze_logic::{
    generator::generation_algorithms::prim::PrimGenerator,
    grid::shapes::rectangular::RectangularGrid,
};

fn main() {
    let mut rng = rand::rng();
    let maze = maze_logic::build(RectangularGrid::new(15, 10), &mut rng, PrimGenerator);

    print!("{}", maze);
}
