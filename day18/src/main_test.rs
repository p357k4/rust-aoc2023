#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day18_part1_example.txt").unwrap();
        assert_eq!(result, 38);
    }

    #[test]
    fn part1_full() {
        let result = part1("../data/day18_part1.txt").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day18_part1_example.txt").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_full() {
        let result = part2("../data/day18_part1.txt").unwrap();
        assert_eq!(result, 0);
    }
}