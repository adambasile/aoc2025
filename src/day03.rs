use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;

pub(crate) fn day03(lines: Vec<String>) -> FunctionOutput {
    IntPair(0, 0)
}

mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day03test.txt");
        assert_eq!(day03(lines), IntPair(0, 0));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day03.txt");
        assert_eq!(day03(lines), IntPair(0, 0));
    }
}
