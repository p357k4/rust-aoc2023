#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day06_part1_example.txt").unwrap();
        assert_eq!(result, 288);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day06_part1.txt").unwrap();
        assert_eq!(result, 303600);
    }

    #[test]
    fn part2_example() {
        let result = part2("../example/day06_part1_example.txt").unwrap();
        assert_eq!(result, 71503);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day06_part1.txt").unwrap();
        assert_eq!(result, 11611182);
    }
}