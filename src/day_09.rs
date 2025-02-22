use num::Integer;
use std::fmt::{Debug, Display, Formatter};
use std::fs;

type DiskMap = Vec<usize>;

enum FileBlock {
    FILE(usize),
    EMPTY,
}

impl Clone for FileBlock {
    fn clone(&self) -> FileBlock {
        match self {
            &FileBlock::FILE(id) => FileBlock::FILE(id),
            &FileBlock::EMPTY => FileBlock::EMPTY,
        }
    }
}

impl Copy for FileBlock {}

impl Debug for FileBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &FileBlock::FILE(id) => write!(f, "{}", id),
            &FileBlock::EMPTY => write!(f, "."),
        }
    }
}

impl Display for FileBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            &FileBlock::FILE(id) => write!(f, "{}", id),
            &FileBlock::EMPTY => write!(f, "."),
        }
    }
}

fn unfold_disk_map(disk_map: &DiskMap) -> Vec<FileBlock> {
    let mut file_idx = 0;
    disk_map
        .iter()
        .enumerate()
        .map(|(idx, &value)| {
            if idx.is_even() {
                let v = vec![FileBlock::FILE(file_idx); value];
                file_idx += 1;
                v
            } else {
                vec![FileBlock::EMPTY; value]
            }
        })
        .flatten()
        .collect()
}

fn file_compaction(disk_map: &mut Vec<FileBlock>) {
    let mut idx = 0;
    let mut reverse_idx = disk_map.len() - 1;
    while idx < reverse_idx {
        if let FileBlock::FILE(_) = disk_map[reverse_idx] {
            if let FileBlock::EMPTY = disk_map[idx] {
                disk_map.swap(idx, reverse_idx);
                idx += 1;
                reverse_idx -= 1;
            } else {
                idx += 1;
            }
        } else {
            reverse_idx -= 1;
        }
    }
}

fn challenge_01(disk_map: &Vec<usize>) -> usize {
    let mut unfolded_map = unfold_disk_map(&disk_map);
    file_compaction(&mut unfolded_map);
    unfolded_map
        .iter()
        .enumerate()
        .filter(|(_, block)| match block {
            &FileBlock::FILE(_) => true,
            _ => false,
        })
        .map(|(idx, block)| match block {
            &FileBlock::FILE(id) => idx * id,
            &FileBlock::EMPTY => unreachable!(),
        })
        .sum()
}

fn get_data() -> DiskMap {
    let mut disk_map: DiskMap = DiskMap::new();
    for line in fs::read_to_string("data/day09.txt").unwrap().lines() {
        for c in line.chars() {
            disk_map.push(c as usize);
        }
    }
    disk_map
}

pub fn part_1() -> usize {
    let data = get_data();
    challenge_01(&data)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_disk_map() -> Vec<usize> {
        vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2]
    }

    #[test]
    fn check_challenge_01() {
        let map = get_disk_map();
        let result = challenge_01(&map);
        assert_eq!(result, 1928);
    }

    macro_rules! assert_unfolded_disk_map {
        ( $expected:expr, $actual:expr, $id:expr) => {
            assert_eq!(
                $expected
                    .chars()
                    .filter(|c| c.to_string() == $id.to_string())
                    .count(),
                $actual
                    .iter()
                    .filter(|block| match block {
                        FileBlock::FILE(id) => *id == $id,
                        _ => false,
                    })
                    .count()
            );
        };
    }

    #[test]
    fn check_unfolded_disk_map() {
        let expected = "00...111...2...333.44.5555.6666.777.888899";
        let actual = unfold_disk_map(&get_disk_map());
        assert_eq!(expected.len(), actual.len());
        assert_eq!(
            expected.chars().filter(|c| *c == '.').count(),
            actual
                .iter()
                .filter(|block| match block {
                    FileBlock::FILE(_) => false,
                    _ => true,
                })
                .count()
        );
        for i in 0..9 {
            assert_unfolded_disk_map!(expected, actual, i);
        }
    }

    #[test]
    fn check_compaction() {
        let expected = "0099811188827773336446555566..............";
        let mut actual = unfold_disk_map(&get_disk_map());
        file_compaction(&mut actual);
        let zipped = actual.into_iter().zip(expected.chars().into_iter());
        zipped.for_each(|(x, y)| assert_eq!(x.to_string(), y.to_string()));
    }
}
