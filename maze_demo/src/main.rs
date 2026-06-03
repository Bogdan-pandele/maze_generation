fn main() {
    let mut rng = rand::rng();
    let maze = maze_logic::build(5, 5, &mut rng);
    print!("{}", maze);
}
