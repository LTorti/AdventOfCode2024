use regex::Regex;
use std::fs;

fn challenge_01(mem: &str) -> u64 {
    let mut sum = 0u64;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [x, y]) in re.captures_iter(mem).map(|group| group.extract()) {
        sum += x.parse::<u64>().unwrap() * y.parse::<u64>().unwrap();
    }
    sum
}

fn next_instruction(mem: &str) -> (Option<usize>, usize) {
    let next_do = mem.find("do()");
    let next_dont = mem.find("don't()");
    match (next_do, next_dont) {
        (Some(i), Some(j)) => {
            if i < j {
                (Some(i), "do()".len())
            } else {
                (Some(j), "don't()".len())
            }
        }
        (Some(i), None) => {
            (Some(i), "do()".len())
        }
        (None, Some(j)) => {
            (Some(j), "don't()".len())
        }
        (None, None) => {
            (None, 0)
        }
    }
}

fn challenge_02(mem: &Vec<String>) -> u64 {
    let mut sum = 0u64;

    let mut enable = true;

    for unit in mem {
        let mut idx = 0;
        while idx < unit.len() {
            let (bloc, offset) = next_instruction(&unit[idx..]);
            match bloc {
                Some(pos) => {
                    if enable {
                        sum += challenge_01(&unit[idx..idx + pos]);
                    }
                    idx = idx + pos + offset;
                    enable = if offset == "do()".len() { true } else { false };
                }
                None => {
                    if enable {
                        sum += challenge_01(&unit[idx..]);
                    }
                    idx = unit.len();
                }
            }
        }
    }

    sum
}

pub fn part_1() -> u64 {
    get_data().iter().map(|x| challenge_01(x)).sum()
}

pub fn part_2() -> u64 {
    challenge_02(&get_data())
}

fn get_data() -> Vec<String> {
    let mut mem: Vec<String> = Vec::with_capacity(6);
    for line in fs::read_to_string("data/day03_01.txt").unwrap().lines() {
        mem.push(String::from(line));
    }
    mem
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_challenge_01() {
        // Arrange
        let mem = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        // Act
        let actual = challenge_01(mem);
        // Assert
        assert_eq!(actual, 161);
    }

    #[test]
    fn check_next_instruction_dont() {
        // Arrange
        let mem = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = (mem.find("don't()"), "don't()".len());
        // Act
        let actual = next_instruction(mem);
        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_next_instruction_do() {
        // Arrange
        let mem = "_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = (mem.find("do()"), "do()".len());
        // Act
        let actual = next_instruction(mem);
        // Assert
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_challenge_02() {
        // Arrange
        let mem = vec!(String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"));
        // Act
        let actual = challenge_02(&mem);
        // Assert
        assert_eq!(actual, 48);
    }
}
