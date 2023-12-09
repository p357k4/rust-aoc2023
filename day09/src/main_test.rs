#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day09_part1_example.txt").unwrap();
        assert_eq!(result, 114);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day09_part1.txt").unwrap();
        assert_eq!(result, 1974913025);
    }

    #[test]
    fn part2_example() {
        let result = part2("../example/day09_part1_example.txt").unwrap();
        assert_eq!(result, 2);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day09_part1.txt").unwrap();
        assert_eq!(result, 884);
    }
}