use std::collections::HashMap;

#[derive(Debug)]
struct Directory {
    size: u32,
    subfolders: Vec<String>,
}

pub fn part_one(input: &str) -> u32 {
    let directories = parse_file_structure(input);

    traverse(&directories)
}

pub fn part_two(input: &str) -> u32 {
    1
}

fn traverse(structure: &HashMap<&str, Directory>) -> u32 {
    let mut dirs_to_visit: Vec<&str> = vec![];

    for k in structure.keys() {
        dirs_to_visit.push(*k);
        let dir = structure.get(k).unwrap();
        println!("{}: {:?}", k, dir);
    }

    let threshold: u32 = 100000;

    let summed: Vec<u32> = dirs_to_visit
        .into_iter()
        // .map(|x| structure.get(x).unwrap().direct_size)
        .map(|x| recurse(x, structure, &mut vec![]))
        .collect();

    let filtered: u32 = summed.into_iter().filter(|x| *x <= threshold).sum();

    filtered
}

fn recurse(key: &str, structure: &HashMap<&str, Directory>, visited: &mut Vec<String>) -> u32 {
    let lookup = structure.get(key).expect(key);
    if visited.contains(&key.to_string()) {
        println!("{:?}", visited);
        visited.clear();
        return 0;
    }
    visited.push(key.to_string());

    if lookup.subfolders.len() > 0 {
        let copy = lookup.subfolders.clone();
        let indirect_size: u32 = copy
            .into_iter()
            .map(|s| recurse(&s, structure, visited))
            .sum();

        return lookup.size + indirect_size;
    }

    return lookup.size;
}

fn parse_file_structure(input: &str) -> HashMap<&str, Directory> {
    // $: command
    // dir: directory
    // <num> size
    //
    // commands:
    //  cd: change directory
    //  ls: list content of current directory -> all lines untill next $ will be in that dir
    //
    //  directories:
    //  root: /
    //  .. -> move back
    //  <char> move into that dir
    //
    //  need to determine total size of each directory -> including subdirectories ofc
    //  Find all directories with a total size of at most 100_000 and calculate their total size
    let mut path: Vec<&str> = vec![];
    let mut ls_mode: bool = false;
    let mut files_in_current_dir: Vec<&str> = vec![];
    let mut directories: HashMap<&str, Directory> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        if line.contains("$ ls") {
            ls_mode = true;
            continue;
        }

        if i == input.lines().count() - 1 && ls_mode {
            if !line.contains("dir") && !line.contains("$") {
                files_in_current_dir.push(line);
            }
            let curr_dir = *path.last().unwrap();
            match directories.get(curr_dir) {
                Some(_) => {}
                None => {
                    let (size, subfolders) = parse_directory_contents(&files_in_current_dir);

                    directories.insert(
                        curr_dir,
                        Directory {
                            size,
                            subfolders,
                        },
                    );
                    files_in_current_dir.clear();
                }
            }
            continue;
        }

        if line.contains("$ cd") {
            if ls_mode {
                // Add parent folders name to the subfolder name
                let curr_dir = *path.last().unwrap();

                match directories.get(curr_dir) {
                    Some(_) => {}
                    None => {
                        let (size, subfolders) = parse_directory_contents(&files_in_current_dir);

                        directories.insert(
                            curr_dir,
                            Directory {
                                size,
                                subfolders,
                            },
                        );
                        files_in_current_dir.clear();
                    }
                }
                ls_mode = false;
            }

            let i: Vec<&str> = line.split(" ").collect();
            match *i.get(2).unwrap() {
                ".." => {
                    path.pop();
                }
                x => {
                    path.push(x);
                }
            }

            continue;
        }

        files_in_current_dir.push(line);
    }

    directories
}

fn parse_directory_contents(files: &Vec<&str>) -> (u32, Vec<String>) {
    let mut direct_size: u32 = 0;
    let mut subfolders: Vec<String> = vec![];

    for f in files {
        if f.contains("dir") {
            let spl: Vec<&str> = f.split(" ").collect();
            let dir_name = spl.last().unwrap();
            // Add current_dir name to dir_name string
            subfolders.push(dir_name.to_string());
        } else {
            let index = f.find(" ").unwrap();
            let int = &f[..index].parse::<u32>().unwrap();
            direct_size += int;
        }
    }

    (direct_size, subfolders)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), 95437);
    }

    // #[test]
    // fn test_part_two() {
    //     use aoc::read_file;
    //     let input = read_file("examples", 7);
    //     assert_eq!(part_two(&input), String::from("MCD"));
    // }
}
