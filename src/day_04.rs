use std::fs;

fn challenge_01(text: &Vec<Vec<char>>) -> u64 {
    text.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| find_xmas(text, x, y))
                .sum::<u64>()
        })
        .sum()
}

fn find_xmas(text: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    let checkers = vec![
        check_rl, check_lr, check_dr, check_ur, check_dl, check_ul, check_d, check_u,
    ];
    checkers
        .iter()
        .map(|f| f(text, x, y))
        .filter(|x| *x)
        .count() as u64
}

fn check_lr(text: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x + 3 >= text[y].len() {
        return false;
    }
    text[y][x] == 'X' && text[y][x + 1] == 'M' && text[y][x + 2] == 'A' && text[y][x + 3] == 'S'
}

fn check_rl(text: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x < 3 {
        return false;
    }
    text[y][x] == 'X' && text[y][x - 1] == 'M' && text[y][x - 2] == 'A' && text[y][x - 3] == 'S'
}

fn check_d(text: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y + 3 >= text.len() {
        return false;
    }
    text[y][x] == 'X' && text[y + 1][x] == 'M' && text[y + 2][x] == 'A' && text[y + 3][x] == 'S'
}

fn check_dr(text: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x + 3 >= text[y].len() || y + 3 >= text.len() {
        return false;
    }
    text[y][x] == 'X'
        && text[y + 1][x + 1] == 'M'
        && text[y + 2][x + 2] == 'A'
        && text[y + 3][x + 3] == 'S'
}

fn check_dl(text: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x < 3 || y + 3 >= text.len() {
        return false;
    }
    text[y][x] == 'X'
        && text[y + 1][x - 1] == 'M'
        && text[y + 2][x - 2] == 'A'
        && text[y + 3][x - 3] == 'S'
}

fn check_u(text: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y < 3 {
        return false;
    }
    text[y][x] == 'X' && text[y - 1][x] == 'M' && text[y - 2][x] == 'A' && text[y - 3][x] == 'S'
}

fn check_ur(text: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x + 3 >= text[y].len() || y < 3 {
        return false;
    }
    text[y][x] == 'X'
        && text[y - 1][x + 1] == 'M'
        && text[y - 2][x + 2] == 'A'
        && text[y - 3][x + 3] == 'S'
}

fn check_ul(text: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if x < 3 || y < 3 {
        return false;
    }
    text[y][x] == 'X'
        && text[y - 1][x - 1] == 'M'
        && text[y - 2][x - 2] == 'A'
        && text[y - 3][x - 3] == 'S'
}

fn challenge_02(text: &Vec<Vec<char>>) -> u64 {
    text.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| find_mas(text, x, y))
                .sum::<u64>()
        })
        .sum()
}

fn find_mas(text: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    let counters = vec![
        mas_diagonal_lr,
        mas_diagonal_rl,
    ];
    if text[y][x] == 'A' && counters.iter().map(|f| f(text, x, y)).sum::<u64>() == 2 {
        return 1;
    }
    0
}

fn mas_diagonal_lr(text: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    if x > 0 && x < text[y].len() - 1 && y > 0 && y < text.len() - 1 {
        if text[y - 1][x - 1] == 'M' && text[y + 1][x + 1] == 'S' {
            return 1;
        } else if text[y - 1][x - 1] == 'S' && text[y + 1][x + 1] == 'M' {
            return 1;
        }
    }
    0
}

fn mas_diagonal_rl(text: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    if x > 0 && x < text[y].len() - 1 && y > 0 && y < text.len() - 1 {
        if text[y - 1][x + 1] == 'M' && text[y + 1][x - 1] == 'S' {
            return 1;
        } else if text[y - 1][x + 1] == 'S' && text[y + 1][x - 1] == 'M' {
            return 1;
        }
    }
    0
}

fn get_data() -> Vec<Vec<char>> {
    let mut text: Vec<Vec<char>> = Vec::with_capacity(140);
    for line in fs::read_to_string("data/day04_01.txt").unwrap().lines() {
        let mut current = Vec::with_capacity(140);
        for c in line.chars() {
            current.push(c);
        }
        text.push(current);
    }
    text
}

pub fn part_1() -> u64 {
    let text = get_data();
    challenge_01(&text)
}

pub fn part_2() -> u64 {
    let text = get_data();
    challenge_02(&text)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_challenge_01() {
        // Arrange
        let input = vec![
            vec!['.', '.', 'X', '.', '.', '.'],
            vec!['.', 'S', 'A', 'M', 'X', '.'],
            vec!['.', 'A', '.', '.', 'A', '.'],
            vec!['X', 'M', 'A', 'S', '.', 'S'],
            vec!['.', 'X', '.', '.', '.', '.'],
        ];
        // Act
        let count = challenge_01(&input);
        // Assert
        assert_eq!(count, 4);
    }

    #[test]
    fn check_challenge_02() {
        // Arrange
        let input = vec![
            vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
            vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
            vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
            vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        // Act
        let count = challenge_02(&input);
        // Assert
        assert_eq!(count, 9);
    }
}
