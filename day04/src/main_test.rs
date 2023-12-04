#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day04_part1_example.txt").unwrap();
        assert_eq!(result, 13);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day04_part1.txt").unwrap();
        assert_eq!(result, 20855);
    }

    #[test]
    fn part2_example() {
        let result = part2("../example/day04_part1_example.txt").unwrap();
        assert_eq!(result, 30);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day04_part1.txt").unwrap();
        assert_eq!(result, 5489600);
    }
}