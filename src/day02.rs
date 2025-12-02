use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use std::ops::Range;

#[derive(Debug)]
struct IdRange {
    low: i64,
    high: i64,
}

impl IdRange {
    fn iter(&self) -> Range<i64> {
        self.low..self.high + 1
    }
}

pub(crate) fn day02(lines: Vec<String>) -> FunctionOutput {
    let id_ranges: Vec<_> = (&lines[0])
        .split(',')
        .map(|r| {
            let (low, high) = r.split_once("-").unwrap();
            IdRange {
                low: low.parse().unwrap(),
                high: high.parse().unwrap(),
            }
        })
        .collect();

    let invalid_ids_p1: Vec<_> = id_ranges
        .iter()
        .map(IdRange::iter)
        .flatten()
        .filter(|id| {
            let id_str = id.to_string();
            if id_str.len() % 2 != 0 {
                return false;
            }
            is_invalid_id(&id_str, id_str.len() / 2)
        })
        .collect();

    let invalid_ids_p2: Vec<_> = id_ranges
        .iter()
        .map(IdRange::iter)
        .flatten()
        .filter(|id| {
            let id_str = id.to_string();
            (1..=(id_str.len() / 2)).any(|stride| is_invalid_id(&id_str, stride))
        })
        .collect();

    IntPair(invalid_ids_p1.iter().sum(), invalid_ids_p2.iter().sum())
}

fn is_invalid_id(id_str: &str, stride: usize) -> bool {
    if stride == 0 {
        return false;
    }
    let num_digits = id_str.len();
    if num_digits % stride != 0 {
        return false;
    }
    id_str
        .chars()
        .zip(id_str.chars().skip(stride))
        .all(|(a, b)| a == b)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day02test.txt");
        assert_eq!(day02(lines), IntPair(1227775554, 4174379265));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day02.txt");
        assert_eq!(day02(lines), IntPair(18893502033, 26202168557));
    }

    #[test]
    fn test_is_invalid_id() {
        assert_eq!(is_invalid_id("123123", 3), true);
        assert_eq!(is_invalid_id("234235", 3), false);
    }
}
