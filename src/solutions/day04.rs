pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let pairs: Vec<_> = line.split(",").collect();
            let first: Vec<u32> = pairs
                .get(0)
                .unwrap()
                .split("-")
                .map(|str| str.parse::<u32>().unwrap())
                .collect();
            let second: Vec<u32> = pairs
                .get(1)
                .unwrap()
                .split("-")
                .map(|str| str.parse::<u32>().unwrap())
                .collect();

            if Some(first.get(0)) >= Some(second.get(0))
                && Some(first.get(1)) <= Some(second.get(1))
            {
                return 1;
            }

            if Some(second.get(0)) >= Some(first.get(0))
                && Some(second.get(1)) <= Some(first.get(1))
            {
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
            let pairs: Vec<_> = line.split(",").collect();
            let first: Vec<u32> = pairs
                .get(0)
                .unwrap()
                .split("-")
                .map(|str| str.parse::<u32>().unwrap())
                .collect();
            let second: Vec<u32> = pairs
                .get(1)
                .unwrap()
                .split("-")
                .map(|str| str.parse::<u32>().unwrap())
                .collect();

            if Some(first.get(1)) >= Some(second.get(0))
                && Some(first.get(1)) <= Some(second.get(1))
            {
                return 1;
            }

            if Some(second.get(1)) >= Some(first.get(0))
                && Some(second.get(1)) <= Some(first.get(1))
            {
                return 1;
            }

            0
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
