use std::collections::HashSet;
use std::fs;

use rayon::prelude::*;

type Map = Vec<Vec<char>>;

type Trail = HashSet<Position>;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum DIRECTION {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn walk(pos: &Position, dir: &DIRECTION) -> Position {
    match dir {
        DIRECTION::UP => Position {
            x: pos.x,
            y: if pos.y > 0 { pos.y - 1 } else { usize::MAX },
        },
        DIRECTION::DOWN => Position {
            x: pos.x,
            y: pos.y + 1,
        },
        DIRECTION::LEFT => Position {
            x: if pos.x > 0 { pos.x - 1 } else { usize::MAX },
            y: pos.y,
        },
        DIRECTION::RIGHT => Position {
            x: pos.x + 1,
            y: pos.y,
        },
    }
}

fn turn(dir: &DIRECTION) -> DIRECTION {
    match *dir {
        DIRECTION::UP => DIRECTION::RIGHT,
        DIRECTION::RIGHT => DIRECTION::DOWN,
        DIRECTION::DOWN => DIRECTION::LEFT,
        DIRECTION::LEFT => DIRECTION::UP,
    }
}

fn is_off_limits(pos: &Position, map: &Map) -> bool {
    pos.x >= map[0].len() || pos.y >= map.len()
}

fn get_position(pos: &Position, map: &Map) -> char {
    map[pos.y][pos.x]
}

fn get_starting_position(map: &Map) -> Position {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '^' {
                return Position { x, y };
            }
        }
    }
    unreachable!()
}

fn guard_route(map: &Map) -> Trail {
    let mut trail = Trail::new();
    let mut pos = get_starting_position(map);
    let mut dir = DIRECTION::UP;
    loop {
        let next = walk(&pos, &dir);
        if is_off_limits(&next, &map) {
            return trail;
        } else {
            if get_position(&next, map) == '#' {
                dir = turn(&dir);
            } else {
                pos = next;
                trail.insert(pos);
            }
        }
    }
}

fn challenge_01(map: &Vec<Vec<char>>) -> usize {
    let trail = guard_route(&map);
    trail.len()
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct PositionAndDirection {
    x: usize,
    y: usize,
    dir: DIRECTION,
}

fn blocked_map(pos: &Position, map: &Vec<Vec<char>>) -> Map {
    let mut clone = map.clone();
    clone[pos.y][pos.x] = '#';
    clone
}

type DirectedTrail = HashSet<PositionAndDirection>;

impl Into<Position> for PositionAndDirection {
    fn into(self) -> Position {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<DIRECTION> for PositionAndDirection {
    fn into(self) -> DIRECTION {
        self.dir
    }
}

fn is_guard_in_a_loop(map: &Vec<Vec<char>>) -> bool {
    let mut trail = DirectedTrail::new();
    let start = get_starting_position(map);
    let mut pos = PositionAndDirection {
        x: start.x,
        y: start.y,
        dir: DIRECTION::UP,
    };

    let mut size = trail.len();
    loop {
        let temp = walk(&pos.into(), &pos.into());
        let next = PositionAndDirection {
            x: temp.x,
            y: temp.y,
            dir: pos.dir,
        };
        if is_off_limits(&next.into(), &map) {
            return false;
        } else {
            if get_position(&next.into(), map) == '#' {
                pos.dir = turn(&next.into());
            } else {
                trail.insert(pos);
                pos = next;
                if size < trail.len() {
                    size += 1;
                } else {
                    return true;
                }
            }
        }
    }
}

fn challenge_02(map: &Vec<Vec<char>>) -> usize {
    let trail = guard_route(map);
    let start = get_starting_position(map);

    trail
        .par_iter()
        .filter(|t| **t != start)
        .filter(|t| {
            let blocked_map = blocked_map(t, &map);
            is_guard_in_a_loop(&blocked_map)
        })
        .count()
}

fn get_data() -> Map {
    let mut map = Map::with_capacity(130);
    for line in fs::read_to_string("data/day06.txt").unwrap().lines() {
        map.push(line.chars().collect());
    }
    map
}

pub fn part_1() -> usize {
    let data = get_data();
    challenge_01(&data)
}

pub fn part_2() -> usize {
    let data = get_data();
    challenge_02(&data)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_map() -> Vec<Vec<char>> {
        vec![
            vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
            vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
        ]
    }

    #[test]
    fn check_challenge_01() {
        let map = get_map();
        let result = challenge_01(&map);
        assert_eq!(result, 41);
    }

    #[test]
    fn check_challenge_02() {
        let map = get_map();
        let result = challenge_02(&map);
        assert_eq!(result, 6);
    }
}
