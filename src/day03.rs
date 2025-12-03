use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;

struct BatteryBank(Vec<i64>);

impl BatteryBank {
    fn max_joltage(&self) -> i64 {
        let first_digit = self.0[..self.0.len() - 1].iter().max().unwrap();
        let first_digit_idx = self.0.iter().position(|i| i == first_digit).unwrap();
        let second_digit = self.0[first_digit_idx + 1..].iter().max().unwrap();

        first_digit * 10 + second_digit
    }
}

pub(crate) fn day03(lines: Vec<String>) -> FunctionOutput {
    let battery_banks: Vec<_> = lines
        .iter()
        .map(|l| {
            BatteryBank(
                l.chars()
                    .map(|c| match c.to_string().parse() {
                        Ok(x) => x,
                        Err(_) => panic!("Could not parse `{}`", c),
                    })
                    .collect(),
            )
        })
        .collect();

    let max_joltages: Vec<_> = battery_banks.iter().map(BatteryBank::max_joltage).collect();

    IntPair(max_joltages.iter().sum(), 0)
}

mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day03test.txt");
        assert_eq!(day03(lines), IntPair(357, 0));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day03.txt");
        assert_eq!(day03(lines), IntPair(17244, 0));
    }
}
