use std::ops::Div;

use itertools::Itertools;

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i16>,
    operator: String,
    operator_num: i16,
    test: i16,
    to_true: usize,
    to_false: usize,
}

pub fn part_one(input: &str) -> i32 {
    let mut monkeys = parse_monkeys(input);
    
    for _ in 0..20 {
        // for monkey in monkeys.iter_mut() {
        for j in 0..monkeys.len() {
            let monkey = monkeys.get_mut(j).unwrap();
            let mapped_items = monkey.items.iter().map(|item| map_item(*item, &monkey)).collect_vec();
            for (i, it) in mapped_items {
                monkeys.get_mut(i).unwrap().items.push(it);
            }
            monkey.items.clear();
            // for item in &monkey.items {
            //     let copy = item.to_owned();
            //     let worry_level = match monkey.operator.as_ref() {
            //         "**" => copy.pow(2),
            //         "*" => copy * monkey.operator_num,
            //         "+" => copy + monkey.operator_num,
            //         _ => unreachable!()
            //     };
            //     let worry_level_divided = worry_level.div(3);

            //     let (i, it) = match worry_level_divided % monkey.test == 0 {
            //         true => (monkey.to_true, copy),
            //         false => (monkey.to_false, copy)
            //     };

            //     monkeys[i].items.push(it);

            //     monkey.items.clear();
            // }
        }
    }

    // two most active monkeys: Count the total number of times each monkey inspects items over 20 rounds:
    // In this example, the two most active monkeys inspected items 101 and 105 times.
    // The level of monkey business in this situation can be found by multiplying these together: 10605.
    //
    // 20 rounds

    // parse monkeys

    // loop 20 rounds
    // for each round:
    // for item in items:
    // operation
    // divide by 3
    // test worry level
    // throw item

    12
}

fn map_item(item: i16, monkey: &Monkey) -> (usize, i16) {
                let worry_level = match monkey.operator.as_ref() {
                    "**" => item.pow(2),
                    "*" => item * monkey.operator_num,
                    "+" => item + monkey.operator_num,
                    _ => unreachable!()
                };
                let worry_level_divided = worry_level.div(3);

                let (i, it) = match worry_level_divided % monkey.test == 0 {
                    true => (monkey.to_true, item),
                    false => (monkey.to_false, item)
                };

    (i, it)
}

pub fn part_two(input: &str) -> i32 {
    10
}
// Monkey 0:
//   Starting items: 79, 98
//   Operation: new = old * 19
//   Test: divisible by 23
//     If true: throw to monkey 2
//     If false: throw to monkey 3
fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").into_iter().map(|m| {
        let lines = m
            .lines()
            .map(|line| line.split(" ").filter(|x| *x != "").collect_vec())
            .collect_vec();

        let id = lines.get(0).unwrap().get(1).unwrap().replace(":", "").parse::<usize>().unwrap();
        let items = lines.get(1).unwrap()[2..].iter().map(|n| n.replace(",", "").parse::<i16>().unwrap()).collect_vec();
        let mut operator = lines.get(2).unwrap().get(4).unwrap().to_string();
        let operator_raw = lines.get(2).unwrap().last().unwrap();
        let operator_num = match *operator_raw {
            "old" => {
                operator.push_str("*");
                2
            },
            i => i.parse::<i16>().unwrap()
        };

        let test = lines.get(3).unwrap().last().unwrap().parse::<i16>().unwrap();
        let to_true = lines.get(4).unwrap().last().unwrap().parse::<usize>().unwrap();
        let to_false = lines.get(5).unwrap().last().unwrap().parse::<usize>().unwrap();

        Monkey { id, items, operator, operator_num, test, to_true, to_false }
    }).collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_one(&input), 13140);
    }

    // #[test]
    // fn test_part_two() {
    //     use aoc::read_file;
    //     let input = read_file("examples", 11);
    //     assert_eq!(part_two(&input), 13140);
    // }
}
