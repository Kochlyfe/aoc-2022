use std::collections::HashSet;

pub fn part_one(input: &str) -> usize {
    calc_moves(input, 2)
}

pub fn part_two(input: &str) -> usize {
    calc_moves(input, 10)
}

fn calc_moves(input: &str, knots: usize) -> usize {
    let mut set = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); knots];

    for cmd in input.lines() {
        let (x, y, n) = match cmd.split_at(2) {
            ("R ", n) => (1, 0, n),
            ("L ", n) => (-1, 0, n),
            ("U ", n) => (0, 1, n),
            ("D ", n) => (0, -1, n),
            _ => unreachable!(),
        };

        for _ in 0..n.parse::<usize>().unwrap() {
            rope[0].0 += x;
            rope[0].1 += y;

            for i in 1..rope.len() {
                let (head, tail) = (rope[i - 1], rope[i]);
                let (dx, dy) = (tail.0 - head.0, tail.1 - head.1);
                let (dxl, dyl) = (dx.clamp(-1, 1), dy.clamp(-1, 1));

                rope[i] = match (dx.abs() > 1, dy.abs() > 1) {
                    (true, true) => (head.0 + dxl, head.1 + dyl),
                    (true, false) => (head.0 + dxl, head.1),
                    (false, true) => (head.0, head.1 + dyl),
                    _ => tail,
                };

                let last = rope.last().unwrap().clone();
                set.insert(last);
            }
        }
    }

    set.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_one(&input), 13);
    }

    #[test]
    fn test_part_one_input() {
        use aoc::read_file;
        let input = read_file("inputs", 9);
        assert_eq!(part_one(&input), 6284);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_two(&input), 1);
    }

    #[test]
    fn test_part_two_extra() {
        let input = String::from("R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20");
        assert_eq!(part_two(&input), 36);
    }
}
