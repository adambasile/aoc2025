use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;

pub(crate) fn day02(lines: Vec<String>) -> FunctionOutput {
    IntPair(0, 0)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day02test.txt");
        assert_eq!(day02(lines), IntPair(0, 0));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day02.txt");
        assert_eq!(day02(lines), IntPair(0, 0));
    }
}
