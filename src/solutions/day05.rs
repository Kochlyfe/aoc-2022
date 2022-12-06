use itertools::Itertools;
//     [D]
// [N] [C]
// [Z] [M] [P]
//  1   2   3

// move 1 from 2 to 1
// move 3 from 1 to 3
// move 2 from 2 to 1
// move 1 from 1 to 2

pub fn part_one(input: &str) -> String {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let crates_input = split_input.get(0).unwrap();
    let actions_input = split_input.get(1).unwrap();

    let mut stacks = create_stacks_from_input(&crates_input);

    for line in actions_input.lines() {
        let spl: Vec<&str> = line.split(" ").collect();
        let cleaned: Vec<usize> = spl
            .into_iter()
            .filter(|x| x.chars().all(char::is_numeric))
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let m = *cleaned.get(0).unwrap();
        let f = *cleaned.get(1).unwrap() - 1;
        let t = *cleaned.get(2).unwrap() - 1;

        let mut counter = 0;
        while counter < m {
            let item = stacks.get_mut(f).unwrap().pop().unwrap();
            stacks.get_mut(t).unwrap().push(item);

            counter += 1;
        }
    }

    let res = stacks
        .into_iter()
        .filter(|v| v.len() > 0)
        .map(|v| v.last().unwrap().to_owned())
        .join("");

    res
}

pub fn part_two(input: &str) -> String {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let crates_input = split_input.get(0).unwrap();
    let actions_input = split_input.get(1).unwrap();

    let mut stacks = create_stacks_from_input(&crates_input);

    for line in actions_input.lines() {
        let spl: Vec<&str> = line.split(" ").collect();
        let cleaned: Vec<usize> = spl
            .into_iter()
            .filter(|x| x.chars().all(char::is_numeric))
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let m = *cleaned.get(0).unwrap();
        let f = *cleaned.get(1).unwrap() - 1;
        let t = *cleaned.get(2).unwrap() - 1;

        // println!("{},{},{}", m, f, t);
        // let mut items = stacks.get(f).unwrap().iter().rev().take(m).rev().collect::<Vec<_>>();
        // stacks.get(t).unwrap().push(items);

        // println!("{:?}", rev_items);

        let mut counter = 0;
        let mut items = vec![];
        while counter < m {
            let item = stacks.get_mut(f).unwrap().pop().unwrap();
            items.push(item);

            counter += 1;
        }
        items.reverse();

        for item in items.iter() {
            stacks.get_mut(t).unwrap().push(item.to_owned());
        }
    }

    let res = stacks
        .into_iter()
        .filter(|v| v.len() > 0)
        .map(|v| v.last().unwrap().to_owned())
        .join("");

    res
}

fn create_stacks_from_input(input: &str) -> Vec<Vec<String>> {
    let backwards = input.lines().rev().collect::<Vec<&str>>();
    let first_line_raw = backwards.get(0).unwrap().replace(" ", "");
    let mut stacks: Vec<Vec<String>> = vec![];

    for _ in first_line_raw.chars() {
        stacks.push(vec![]);
    }

    for line in backwards[1..].into_iter() {
        let cleaned = line
            .replace("    ", "0")
            .replace(" ", "")
            .replace("[", "")
            .replace("]", "");

        for (i, c) in cleaned.chars().enumerate() {
            if c.to_string() == "0" {
                continue;
            }
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

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), String::from("MCD"));
    }
}
