#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day07_part1_example.txt").unwrap();
        assert_eq!(result, 6440);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day07_part1.txt").unwrap();
        assert_eq!(result, 253205868);
    }

    #[test]
    fn part2_example() {
        let result = part2("../example/day07_part1_example.txt").unwrap();
        assert_eq!(result, 5905);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day07_part1.txt").unwrap();
        assert_eq!(result, 253907829);
    }
}