use std::fs;
use std::collections::HashMap;

struct Data {
    pub col_1: Vec<u64>,
    pub col_2: Vec<u64>,
}
fn challenge_01(mut data: Data) -> u64 {
    let mut sum = 0;
    data.col_1.sort();
    data.col_2.sort();
    while data.col_1.len() > 0 {
        let val_1 = data.col_1.pop().unwrap();
        let val_2 = data.col_2.pop().unwrap();
        sum += val_1.abs_diff(val_2);
    }
    sum
}

fn challenge_02(data: Data) -> u64 {
    let mut sum = 0;
    let mut map : HashMap<u64,u64> = HashMap::new();
    for value in data.col_1.iter() {
        if !map.contains_key(value) {
            map.insert(*value, data.col_2.iter().filter(|&x| *x == *value).count() as u64);
        }
        sum += value * map.get(value).unwrap();
    }
    sum
}

fn get_data() -> Data {
    let mut col_1 = Vec::with_capacity(1000);
    let mut col_2 = Vec::with_capacity(1000);
    for line in fs::read_to_string("data/day01_01.txt").unwrap().lines() {
        col_1.push(line[0..5].parse().unwrap());
        col_2.push(line[8..13].parse().unwrap());
    }
    Data { col_1, col_2 }
}

pub fn part_1() -> u64 {
    let data = get_data();
    challenge_01(data)
}

pub fn part_2() -> u64 {
    let data = get_data();
    challenge_02(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_01() {
        // Arrange
        let col_1 = vec![3, 4, 2, 1, 3, 3];
        let col_2 = vec![4, 3, 5, 3, 9, 3];
        let data = Data { col_1, col_2 };
        // Act
        let actual = challenge_01(data);
        // Assert
        assert_eq!(actual, 11);
    }

    #[test]
    fn test_example_02() {
        // Arrange
        let col_1 = vec![3, 4, 2, 1, 3, 3];
        let col_2 = vec![4, 3, 5, 3, 9, 3];
        let data = Data { col_1, col_2 };
        // Act
        let actual = challenge_02(data);
        // Assert
        assert_eq!(actual, 31);
    }
}
