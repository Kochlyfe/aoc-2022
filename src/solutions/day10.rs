pub fn part_one(input: &str) -> i32 {
    let mut lines = input.lines();

    let mut cpu_cycle = 0;
    let reporting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut register = 1;
    let mut signal_strength = 0;
    let mut add: Option<i32> = None;
    loop {
        cpu_cycle += 1;
        if reporting_cycles.contains(&cpu_cycle) {
            signal_strength += register * &cpu_cycle;
            println!("{}, {}", cpu_cycle, signal_strength);
        }

        match add {
            Some(a) => {
                register += a;
                add = None;
                continue;
            }
            None => {}
        }

        let next_instruction = lines.next();
        match next_instruction {
            Some("noop") => {}
            Some(i) => {
                let val = i.split(" ").last().unwrap().parse::<i32>().unwrap();
                add = Some(val);
            }
            None => break,
        }
    }

    signal_strength
}

pub fn part_two(input: &str) -> String {
    let mut lines = input.lines();

    let mut cpu_cycle = 0;
    let mut output = String::new();
    let mut register = 1;
    let mut add: Option<i32> = None;
    let mut offset = 0;
    loop {
        if register + 1 == (cpu_cycle -offset) || register == (cpu_cycle - offset) || register - 1 == (cpu_cycle - offset) {
            output.push_str("#");
        } else {
            output.push_str(".");
        }

        cpu_cycle += 1;

        if cpu_cycle % 40 == 0 {
            output.push_str("\n");
            offset = cpu_cycle;
        }

        match add {
            Some(a) => {
                register += a;
                add = None;
                continue;
            }
            None => {}
        }

        let next_instruction = lines.next();
        match next_instruction {
            Some("noop") => {}
            Some(i) => {
                let val = i.split(" ").last().unwrap().parse::<i32>().unwrap();
                add = Some(val);
            }
            None => break,
        }
    }
    output.pop();

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 10);
        assert_eq!(part_one(&input), 13140);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 10);
        let assertion_string = String::from("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");

        assert_eq!(part_two(&input), assertion_string);
    }
}
