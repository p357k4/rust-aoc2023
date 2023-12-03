#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day03_part1_example.txt").unwrap();
        assert_eq!(result, 4361);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day03_part1.txt").unwrap();
        assert_eq!(result, 527364);
    }

    #[test]
    fn part2_example() {
        let result = part2("../example/day03_part1_example.txt").unwrap();
        assert_eq!(result, 467835);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day03_part1.txt").unwrap();
        assert_eq!(result, 79026871);
    }
}