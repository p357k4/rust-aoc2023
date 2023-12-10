#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day10_part1_example.txt").unwrap();
        assert_eq!(result, 8);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day10_part1.txt").unwrap();
        assert_eq!(result, 6979);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day10_part2_example1.txt").unwrap();
        assert_eq!(result, 4);
    }
    #[test]
    fn part2_example2() {
        let result = part2("../example/day10_part2_example2.txt").unwrap();
        assert_eq!(result, 10);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day10_part1.txt").unwrap();
        assert_eq!(result, 884);
    }
}