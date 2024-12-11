use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;

fn challenge_01(ordering: &HashMap<u32, HashSet<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    let mut sum = 0;
    for update in updates {
        if is_ordered(update, ordering) {
            sum += update[update.len() / 2];
        }
    }
    sum
}

fn is_ordered(update: &[u32], ordering: &HashMap<u32, HashSet<u32>>) -> bool {
    if update.len() == 1 {
        true
    } else {
        let last = update.last().unwrap();
        let rest = &update[..update.len() - 1];
        if rest
            .iter()
            .any(|i| ordering.contains_key(last) && ordering[last].contains(i))
        {
            false
        } else {
            is_ordered(rest, ordering)
        }
    }
}

fn order(update: &Vec<u32>, ordering: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let mut ordered = update.clone();
    ordered.sort_by(|a, b| {
        if ordering.contains_key(a) && ordering[a].contains(b) {
            Ordering::Less
        } else if ordering.contains_key(b) && ordering[b].contains(a) {
            Ordering::Greater
        } else {
            a.cmp(b)
        }
    });
    ordered
}

fn challenge_02(ordering: &HashMap<u32, HashSet<u32>>, updates: &Vec<Vec<u32>>) -> u32 {
    updates
        .iter()
        .filter(|u| !is_ordered(u, ordering))
        .map(|f| order(&f, ordering))
        .map(|v| v[v.len() / 2])
        .sum()
}

struct Data {
    ordering: HashMap<u32, HashSet<u32>>,
    updates: Vec<Vec<u32>>,
}

fn get_data() -> Data {
    let mut data = Data {
        ordering: HashMap::with_capacity(1200),
        updates: Vec::with_capacity(500),
    };
    let mut first_part = true;
    for line in fs::read_to_string("data/day05_01.txt").unwrap().lines() {
        if line == "" {
            first_part = false;
        } else if first_part {
            let slice = line.split("|").collect::<Vec<&str>>();
            let key: u32 = slice[0].parse().unwrap();
            let val: u32 = slice[1].parse().unwrap();
            data.ordering
                .entry(key)
                .or_insert(HashSet::new())
                .insert(val);
        } else {
            let slice: Vec<u32> = line.split(",").map(|n| n.parse().unwrap()).collect();
            data.updates.push(slice);
        }
    }
    data
}

pub fn part_1() -> u32 {
    let data = get_data();
    challenge_01(&data.ordering, &data.updates)
}

pub fn part_2() -> u32 {
    let data = get_data();
    challenge_02(&data.ordering, &data.updates)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_ordering() -> HashMap<u32, HashSet<u32>> {
        HashMap::from([
            (29, HashSet::from([13])),
            (47, HashSet::from([13, 29, 53, 61])),
            (53, HashSet::from([13, 29])),
            (61, HashSet::from([13, 29, 53])),
            (75, HashSet::from([13, 29, 47, 53, 61])),
            (97, HashSet::from([13, 29, 47, 53, 61, 75])),
        ])
    }

    fn get_updates() -> Vec<Vec<u32>> {
        vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
            vec![75, 97, 47, 61, 53],
            vec![61, 13, 29],
            vec![97, 13, 75, 29, 47],
        ]
    }

    #[test]
    fn check_challenge_01() {
        // Arrange
        let ordering = get_ordering();
        let updates = get_updates();
        // Act
        let result = challenge_01(&ordering, &updates);
        // Assert
        assert_eq!(result, 143);
    }

    #[test]
    fn is_ordered_test() {
        // Arrange
        let ordering = get_ordering();
        // Act and Assert
        assert!(is_ordered(&vec![75, 47, 61, 53, 29], &ordering));
        assert!(is_ordered(&vec![97, 61, 53, 29, 13], &ordering));
        assert!(is_ordered(&vec![75, 29, 13], &ordering));
        assert!(!is_ordered(&vec![75, 97, 47, 61, 53], &ordering));
        assert!(!is_ordered(&vec![61, 13, 29], &ordering));
        assert!(!is_ordered(&vec![97, 13, 75, 29, 47], &ordering));
    }

    #[test]
    fn check_order_01() {
        // Arrange
        let input = vec![75, 97, 47, 61, 53];
        let expected = vec![97, 75, 47, 61, 53];
        // Act
        let actual = order(&input, &get_ordering());
        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_order_02() {
        // Arrange
        let input = vec![97, 13, 75, 29, 47];
        let expected = vec![97, 75, 47, 29, 13];
        // Act
        let actual = order(&input, &get_ordering());
        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_order_03() {
        // Arrange
        let input = vec![61, 13, 29];
        let expected = vec![61, 29, 13];
        // Act
        let actual = order(&input, &get_ordering());
        // Assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn check_challenge_02() {
        // Arrange
        let ordering = get_ordering();
        let updates = get_updates();
        // Act
        let result = challenge_02(&ordering, &updates);
        // Assert
        assert_eq!(result, 123);
    }
}
