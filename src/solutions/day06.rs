pub fn part_one(input: &str) -> usize {

    let mut counter = 0;
    while counter < input.len() {
        let current = &input[counter..counter + 4];
        if unique_chars(current) {
            return counter + 4;
        }

        counter += 1;
    }

    0
}

pub fn part_two(input: &str) -> usize {
    let mut counter = 0;
    while counter < input.len() {
        let current = &input[counter..counter + 14];
        if unique_chars(current) {
            return counter + 14;
        }

        counter += 1;
    }

    0
}

fn unique_chars(str: &str) -> bool {
    for c in str.chars() {
        let matches: Vec<_> = str.rmatches(c).collect();
        if matches.len() > 1 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let inputs = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let assert_vals = vec![7, 5, 6, 10, 11];

        for (i, v) in inputs.iter().enumerate() {
            let assert_val = assert_vals[i];
            assert_eq!(part_one(&v), assert_val);
        }
    }

    #[test]
    fn test_part_two() {
        let inputs = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
            "bvwbjplbgvbhsrlpgdmjqwftvncz",
            "nppdvjthqldpwncqszvftbrmjlhg",
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
        ];
        let assert_vals = vec![19, 23, 23, 29, 26];

        for (i, v) in inputs.iter().enumerate() {
            let assert_val = assert_vals[i];
            assert_eq!(part_two(&v), assert_val);
        }
    }
}
