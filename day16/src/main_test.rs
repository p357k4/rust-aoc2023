#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day16_part1_example.txt").unwrap();
        assert_eq!(result, 46);
    }

    #[test]
    fn part1_full() {
        let result = part1("../data/day16_part1.txt").unwrap();
        assert_eq!(result, 8539);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day16_part1_example.txt").unwrap();
        assert_eq!(result, 8674);
    }

    #[test]
    fn part2_full() {
        let result = part2("../data/day16_part1.txt").unwrap();
        assert_eq!(result, 0);
    }
}