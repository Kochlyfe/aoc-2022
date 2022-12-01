use itertools::enumerate;

pub fn part_one(input: &str) -> u32 {
    let it = input.lines();

    let mut max = 0;
    let mut current = 0;

    for el in it {
        if el.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
            continue;
        }
        current += el.parse::<u32>().unwrap();
    }

    max
}

pub fn part_two(input: &str) -> u32 {
    let it = input.lines();
    let num_lines = input.lines().count();

    let mut top_three: Vec<u32> = vec![0, 0, 0];
    let mut current: u32 = 0;

    for (i, el) in enumerate(it) {
        if i == num_lines - 1 {
            current += el.parse::<u32>().unwrap();
            if current > top_three[2] {
                top_three.pop();
                top_three.push(current);
                continue;
            }
        }

        if el.is_empty() {
            if current > top_three[2] {
                top_three.pop();
                top_three.push(current);
                top_three.sort();
                top_three.reverse();
            }
            current = 0;
            continue;
        }
        current += el.parse::<u32>().unwrap();
    }

    top_three.iter().sum()
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
