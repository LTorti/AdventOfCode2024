use std::collections::{HashMap, HashSet};
use std::fs;

type Map = Vec<Vec<char>>;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

fn get_all_antennas(map: &Map) -> HashMap<char, Vec<Position>> {
    let mut antennas_map = HashMap::<char, Vec<Position>>::new();
    map.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, antenna)| {
            if *antenna != '.' {
                let pos = Position {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                let entry = antennas_map.entry(*antenna).or_insert(vec![]);
                entry.push(pos);
            }
        });
    });
    antennas_map
}

fn generate_all_pairs(antennas: &Vec<Position>) -> Vec<[Position; 2]> {
    let mut pairs = Vec::<[Position; 2]>::new();
    for i in 0..antennas.len() {
        for j in i + 1..antennas.len() {
            pairs.push([antennas[i], antennas[j]]);
        }
    }
    pairs
}

fn generate_anti_nodes(first: Position, second: Position) -> [Position; 2] {
    let first_antinode = Position {
        x: first.x - (second.x - first.x),
        y: first.y - (second.y - first.y),
    };
    let second_antinode = Position {
        x: second.x - (first.x - second.x),
        y: second.y - (first.y - second.y),
    };
    [first_antinode, second_antinode]
}

fn is_inside_map(map: &Map, position: Position) -> bool {
    position.x >= 0
        && position.y >= 0
        && position.y < map.len().try_into().unwrap()
        && position.x < map[0].len().try_into().unwrap()
}

fn challenge_01(map: &Map) -> usize {
    let mut anti_nodes = HashSet::<Position>::new();
    let antennas = get_all_antennas(map);
    for (_, positions) in antennas {
        let pairs = generate_all_pairs(&positions);
        for pair in pairs {
            let antinodes_pair = generate_anti_nodes(pair[0], pair[1]);
            anti_nodes.insert(antinodes_pair[0]);
            anti_nodes.insert(antinodes_pair[1]);
        }
    }
    anti_nodes
        .iter()
        .filter(|node| is_inside_map(map, **node))
        .count()
}

fn get_data() -> Map {
    let mut map = Map::new();
    for line in fs::read_to_string("data/day08.txt").unwrap().lines() {
        map.push(line.chars().collect());
    }
    map
}

pub fn part_1() -> usize {
    let map = get_data();
    challenge_01(&map)
}

fn generate_anti_nodes_with_harmonics(
    first: Position,
    second: Position,
    map: &Map,
) -> HashSet<Position> {
    let mut anti_nodes = HashSet::<Position>::new();
    anti_nodes.insert(first.clone());
    anti_nodes.insert(second.clone());
    let mut first_antinode = Position {
        x: first.x - (second.x - first.x),
        y: first.y - (second.y - first.y),
    };
    while is_inside_map(map, first_antinode) {
        anti_nodes.insert(first_antinode.clone());
        first_antinode.x -= second.x - first.x;
        first_antinode.y -= second.y - first.y;
    }
    let mut second_antinode = Position {
        x: second.x - (first.x - second.x),
        y: second.y - (first.y - second.y),
    };
    while is_inside_map(map, second_antinode) {
        anti_nodes.insert(second_antinode.clone());
        second_antinode.x -= first.x - second.x;
        second_antinode.y -= first.y - second.y;
    }
    anti_nodes
}

fn challenge_02(map: &Map) -> usize {
    let mut anti_nodes = HashSet::<Position>::new();
    let antennas = get_all_antennas(map);
    for (_, positions) in antennas {
        let pairs = generate_all_pairs(&positions);
        for pair in pairs {
            anti_nodes.extend(generate_anti_nodes_with_harmonics(pair[0], pair[1], map));
        }
    }
    anti_nodes.len()
}

pub fn part_2() -> usize {
    let map = get_data();
    challenge_02(&map)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_map() -> Map {
        vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '0', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '0', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', 'A', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', 'A', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ]
    }

    #[test]
    fn check_challenge_01() {
        let map = get_map();
        let result = challenge_01(&map);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_antinodes_01() {
        let first = Position { x: 5, y: 2 };
        let second = Position { x: 4, y: 4 };
        let anti_nodes = generate_anti_nodes(first, second);
        assert_eq!(anti_nodes[0], Position { x: 6, y: 0 });
        assert_eq!(anti_nodes[1], Position { x: 3, y: 6 });
    }

    #[test]
    fn test_antinodes_02() {
        let first = Position { x: 5, y: 2 };
        let second = Position { x: 7, y: 3 };
        let anti_nodes = generate_anti_nodes(first, second);
        assert_eq!(anti_nodes[0], Position { x: 3, y: 1 });
        assert_eq!(anti_nodes[1], Position { x: 9, y: 4 });
    }

    #[test]
    fn check_challenge_02() {
        let map = get_map();
        let result = challenge_02(&map);
        assert_eq!(result, 34);
    }
}
