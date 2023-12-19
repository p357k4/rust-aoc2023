#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn part1_example() {
        let result = part1("../example/day19_part1_example.txt").unwrap();
        assert_eq!(result, 19114);
    }

    #[test]
    fn part1_full() {
        let result = part1("../data/day19_part1.txt").unwrap();
        assert_eq!(result, 350678);
    }

    #[test]
    fn part2_example1() {
        let result = part2("../example/day19_part1_example.txt").unwrap();
        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn part2_full() {
        let result = part2("../data/day19_part1.txt").unwrap();
        assert_eq!(result, 124831893423809);
    }
}