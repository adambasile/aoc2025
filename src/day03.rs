use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;

type BatteryBank = Vec<i64>;

fn get_max_joltage(battery_bank: &BatteryBank, num_batteries: usize) -> i64 {
    let mut batteries: Vec<&i64> = vec![];
    let mut last_battery_idx = 0;
    for batteries_remaining in (0..num_batteries).rev() {
        let battery = battery_bank[last_battery_idx..battery_bank.len() - batteries_remaining]
            .iter()
            .max()
            .unwrap();
        last_battery_idx += 1
            + (battery_bank[last_battery_idx..]
                .iter()
                .position(|i| i == battery)
                .unwrap());
        batteries.push(battery)
    }

    batteries
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, battery)| *battery * 10_i64.pow(idx as u32))
        .sum()
}

pub(crate) fn day03(lines: Vec<String>) -> FunctionOutput {
    let battery_banks: Vec<BatteryBank> = lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c.to_string().parse() {
                    Ok(x) => x,
                    Err(_) => panic!("Could not parse `{}`", c),
                })
                .collect()
        })
        .collect();

    let max_joltages_2: Vec<_> = battery_banks
        .iter()
        .map(|bb| get_max_joltage(bb, 2))
        .collect();
    let max_joltages_12: Vec<_> = battery_banks
        .iter()
        .map(|bb| get_max_joltage(bb, 12))
        .collect();

    IntPair(max_joltages_2.iter().sum(), max_joltages_12.iter().sum())
}

mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day03test.txt");
        assert_eq!(day03(lines), IntPair(357, 3121910778619));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day03.txt");
        assert_eq!(day03(lines), IntPair(17244, 171435596092638));
    }
}
