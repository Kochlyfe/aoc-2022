use std::{collections::HashMap, ops::Div};

use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    operator: String,
    operator_num: i64,
    test: i64,
    to_true: usize,
    to_false: usize,
}

pub fn part_one(input: &str) -> i64 {
    monkey_game(input, 20, Some(3))
}

pub fn part_two(input: &str) -> i64 {
    monkey_game(input, 10000, None)
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .into_iter()
        .map(|m| {
            let lines = m
                .lines()
                .map(|line| line.split(" ").filter(|x| *x != "").collect_vec())
                .collect_vec();

            let id = lines
                .get(0)
                .unwrap()
                .get(1)
                .unwrap()
                .replace(":", "")
                .parse::<usize>()
                .unwrap();
            let items = lines.get(1).unwrap()[2..]
                .iter()
                .map(|n| n.replace(",", "").parse::<i64>().unwrap())
                .collect_vec();
            let mut operator = lines.get(2).unwrap().get(4).unwrap().to_string();
            let operator_raw = lines.get(2).unwrap().last().unwrap();
            let operator_num = match *operator_raw {
                "old" => {
                    operator.push_str("*");
                    2
                }
                i => i.parse::<i64>().unwrap(),
            };

            let test = lines
                .get(3)
                .unwrap()
                .last()
                .unwrap()
                .parse::<i64>()
                .unwrap();
            let to_true = lines
                .get(4)
                .unwrap()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let to_false = lines
                .get(5)
                .unwrap()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();

            Monkey {
                id,
                items,
                operator,
                operator_num,
                test,
                to_true,
                to_false,
            }
        })
        .collect_vec()
}

fn monkey_game(input: &str, rounds: i16, worry_level: Option<i64>) -> i64 {
    let mut monkeys = parse_monkeys(input);

    let lowest_common_denominator = monkeys.iter().map(|m| m.test).product::<i64>();

    let mut item_map: HashMap<usize, Vec<i64>> = HashMap::new();
    monkeys.iter().for_each(|m| {
        item_map.insert(m.id, m.items.to_owned());
    });

    let mut count: Vec<i64> = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for monkey in monkeys.iter_mut() {
            let mapped_items = item_map
                .get(&monkey.id)
                .unwrap()
                .iter()
                .map(|item| map_item(*item, &monkey, worry_level, lowest_common_denominator))
                .collect_vec();

            let len = mapped_items.len() as i64;
            for (i, it) in mapped_items {
                item_map.get_mut(&i).unwrap().push(it);
            }
            count[monkey.id] += len;
            item_map.get_mut(&monkey.id).unwrap().clear();
        }
    }

    count.sort();

    count[count.len() - 2] * count[count.len() - 1]
} 

fn map_item(item: i64, monkey: &Monkey, worry: Option<i64>, lcd: i64) -> (usize, i64) {
    let worry_level = match monkey.operator.as_ref() {
        "**" => item.pow(2),
        "*" => item * monkey.operator_num,
        "+" => item + monkey.operator_num,
        _ => unreachable!(),
    };
    let worry_level_divided = match worry {
        Some(w) => worry_level.div(w),
        None => worry_level % lcd
    };
    
    let (i, it) = match worry_level_divided % monkey.test == 0 {
        true => (monkey.to_true, worry_level_divided),
        false => (monkey.to_false, worry_level_divided),
    };

    (i, it)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_one(&input), 10605);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_two(&input), 2713310158);
    }
}
