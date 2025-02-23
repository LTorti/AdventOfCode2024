use num::Integer;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

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

impl FileBlock {
    fn fmt_inner(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FileBlock::FILE(id) => write!(f, "{}", id),
            FileBlock::EMPTY => write!(f, "."),
        }
    }
}

impl Debug for FileBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_inner(f)
    }
}

impl Display for FileBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_inner(f)
    }
}

fn unfold_disk_map(disk_map: &DiskMap) -> Vec<Vec<FileBlock>> {
    let mut counter = 0;
    let mut file_idx = 0;
    disk_map
        .iter()
        .enumerate()
        .map(|(idx, &value)| {
            if idx.is_even() {
                let v = vec![FileBlock::FILE(file_idx); value];
                file_idx += 1;
                counter += value;
                v
            } else {
                counter += value;
                vec![FileBlock::EMPTY; value]
            }
        })
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

fn checksum(disk_map: &Vec<FileBlock>) -> usize {
    disk_map
        .iter()
        .enumerate()
        .map(|(idx, block)| match block {
            &FileBlock::FILE(id) => idx * id,
            &FileBlock::EMPTY => 0,
        })
        .sum()
}

fn challenge_01(disk_map: &Vec<usize>) -> usize {
    let mut unfolded_map = unfold_disk_map(&disk_map)
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    file_compaction(&mut unfolded_map);
    checksum(&unfolded_map)
}

fn get_data() -> DiskMap {
    let input: &str = include_str!("../data/day09.txt");
    let mut data = DiskMap::new();
    for size in input.trim().chars() {
        let size = size.to_digit(10).unwrap() as usize;
        data.push(size);
    }
    data
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
        let actual = unfold_disk_map(&get_disk_map())
            .into_iter()
            .flatten()
            .collect::<Vec<FileBlock>>();
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
        let mut actual = unfold_disk_map(&get_disk_map())
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();
        file_compaction(&mut actual);
        let zipped = actual.into_iter().zip(expected.chars().into_iter());
        zipped.for_each(|(x, y)| assert_eq!(x.to_string(), y.to_string()));
    }

    #[test]
    fn test_dummy() {
        let data = get_data();
        assert_eq!(data.len(), 19999);
        let mut file_blocks = unfold_disk_map(&data)
            .into_iter()
            .flatten()
            .collect::<Vec<FileBlock>>();
        assert_eq!(file_blocks.len(), 95070);
        file_compaction(&mut file_blocks);
        assert_eq!(file_blocks.len(), 95070);
        assert_eq!(checksum(&file_blocks), 6353658451014);
    }
}
