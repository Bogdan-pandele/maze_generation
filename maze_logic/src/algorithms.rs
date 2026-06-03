use std::collections::VecDeque;

use crate::maze::Maze;

pub fn find_furthest(maze: &Maze, start: usize) -> usize {
    let mut visited = vec![false; maze.width() * maze.height()];
    let mut queue = VecDeque::new();
    let mut furthest = maze.start();

    queue.push_front(start);
    visited[start] = true;

    while let Some(current) = queue.pop_back() {
        furthest = current;

        for neighbour in maze.get_neighbours(current) {
            if !visited[neighbour] {
                visited[neighbour] = true;
                queue.push_front(neighbour);
            }
        }
    }
    furthest
}

pub fn find_path(maze: &Maze, from: usize, to: usize) -> Vec<usize> {
    let mut visited = vec![false; maze.width() * maze.height()];
    let mut path = Vec::<usize>::new();
    dfs(maze, from, to, &mut path, &mut visited);
    path
}

fn dfs(
    maze: &Maze,
    current: usize,
    end: usize,
    path: &mut Vec<usize>,
    visited: &mut [bool],
) -> bool {
    if current == end {
        path.push(current);
        return true;
    }
    visited[current] = true;
    path.push(current);

    for neighbour in maze.get_neighbours(current) {
        if !visited[neighbour] && dfs(maze, neighbour, end, path, visited) {
            return true;
        }
    }
    path.pop();
    false
}
