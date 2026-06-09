use maze_logic::{
    generator::generation_algorithms::prim::PrimGenerator, grid::shapes::hexagon::HexagonalGrid,
};

fn main() {
    let mut rng = rand::rng();
    let maze = maze_logic::build(HexagonalGrid::new(10, 15), &mut rng, PrimGenerator);

    println!("{:?}", maze);
}
