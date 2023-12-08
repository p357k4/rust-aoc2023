#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day08_part1_example.txt").unwrap();
        assert_eq!(result, 2);
    }
    #[test]
    fn part1_full() {
        let result = part1("../data/day08_part1.txt").unwrap();
        assert_eq!(result, 18157);
    }

    #[test]
    fn part2_example() {
        let result = part2("../example/day08_part2_example.txt").unwrap();
        assert_eq!(result, 6);
    }
    #[test]
    fn part2_full() {
        let result = part2("../data/day08_part1.txt").unwrap();
        assert_eq!(result, 14299763833181);
    }
}