#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day12_part1_example.txt").unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn part1_full() {
        let result = part1("../data/day12_part1.txt").unwrap();
        assert_eq!(result, 6488);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day12_part1_example.txt").unwrap();
        assert_eq!(result, 525152);
    }

    #[test]
    fn part2_full() {
        let result = part2("../data/day12_part1.txt").unwrap();
        assert_eq!(result, 0);
    }
}