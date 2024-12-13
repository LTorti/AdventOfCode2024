use std::fs;
use std::usize;

use rayon::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Calibration {
    result: i64,
    numbers: Vec<i64>,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum OPERATION {
    PLUS,
    MULT,
    CONCAT,
}

type Operators = Vec<OPERATION>;

fn evaluate(calibration: &Calibration, operators: &Operators) -> i64 {
    let mut result = calibration.numbers.clone();
    result.reverse();
    for idx in 0..operators.len() {
        let a = result.pop().unwrap();
        let b = result.pop().unwrap();
        match operators[idx] {
            OPERATION::PLUS => result.push(a + b),
            OPERATION::MULT => result.push(a * b),
            OPERATION::CONCAT => {
                let b_str = b.to_string();
                let a_str = a.to_string();
                let c: String = format!("{}{}", a_str, b_str).parse().unwrap();
                result.push(c.parse().unwrap());
            }
        }
        if *result.last().unwrap() > calibration.result {
            return *result.last().unwrap();
        }
    }
    result.pop().unwrap()
}

fn generate_all_operations(size: usize) -> Vec<Operators> {
    if size == 1 {
        vec![vec![OPERATION::PLUS], vec![OPERATION::MULT]]
    } else {
        let prev = generate_all_operations(size - 1);
        let mut result = Vec::<Operators>::with_capacity(prev.len() * 2);
        for op in prev {
            let mut plus = vec![OPERATION::PLUS];
            plus.append(&mut op.clone());
            let mut mult = vec![OPERATION::MULT];
            mult.append(&mut op.clone());
            result.push(plus);
            result.push(mult);
        }
        result
    }
}

fn is_calibration_valid(calibration: &Calibration) -> bool {
    let operations = generate_all_operations(calibration.numbers.len() - 1);
    operations
        .par_iter()
        .filter(|op| evaluate(calibration, op) == calibration.result)
        .count()
        > 0
}

fn challenge_01(calibrations: &Vec<Calibration>) -> i64 {
    calibrations
        .par_iter()
        .filter(|c| is_calibration_valid(c))
        .map(|c| c.result)
        .sum()
}

fn get_data() -> Vec<Calibration> {
    let mut data = Vec::<Calibration>::with_capacity(850);
    for line in fs::read_to_string("data/day07.txt").unwrap().lines() {
        let slice = line.split(':').collect::<Vec<&str>>();
        data.push(Calibration {
            result: slice[0].trim().parse().unwrap(),
            numbers: slice[1]
                .split(' ')
                .filter(|x| x.trim().len() > 0)
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>(),
        });
    }
    data
}

pub fn part_1() -> i64 {
    let data = get_data();
    challenge_01(&data)
}

fn generate_all_operations_with_concat(size: usize) -> Vec<Operators> {
    if size == 1 {
        vec![
            vec![OPERATION::PLUS],
            vec![OPERATION::MULT],
            vec![OPERATION::CONCAT],
        ]
    } else {
        let prev = generate_all_operations_with_concat(size - 1);
        let mut result = Vec::<Operators>::with_capacity(prev.len() * 3);
        for op in prev {
            let mut plus = vec![OPERATION::PLUS];
            plus.append(&mut op.clone());
            result.push(plus);
            let mut mult = vec![OPERATION::MULT];
            mult.append(&mut op.clone());
            result.push(mult);
            let mut concat = vec![OPERATION::CONCAT];
            concat.append(&mut op.clone());
            result.push(concat)
        }
        result
    }
}

fn is_calibration_valid_with_concat(calibration: &Calibration) -> bool {
    let operations = generate_all_operations_with_concat(calibration.numbers.len() - 1);
    operations
        .par_iter()
        .filter(|op| evaluate(calibration, op) == calibration.result)
        .count()
        > 0
}

fn challenge_02(calibrations: &Vec<Calibration>) -> i64 {
    calibrations
        .par_iter()
        .filter(|c| is_calibration_valid_with_concat(c))
        .map(|c| c.result)
        .sum()
}

pub fn part_2() -> i64 {
    let data = get_data();
    challenge_02(&data)
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_calibrations() -> Vec<Calibration> {
        vec![
            Calibration {
                result: 190,
                numbers: vec![10, 19],
            },
            Calibration {
                result: 3267,
                numbers: vec![81, 40, 27],
            },
            Calibration {
                result: 83,
                numbers: vec![17, 5],
            },
            Calibration {
                result: 156,
                numbers: vec![15, 6],
            },
            Calibration {
                result: 7290,
                numbers: vec![6, 8, 6, 15],
            },
            Calibration {
                result: 161011,
                numbers: vec![16, 10, 13],
            },
            Calibration {
                result: 192,
                numbers: vec![17, 8, 14],
            },
            Calibration {
                result: 21037,
                numbers: vec![9, 7, 18, 13],
            },
            Calibration {
                result: 292,
                numbers: vec![11, 6, 16, 20],
            },
        ]
    }

    #[test]
    fn check_challenge_01() {
        let calibrations = get_calibrations();
        let result = challenge_01(&calibrations);
        assert_eq!(result, 3749);
    }

    #[test]
    fn check_generate_all_operations() {
        let result = generate_all_operations(6);
        assert_eq!(result.len(), 64);
    }

    #[test]
    fn check_is_calibration_valid() {
        let calibrations = get_calibrations();
        assert!(is_calibration_valid(calibrations.get(0).unwrap()));
        assert!(is_calibration_valid(calibrations.get(1).unwrap()));
        assert!(!is_calibration_valid(calibrations.get(2).unwrap()));
        assert!(!is_calibration_valid(calibrations.get(3).unwrap()));
        assert!(!is_calibration_valid(calibrations.get(4).unwrap()));
        assert!(!is_calibration_valid(calibrations.get(5).unwrap()));
        assert!(!is_calibration_valid(calibrations.get(6).unwrap()));
        assert!(!is_calibration_valid(calibrations.get(7).unwrap()));
        assert!(is_calibration_valid(calibrations.get(8).unwrap()));
    }

    #[test]
    fn check_challenge_02() {
        let calibrations = get_calibrations();
        let result = challenge_02(&calibrations);
        assert_eq!(result, 11387);
    }

    #[test]
    fn check_generate_all_operations_with_concat() {
        let result = generate_all_operations_with_concat(6);
        assert_eq!(result.len(), 729);
    }

    #[test]
    fn check_is_calibration_valid_with_concat() {
        let calibrations = get_calibrations();
        assert!(is_calibration_valid_with_concat(calibrations.get(0).unwrap()));
        assert!(is_calibration_valid_with_concat(calibrations.get(1).unwrap()));
        assert!(!is_calibration_valid_with_concat(calibrations.get(2).unwrap()));
        assert!(is_calibration_valid_with_concat(calibrations.get(3).unwrap()));
        assert!(is_calibration_valid_with_concat(calibrations.get(4).unwrap()));
        assert!(!is_calibration_valid_with_concat(calibrations.get(5).unwrap()));
        assert!(is_calibration_valid_with_concat(calibrations.get(6).unwrap()));
        assert!(!is_calibration_valid_with_concat(calibrations.get(7).unwrap()));
        assert!(is_calibration_valid_with_concat(calibrations.get(8).unwrap()));
    }

}
