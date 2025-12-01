use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
enum Dir {
    Left,
    Right,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Dir,
    distance: i64,
}

impl Instruction {
    fn clockwise(&self) -> i64 {
        match self.dir {
            Dir::Left => -self.distance,
            Dir::Right => self.distance,
        }
    }
}

struct Wheel {
    size: i64,
    location: i64,
}

impl Wheel {
    fn rotate(&mut self, instruction: &Instruction) -> (i64, i64) {
        if instruction.distance == 0 {
            return (0, 0);
        }
        let starting_on_zero = self.location == 0;

        let before_norm = self.location + instruction.clockwise();
        self.location = before_norm.rem_euclid(self.size);

        let exact = if self.location == 0 { 1 } else { 0 };
        let mut passing = (before_norm - self.location).abs() / self.size;
        if instruction.dir == Dir::Left {
            if starting_on_zero {
                passing -= 1
            };
            if self.location == 0 {
                passing += 1;
            }
        }
        (exact, passing)
    }
}

pub(crate) fn day01(lines: Vec<String>) -> FunctionOutput {
    let instructions: Vec<Instruction> = lines
        .iter()
        .map(|line| Instruction {
            dir: line.chars().next().unwrap().into(),
            distance: line.chars().skip(1).collect::<String>().parse().unwrap(),
        })
        .collect();

    let mut wheel = Wheel {
        size: 100,
        location: 50,
    };
    let mut exact_zero_count = 0;
    let mut passing_zero_count = 0;
    for instruction in instructions {
        let (exact, passing) = wheel.rotate(&instruction);
        exact_zero_count += exact;
        passing_zero_count += passing;
    }

    IntPair(exact_zero_count, passing_zero_count)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day01test.txt");
        assert_eq!(day01(lines), IntPair(3, 6));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day01.txt");
        assert_eq!(day01(lines), IntPair(1092, 6616));
    }

    #[test]
    fn test_wheel_rotation() {
        let inst = Instruction {
            dir: Dir::Left,
            distance: 68,
        };
        let mut wheel = Wheel {
            size: 100,
            location: 50,
        };
        assert_eq!(wheel.rotate(&inst), (0, 1));
        assert_eq!(wheel.location, 82);
    }
    #[test]
    fn test_wheel_rotation2() {
        let inst = Instruction {
            dir: Dir::Left,
            distance: 1,
        };
        let mut wheel = Wheel {
            size: 5,
            location: 0,
        };
        assert_eq!(wheel.rotate(&inst), (0, 0));
        assert_eq!(wheel.location, 4);
    }
    #[test]
    fn test_wheel_rotation3() {
        let inst = Instruction {
            dir: Dir::Left,
            distance: 28,
        };
        let mut wheel = Wheel {
            size: 5,
            location: 3,
        };
        assert_eq!(wheel.rotate(&inst), (1, 6));
        assert_eq!(wheel.location, 0);
    }
}
