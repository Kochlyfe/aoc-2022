pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (min1, max1, min2, max2) = parse_line_to_elfs_min_max(line);

            if min1 >= min2 && max1 <= max2 {
                return 1;
            }

            if min2 >= min1 && max2 <= max1 {
                return 1;
            }

            0
        })
        .into_iter()
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (min1, max1, min2, max2) = parse_line_to_elfs_min_max(line);

            if max1 >= min2 && max1 <= max2 {
                return 1;
            }

            if max2 >= min1 && max2 <= max1 {
                return 1;
            }

            0
        })
        .into_iter()
        .sum()
}

fn parse_line_to_elfs_min_max(line: &str) -> (u32, u32, u32, u32) {
    let pairs: Vec<Vec<u32>> = line
        .split(",")
        .map(|elf| {
            elf.split("-")
                .map(|str| str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let first: &Vec<u32> = pairs.get(0).unwrap();
    let second: &Vec<u32> = pairs.get(1).unwrap();

    return (
        *first.get(0).unwrap(),
        *first.get(1).unwrap(),
        *second.get(0).unwrap(),
        *second.get(1).unwrap(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), 4);
    }
}
