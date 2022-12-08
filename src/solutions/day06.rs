// My own scrappy code. Runs in:
//🎄 Part 1 🎄
// 1343 (elapsed: 1.73ms)
// 🎄 Part 2 🎄
// 2193 (elapsed: 3.15ms)
// so not too bad, but quite far from optimal

pub fn part_one_old(input: &str) -> usize {

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

pub fn part_two_old(input: &str) -> usize {
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

pub fn part_one(input: &str) -> usize {
    find_unique_sequence(input.as_bytes(), 4)
}

pub fn part_two(input: &str) -> usize {
    find_unique_sequence(input.as_bytes(), 14)
}

// Completely stolen from a genius from Reddit.
// 🎄 Part 1 🎄
// 1343 (elapsed: 443.17µs)
// 🎄 Part 2 🎄
// 2193 (elapsed: 992.58µs)
fn find_unique_sequence(buffer: &[u8], length: usize) -> usize {
    buffer.windows(length).position(all_bytes_unique).unwrap() + length
}

fn all_bytes_unique(sequence: &[u8]) -> bool {
    sequence
        .iter()
        .enumerate()
        .all(|(i, byte)| !sequence[i + 1..].contains(byte))
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
