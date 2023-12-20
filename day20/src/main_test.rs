#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day20_part1_example.txt").unwrap();
        assert_eq!(result, 32000000);
    }

    #[test]
    fn part1_example2() {
        let result = part1("../example/day20_part1_example2.txt").unwrap();
        assert_eq!(result, 11687500);
    }

    #[test]
    fn part1_full() {
        let result = part1("../data/day20_part1.txt").unwrap();
        assert_eq!(result, 670984704);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day20_part1_example.txt").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_full() {
        let result = part2("../data/day20_part1.txt").unwrap();
        assert_eq!(result, 0);
    }
}