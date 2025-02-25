use num::{Integer, ToPrimitive};
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, iter};

type DiskMap = Vec<usize>;

enum FileBlock {
    FILE { id: usize, size: usize },
    EMPTY { size: usize },
}

impl FileBlock {
    fn get_size(&self) -> usize {
        match self {
            &FileBlock::FILE { size, .. } => size,
            &FileBlock::EMPTY { size } => size,
        }
    }

    fn file(id: usize, size: usize) -> Self {
        FileBlock::FILE { id, size }
    }

    fn empty(size: usize) -> Self {
        FileBlock::EMPTY { size }
    }
}

impl Clone for FileBlock {
    fn clone(&self) -> FileBlock {
        match self {
            &FileBlock::FILE { id, size } => FileBlock::FILE { id, size },
            &FileBlock::EMPTY { size } => FileBlock::EMPTY { size },
        }
    }
}

impl FileBlock {
    fn fmt_inner(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut c = String::new();
        let c_size;
        match self {
            FileBlock::FILE { id, size } => {
                c = format!("{}", id);
                c_size = *size;
            }
            FileBlock::EMPTY { size } => {
                c.push('.');
                c_size = *size;
            }
        }
        let mut output = String::new();
        for v in iter::repeat(c).take(c_size) {
            output.push_str(&v);
        }
        write!(f, "{}", output)
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

impl PartialEq for FileBlock {
    fn eq(&self, other: &Self) -> bool {
        match self {
            &FileBlock::FILE { id: self_id, size: self_size } => {
                match other {
                    &FileBlock::FILE { id: other_id, size: other_size } => self_id == other_id && self_size == other_size,
                    _ => false,
                }
            },
            &FileBlock::EMPTY { size: self_size } => {
                match other {
                    &FileBlock::EMPTY { size: other_size } => self_size == other_size,
                    _ => false,
                }
            }
        }
    }
}

impl Eq for FileBlock {}

fn unfold_disk_map(disk_map: &DiskMap) -> Vec<Vec<FileBlock>> {
    let mut file_idx = 0;
    disk_map
        .iter()
        .enumerate()
        .map(|(idx, &value)| {
            if idx.is_even() {
                let v = vec![
                    FileBlock::FILE {
                        id: file_idx,
                        size: value
                    };
                    value
                ];
                file_idx += 1;
                v
            } else {
                vec![FileBlock::EMPTY { size: value }; value]
            }
        })
        .collect()
}

fn file_compaction(disk_map: &mut Vec<FileBlock>) {
    let mut idx = 0;
    let mut reverse_idx = disk_map.len() - 1;
    while idx < reverse_idx {
        if let FileBlock::FILE { .. } = disk_map[reverse_idx] {
            if let FileBlock::EMPTY { .. } = disk_map[idx] {
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
            &FileBlock::FILE { id, .. } => idx * id,
            &FileBlock::EMPTY { .. } => 0,
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

fn build_file_blocks(disk_map: &DiskMap) -> Vec<FileBlock> {
    let mut file_idx = 0;
    let mut file_blocks = disk_map
        .iter()
        .enumerate()
        .map(|(idx, &value)| {
            if idx.is_even() {
                let v = FileBlock::FILE {
                    id: file_idx,
                    size: value,
                };
                file_idx += 1;
                v
            } else {
                FileBlock::EMPTY { size: value }
            }
        })
        .collect::<Vec<FileBlock>>();
    assert_eq!(
        disk_map
            .iter()
            .map(|n| n.to_usize().unwrap())
            .sum::<usize>(),
        file_blocks.iter().map(|b| b.get_size()).sum()
    );
    file_blocks
}

fn compact_free_space(fb: &mut Vec<FileBlock>) {
    loop {
        let length = fb.len();
        'outer: for i in 0..fb.len() {
            if let FileBlock::EMPTY { size: i_size } = fb[i] {
                for j in i + 1..fb.len() {
                    if let FileBlock::EMPTY { size: j_size } = fb[j] {
                        fb[i] = FileBlock::EMPTY { size: i_size + j_size };
                        fb.remove(j_size);
                        break 'outer;
                    }
                }
            }
        }
        if length == fb.len() {
            break;
        }
    }
}

fn block_compaction(fb: &mut Vec<FileBlock>) {
    let mut reverse_idx = fb.len() - 1;
    while reverse_idx > 0 {
        if let FileBlock::FILE { id, size } = fb[reverse_idx] {
            if let Some(free_block_idx) = fb.iter().position(|b| match b {
                FileBlock::EMPTY { size: free_space } =>  size <= *free_space,
                _ => false,
            }) {
                if free_block_idx < reverse_idx {
                    let block = fb[reverse_idx].clone();
                    fb[reverse_idx] = FileBlock::EMPTY {
                        size: block.get_size(),
                    };
                    if fb[free_block_idx].get_size() > size {
                        fb[free_block_idx] = FileBlock::EMPTY {
                            size: fb[free_block_idx].get_size() - size,
                        };
                        fb.insert(free_block_idx, FileBlock::FILE { id, size });
                    } else {
                        fb[free_block_idx] = block.clone();
                    }
                }
            }
        }
        compact_free_space(fb);
        reverse_idx -= 1;
    }
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
                        FileBlock::FILE { id, .. } => *id == $id,
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
                    FileBlock::FILE { .. } => false,
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
        let zipped = actual
            .iter()
            .map(|b| match b {
                FileBlock::FILE { id, .. } => id.to_string(),
                FileBlock::EMPTY { .. } => ".".to_string(),
            })
            .collect::<Vec<String>>()
            .into_iter()
            .zip(expected.chars().into_iter());
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

    #[test]
    fn test_block_compaction() {
        let data = get_disk_map();
        let expected = vec![
            FileBlock::file(0, 2),
            FileBlock::file(9, 2),
            FileBlock::file(2, 1),
            FileBlock::file(1, 3),
            FileBlock::file(7, 3),
            FileBlock::empty(1),
            FileBlock::file(4, 2),
            FileBlock::empty(1),
            FileBlock::file(3, 3),
            FileBlock::empty(1),
            FileBlock::empty(2),
            FileBlock::empty(1),
            FileBlock::file(5, 4),
            FileBlock::empty(1),
            FileBlock::file(6, 4),
            FileBlock::empty(1),
            FileBlock::empty(3),
            FileBlock::empty(1),
            FileBlock::file(8, 4),
            FileBlock::empty(2),
        ];
        let mut actual = build_file_blocks(&data);
        block_compaction(&mut actual);
        expected
            .iter()
            .zip(actual.iter())
            .for_each(|(a, b)| assert_eq!(*a, *b));
    }

    #[test]
    fn test_part_02() {
        let data = get_disk_map();
        let mut file_blocks = build_file_blocks(&data);
        block_compaction(&mut file_blocks);
        let mut sum = 0;
        let mut idx = 0;
        for block in file_blocks {
            match block {
                FileBlock::FILE { id, size } => {
                    for i in 0..size {
                        sum += id * (idx + i);
                    }
                    idx += size;
                }
                _ => (),
            }
        }
        assert_eq!(sum, 28581);
    }
}
