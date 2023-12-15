#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day15_part1_example.txt").unwrap();
        assert_eq!(result, 1320);
    }

    #[test]
    fn part1_full() {
        let result = part1("../data/day15_part1.txt").unwrap();
        assert_eq!(result, 509167);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day15_part1_example.txt").unwrap();
        assert_eq!(result, 145);
    }

    #[test]
    fn part2_full() {
        let result = part2("../data/day15_part1.txt").unwrap();
        assert_eq!(result, 259333);
    }
}