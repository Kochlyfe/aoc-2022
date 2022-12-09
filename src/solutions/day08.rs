use itertools::Itertools;

pub fn part_one(input: &str) -> u32 {
    let forest = parse_input_to_matrix(input);
    let outer_trees = (forest.get(0).unwrap().len() * 2) + (forest.len() - 2) * 2;

    let mut inner_trees = 0;
    for (i, row) in forest.iter().enumerate() {
        if i == 0 || i == forest.len() - 1 {
            continue;
        }

        for (j, val) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                continue;
            }
            // check across row
            let left_side_max = forest[i][..j].iter().max().unwrap();
            let right_side_max = forest[i][j + 1..].iter().max().unwrap();
            if val > left_side_max || val > right_side_max {
                inner_trees += 1;
                continue;
            }

            // check across column
            let above_max = forest[..i].iter().map(|r| r.get(j).unwrap()).max().unwrap();
            let below_max = forest[i + 1..]
                .iter()
                .map(|r| r.get(j).unwrap())
                .max()
                .unwrap();
            if val > above_max || val > below_max {
                inner_trees += 1;
            }
        }
    }

    outer_trees as u32 + inner_trees
}

pub fn part_two(input: &str) -> u32 {
    let forest = parse_input_to_matrix(input);
    let mut highest_scenic_score = 0;

    for (i, row) in forest.iter().enumerate() {
        if i == 0 || i == forest.len() - 1 {
            continue;
        }
        for (j, val) in row.iter().enumerate() {
            if j == 0 || j == row.len() - 1 {
                continue;
            }

            let left: Vec<&u32> = row[..j].iter().rev().collect_vec();
            let left_side_score = calc_score(&left, &val);

            let right: Vec<&u32> = row[j + 1..].iter().collect_vec();
            let right_side_score = calc_score(&right, &val);

            let up: Vec<&u32> = forest[..i].iter().map(|r| r.get(j).unwrap()).rev().collect_vec();
            let up_score = calc_score(&up, &val);

            let down: Vec<&u32> = forest[i + 1..].iter().map(|r| r.get(j).unwrap()).collect_vec();
            let down_score = calc_score(&down, &val);

            let scenic_score = left_side_score * right_side_score * up_score * down_score;

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    highest_scenic_score
}

fn parse_input_to_matrix(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            return line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
        })
        .collect::<Vec<Vec<u32>>>()
}

fn calc_score(input: &Vec<&u32>, val: &u32) -> u32 {
    let mut curr_count: u32 = 0;

    for tree in input.into_iter() {
        curr_count += 1;
        if tree >= &&val {
            break;
        }
    }

    curr_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 8);
        assert_eq!(part_two(&input), 8);
    }
}
