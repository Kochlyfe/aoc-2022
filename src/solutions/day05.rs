use itertools::Itertools;
use regex::Regex;
use std::borrow::Cow;
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2

use std::collections::{HashMap, hash_map};

pub fn part_one(input: &str) -> String {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let crates_input = split_input.get(0).unwrap();
    let actions_input = split_input.get(1).unwrap();

    let mut stacks = create_stacks_from_input(&crates_input);

    for line in actions_input.lines() {
        let NUM_REGEX: Regex = Regex::new(r"\D").unwrap();
        let cleaned: Cow<'_, str> = NUM_REGEX.replace_all(&line, "");

        for l in cleaned.lines() {
            let m = usize::try_from(l.chars().nth(0).unwrap().to_digit(10).unwrap()).unwrap();
            let f = usize::try_from(l.chars().nth(1).unwrap().to_digit(10).unwrap()).unwrap() - 1;
            let t = usize::try_from(l.chars().nth(2).unwrap().to_digit(10).unwrap()).unwrap() - 1;

            let mut counter = 0;
            while counter < m {
                let item = stacks.get_mut(f).unwrap().pop().unwrap();
                stacks.get_mut(t).unwrap().push(item);
                counter += 1;
            }
        }
    }

    let res = stacks.into_iter().filter(|v| v.len() > 0 ).map(|v| v.last().unwrap().to_owned()).join("");

    res
}

pub fn part_two(input: &str) -> String {
    String::from("hello2")
}

fn create_stacks_from_input(input: &str) -> Vec<Vec<String>> {
    let backwards = input.lines().rev().collect::<Vec<&str>>();
    let first_line_raw = backwards.get(0).unwrap().replace(" ", "");
    let mut stacks: Vec<Vec<String>> = vec![];

    for c in first_line_raw.chars() {
        // let i = c.to_digit(10).unwrap();
        // stacks.push(vec![c.to_string()]);
        stacks.push(vec![]);
    }

    for line in backwards[1..].into_iter() {
        // let double_space_regex = Regex::new(r"\s{4}").unwrap();
        let cleaned = line.replace("    ", "0").replace(" ", "").replace("[", "").replace("]", "");
        
        for (i, c) in cleaned.chars().enumerate() {
            if c.to_string() == "0" {continue;}
            stacks[i].push(c.to_string())
        }
    }

    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), String::from("CMZ"));
    }

    // #[test]
    // fn test_part_two() {
    //     use aoc::read_file;
    //     let input = read_file("examples", 5);
    //     assert_eq!(part_two(&input), String::from("what"));
    // }
}
