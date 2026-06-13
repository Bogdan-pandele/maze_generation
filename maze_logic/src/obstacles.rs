use std::collections::{HashSet, VecDeque};

use rand::{
    RngExt,
    rngs::ThreadRng,
    seq::{IndexedRandom, IteratorRandom},
};

use crate::{algorithms::find_path, cell::CellType, grid::Shape, maze::Maze};

pub fn place_doors_and_keys<S: Shape>(maze: &mut Maze<S>, rng: &mut ThreadRng) {
    let path = find_path(maze, maze.start(), maze.end());
    if let Some(doors_number) = place_doors(maze, &path[1..&path.len() - 1], rng) {
        place_keys(maze, &path, doors_number, rng);
    }
}

fn place_doors<S: Shape>(maze: &mut Maze<S>, path: &[usize], rng: &mut ThreadRng) -> Option<u8> {
    let mut door_id = 0;
    let max_doors = path.len() / 5;
    if max_doors == 0 {
        return None;
    }

    let number_of_doors = rng.random_range(1..=max_doors);

    for i in 0..number_of_doors {
        let region_start = i * path.len() / number_of_doors;
        let region_end = ((i + 1) * path.len() / number_of_doors).min(path.len() - 1);

        let idx = rng.random_range(region_start..region_end);

        maze.place_door(path[idx], path[idx + 1], door_id);
        door_id += 1;
    }

    Some(door_id)
}

fn place_keys<S: Shape>(maze: &mut Maze<S>, path: &[usize], max_door_id: u8, rng: &mut ThreadRng) {
    let path_set: HashSet<usize> = path.iter().copied().collect();
    for key_id in 0..max_door_id {
        let accessible = maze.get_accessible_zone(key_id);
        let dead_ends = find_dead_ends_in_region(maze, &accessible, &path_set);

        let candidate_dead_ends: Vec<usize> = dead_ends[dead_ends.len() / 2..]
            .iter()
            .copied()
            .filter(|&idx| maze.cell_type(idx) == CellType::Normal)
            .collect();

        let key_pos = if let Some(&pos) = candidate_dead_ends.choose(rng) {
            Some(pos)
        } else {
            accessible.iter().choose(rng).copied()
        };

        if let Some(pos) = key_pos {
            maze.set_cell_type(pos, CellType::Key(key_id));
        }
    }
}

fn find_dead_ends_in_region<S: Shape>(
    maze: &mut Maze<S>,
    accessible: &HashSet<usize>,
    path_set: &HashSet<usize>,
) -> Vec<usize> {
    let mut visited = vec![false; maze.size()];
    let mut dead_ends = Vec::<usize>::new();
    let mut queue = VecDeque::new();

    queue.push_back(maze.start());
    visited[maze.start()] = true;
    while let Some(current) = queue.pop_front() {
        for neighbour in maze.get_neighbours(current) {
            if accessible.contains(&neighbour) && !visited[neighbour] {
                queue.push_back(neighbour);
                visited[neighbour] = true;
                if maze.is_dead_end(neighbour) && !path_set.contains(&neighbour) {
                    dead_ends.push(neighbour);
                }
            }
        }
    }
    dead_ends
}
