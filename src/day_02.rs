use std::fs;

struct Report {
    levels: Vec<i64>,
}

impl Report {
    fn safe(&self) -> bool {
        let mut orientation = 0i64;
        for i in 1..self.levels.len() {
            let diff = self.levels[i - 1] - self.levels[i];
            if diff < -3 || diff > 3 {
                return false;
            }
            if orientation == 0 && diff != 0 {
                orientation = diff;
            } else if (diff == 0) || (orientation < 0 && diff > 0) || (orientation > 0 && diff < 0)
            {
                return false;
            }
        }
        true
    }

    fn safe_with_naive_dampener(&self) -> bool {
        if self.safe() {
            return true;
        } else {
            for i in 0..self.levels.len() {
                if damper(self, i).safe() {
                    return true;
                }
            }
        }
        false
    }
}

fn damper(report: &Report, level: usize) -> Report {
    let mut levels = report.levels.clone();
    levels.remove(level);
    Report { levels }
}

fn challenge_01(reports: Vec<Report>) -> u64 {
    let mut safe = 0u64;

    reports.iter().for_each(|r| {
        if r.safe() {
            safe += 1;
        }
    });

    safe
}

fn challenge_02(reports: Vec<Report>) -> u64 {
    let mut safe = 0u64;

    reports.iter().for_each(|r| {
        if r.safe_with_naive_dampener() {
            safe += 1;
        }
    });

    safe
}

fn get_data() -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::with_capacity(1000);
    for line in fs::read_to_string("data/day02_01.txt").unwrap().lines() {
        reports.push(Report {
            levels: line
                .split(" ")
                .map(|lvl: &str| -> i64 { lvl.parse().unwrap() })
                .collect(),
        });
    }
    reports
}

pub fn part_1() -> u64 {
    let reports = get_data();
    challenge_01(reports)
}

pub fn part_2() -> u64 {
    let reports = get_data();
    challenge_02(reports)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_challenge_01() {
        // Arrange
        let reports: Vec<Report> = vec![
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ];
        // Act
        let actual = challenge_01(reports);
        // Assert
        assert_eq!(actual, 2);
    }

    #[test]
    fn check_challenge_02() {
        // Arrange
        let reports: Vec<Report> = vec![
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        ];
        // Act
        let actual = challenge_02(reports);
        // Assert
        assert_eq!(actual, 4);
    }
}
