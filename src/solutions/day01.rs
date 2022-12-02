pub fn part_one(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let mut cals = input
    .split("\n\n")   
    .map(|elf| 
            elf.lines()
            .map(|cal| 
                    cal.parse::<u32>().unwrap()
                )
                .sum::<u32>()
        ).collect::<Vec<u32>>();

    cals.sort();
    cals.reverse();

    cals[0..3].into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_one(&input), 24000);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 1);
        assert_eq!(part_two(&input), 45000);
    }
}
