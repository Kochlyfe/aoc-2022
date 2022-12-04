use std::char;

pub trait Priority {
    fn priority(&self) -> u32;
}

impl Priority for char {
    fn priority(&self) -> u32 {
        if self.is_lowercase() {
            return *self as u32 - 96;
        }

        return *self as u32 - 64 + 26;
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| {
            let rucksack_half = rucksack.len() / 2;
            let first = rucksack[0..rucksack_half].to_owned();
            let second = rucksack[rucksack_half..].to_owned();

            for el in first.chars() {
                if second.contains(el) {
                    return get_priority(el);
                }
            }
            // Not used for anything, just fallback
            return get_priority(rucksack.chars().nth(0).unwrap());
        })
        .into_iter()
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| {
            let first_elf = group.get(0).unwrap();

            for el in first_elf.chars() {
                if group.get(1).unwrap().contains(el) && group.get(2).unwrap().contains(el) {
                    return el.priority();
                }
            }
            // not used, just fallback for type safety
            return get_priority(first_elf.chars().nth(0).unwrap());
        })
        .into_iter()
        .sum()
}

// Used ASCII codes to get prio https://www.ascii-code.com/
fn get_priority(element: char) -> u32 {
    if element.is_lowercase() {
        return element as u32 - 96;
    }

    return element as u32 - 64 + 26;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), 157);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), 70);
    }
}
