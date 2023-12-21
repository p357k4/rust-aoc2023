#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day21_part1_example.txt", 6).unwrap();
        assert_eq!(result, 16);
    }

    #[test]
    fn part1_full() {
        let result = part1("../data/day21_part1.txt", 64).unwrap();
        assert_eq!(result, 3776);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day21_part1_example.txt", 10).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_full() {
        let result = part2("../data/day21_part1.txt", 64).unwrap();
        assert_eq!(result, 0);
    }
}