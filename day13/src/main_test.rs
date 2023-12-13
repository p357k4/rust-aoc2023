#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day13_part1_example.txt").unwrap();
        assert_eq!(result, 405);
    }

    #[test]
    fn part1_full() {
        let result = part1("../data/day13_part1.txt").unwrap();
        assert_eq!(result, 28651);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day13_part1_example.txt").unwrap();
        assert_eq!(result, 400);
    }

    #[test]
    fn part2_full() {
        let result = part2("../data/day13_part1.txt").unwrap();
        assert_eq!(result, 25450);
    }
}