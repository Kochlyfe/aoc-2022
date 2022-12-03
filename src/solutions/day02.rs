use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    let scores = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let elf_wins = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);
    let i_win = HashMap::from([("X", "C"), ("Y", "A"), ("Z", "B")]);

    input
        .lines()
        .map(|game| {
            let hands: Vec<&str> = game.split(" ").collect();

            let w = *i_win.get(hands[1]).unwrap();
            let e = *elf_wins.get(hands[0]).unwrap();
            let score = *scores.get(hands[1]).unwrap();

            if w == hands[0] {
                return score + 6;
            } else if e == hands[1] {
                return score;
            } else {
                return score + 3;
            };
        })
        .into_iter()
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let scores = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    let paths = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 1), ("Z", 2)])),
        ("B", HashMap::from([("X", 1), ("Y", 2), ("Z", 3)])),
        ("C", HashMap::from([("X", 2), ("Y", 3), ("Z", 1)])),
    ]);

    input
        .lines()
        .map(|game| {
            let hands: Vec<&str> = game.split(" ").collect();
            let score = scores.get(hands[1]).unwrap();

            return paths.get(hands[0]).unwrap().get(hands[1]).unwrap() + score;
        })
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 12);
    }
}
