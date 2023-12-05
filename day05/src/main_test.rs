#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day05_part1_example.txt").unwrap();
        assert_eq!(result, 35);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day05_part1.txt").unwrap();
        assert_eq!(result, 173706076);
    }

    #[test]
    fn part2_example() {
        let result = part2("../example/day05_part1_example.txt").unwrap();
        assert_eq!(result, 46);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day05_part1.txt").unwrap();
        assert_eq!(result, 11611182);
    }
}