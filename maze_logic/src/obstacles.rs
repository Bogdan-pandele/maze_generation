use std::collections::{HashSet, VecDeque};

use rand::{RngExt, rngs::ThreadRng, seq::IndexedRandom};

use crate::{algorithms::find_path, cell::CellType, maze::Maze};

pub fn place_doors_and_keys(maze: &mut Maze, rng: &mut ThreadRng) {
    let path = find_path(maze, maze.start(), maze.end());
    if let Some(doors) = place_doors(maze, &path[1..&path.len() - 1], rng) {
        place_keys(maze, &path, &doors, rng);
    }
}

fn place_doors(maze: &mut Maze, path: &[usize], rng: &mut ThreadRng) -> Option<Vec<usize>> {
    let mut door_id = 0;
    let max_doors = path.len() / 5;
    if max_doors == 0 {
        return None;
    }

    let number_of_doors = rng.random_range(1..=max_doors);
    let mut doors = Vec::with_capacity(number_of_doors);

    for i in 0..number_of_doors {
        let region_start = i * path.len() / number_of_doors;
        let region_end = ((i + 1) * path.len() / number_of_doors).min(path.len() - 1);

        let door_pos = path[rng.random_range(region_start..region_end)];

        if maze.cell(door_pos).cell_type == CellType::Normal {
            maze.set_cell_type(door_pos, CellType::Door(door_id));
            door_id += 1;
            doors.push(door_pos);
        }
    }

    Some(doors)
}

fn place_keys(maze: &mut Maze, path: &[usize], doors: &[usize], rng: &mut ThreadRng) {
    let path_set: HashSet<usize> = path.iter().copied().collect();
    for i in 0..doors.len() {
        let accessible = maze.get_accessible_zone(&doors[i..]);
        let dead_ends = find_dead_ends_in_region(maze, &accessible, &path_set);

        let candidate_dead_ends = &dead_ends[dead_ends.len() / 2..];
        if let CellType::Door(id) = maze.cell(doors[i]).cell_type {
            if let Some(&key_pos) = candidate_dead_ends.choose(rng) {
                maze.set_cell_type(key_pos, CellType::Key(id));
            }
        }
    }

    fn find_dead_ends_in_region(
        maze: &mut Maze,
        accessible: &HashSet<usize>,
        path_set: &HashSet<usize>,
    ) -> Vec<usize> {
        let mut visited = vec![false; maze.height() * maze.width()];
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
}
