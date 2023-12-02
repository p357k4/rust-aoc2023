#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day02_part1_example.txt").unwrap();
        assert_eq!(result, 8);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day02_part1.txt").unwrap();
        assert_eq!(result, 2776);
    }

    #[test]
    fn part2_example() {
        let result = part2("../example/day02_part1_example.txt").unwrap();
        assert_eq!(result, 2286);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day02_part1.txt").unwrap();
        assert_eq!(result, 68638);
    }
}