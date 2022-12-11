use itertools::Itertools;

pub fn part_one(input: &str) -> u32 {
    // you can count up all of the positions the tail visited at least once - set I guess
    // parse board from input - max R/L and max U/D
    // for each line
    // move H ONE AT A TIME
    // for each H move, calc new T position
    // add T position to set
    
    // return length/count of set elements
}

pub fn part_two(input: &str) -> u32 {
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
