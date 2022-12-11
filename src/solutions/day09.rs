use std::collections::HashSet;

#[derive(Debug)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    fn update_position_from_direction(&mut self, direction: char) -> &mut Self {
        match direction {
            'U' => {
                self.y += 1;
            }
            'D' => {
                self.y -= 1;
            }
            'R' => {
                self.x += 1;
            }
            'L' => {
                self.x -= 1;
            }
            _ => {}
        };

        self
    }

    fn move_tail(&mut self, head: &mut Position) -> &mut Self {
        let vertical_diff = head.y - self.y;
        let horizontal_diff = head.x - self.x;
        // Diagonal
        if vertical_diff.abs() + horizontal_diff.abs() > 2 {
            if horizontal_diff > 0 {
                self.x += 1;
            } else {
                self.x -= 1;
            }
            if vertical_diff > 0 {
                self.y += 1;
            } else {
                self.y -= 1;
            }

            return self;
        }
        // horizontal
        if horizontal_diff > 1 {
            self.x += 1;
        } else if horizontal_diff < -1 {
            self.x -= 1; 
        }
        //
        // vertical
        if vertical_diff > 1 {
            self.y += 1;
        } else if vertical_diff < -1 {
            self.y -= 1;
        }

        self
    }

    fn get_position(&mut self) -> (i16, i16) {
        (self.x, self.y)
    }
}

pub fn part_one(input: &str) -> u32 {
    let mut position_set: HashSet<(i16,i16)> = HashSet::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    for cmd in input.lines() {
        let direction = cmd.chars().nth(0).unwrap();
        let mut amount = cmd.chars().nth(2).unwrap().to_digit(10).unwrap();

        while amount > 0 {
            let updated_x = head.update_position_from_direction(direction);
            let updated_y = tail.move_tail(updated_x);
            position_set.insert(updated_y.get_position());

            println!("x: {:?}, y: {:?}", updated_x, updated_y);
            amount -= 1;
        }
    }

    position_set.len() as u32
}

pub fn part_two(input: &str) -> u32 {
    10
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

    // #[test]
    // fn test_part_two() {
    //     use aoc::read_file;
    //     let input = read_file("examples", 9);
    //     assert_eq!(part_two(&input), 8);
    // }
}
