use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;

pub(crate) fn day01(lines: Vec<String>) -> FunctionOutput {
    IntPair(0, 0)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day01test.txt");
        assert_eq!(day01(lines), IntPair(3, 0));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day01.txt");
        assert_eq!(day01(lines), IntPair(0, 0));
    }
}
