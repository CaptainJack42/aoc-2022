use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

lazy_static! {
    static ref REGEX_CMD: Regex = Regex::new(r"\$\s+(?P<cmd>\w+)(:?\s+)?(?P<arg>.*)?").unwrap();
    static ref REGEX_DIR: Regex = Regex::new(r"dir\s(?P<dir>.+)").unwrap();
    static ref REGEX_FILE: Regex =
        Regex::new(r"(?P<size>\d+)\s(?P<name>[^0-9\.])(:?\.+)?(?P<type>\D+)?").unwrap();
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let disk: HashMap<String, HashSet<String>> = get_disk(&input);
    println!("Part 1: {}", part1(&disk));
    println!("Part 2: {}", part2(&disk));
}

fn get_disk(input: &Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut cwd: Vec<String> = Vec::new();
    let mut lines = input.iter();
    let mut curr_line = lines.next();
    let mut disk: HashMap<String, HashSet<String>> = HashMap::new();

    loop {
        if curr_line == None {
            break;
        }
        if let Some(capture) = REGEX_CMD.captures(curr_line.unwrap()) {
            let cmd = &capture["cmd"];
            match cmd {
                "cd" => {
                    let arg = &capture["arg"];
                    if arg == ".." {
                        cwd.pop();
                    } else if arg == "/" {
                        cwd = Vec::from(["".to_string()]);
                        disk.entry(cwd.join("/") + "/").or_insert(HashSet::new());
                    } else {
                        let dir = disk.get_mut(&(cwd.join("/") + "/")).unwrap();
                        dir.insert("dir ".to_owned() + &cwd.join("/") + "/" + arg + "/");
                        cwd.push(arg.to_string());
                        disk.entry(cwd.join("/") + "/").or_insert(HashSet::new());
                    }
                    curr_line = lines.next();
                }
                "ls" => {
                    let dir = disk.get_mut(&(cwd.join("/") + "/")).unwrap();
                    loop {
                        curr_line = lines.next();
                        if curr_line.is_none() {
                            break;
                        }
                        if curr_line.unwrap().starts_with('$') {
                            break;
                        }
                        if let Some(directory) = REGEX_DIR.captures(curr_line.unwrap()) {
                            dir.insert(
                                "dir ".to_owned() + &cwd.join("/") + "/" + &directory["dir"] + "/",
                            );
                        } else {
                            dir.insert(curr_line.unwrap().to_string());
                        }
                    }
                }
                _ => panic!("Unexpected command: {}", cmd),
            }
        }
    }
    return disk;
}

fn calc_dir_size(
    dir_sizes: &HashMap<String, usize>,
    disk: &HashMap<String, HashSet<String>>,
    curr_dir: String,
) -> usize {
    let mut size = 0;
    let dir = &disk[&curr_dir];
    for cont in dir {
        if let Some(file) = REGEX_FILE.captures(cont) {
            size += file["size"].parse::<usize>().unwrap();
        } else if let Some(directory) = REGEX_DIR.captures(cont) {
            if let Some(dir_size) = dir_sizes.get(&directory["dir"].to_string()) {
                size += dir_size;
            } else {
                size += calc_dir_size(dir_sizes, disk, directory["dir"].to_string());
            }
        }
    }
    return size;
}

fn part1(disk: &HashMap<String, HashSet<String>>) -> usize {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    let mut ret = 0;
    for cd in disk.keys() {
        let size = calc_dir_size(&dir_sizes, &disk, cd.to_string());
        dir_sizes.insert(cd.to_string(), size);
        if size <= 100_000 {
            ret += size;
        }
    }
    return ret;
}

fn part2(disk: &HashMap<String, HashSet<String>>) -> usize {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    for cd in disk.keys() {
        let size = calc_dir_size(&dir_sizes, &disk, cd.to_string());
        dir_sizes.insert(cd.to_string(), size);
    }
    let mut dir_choices: Vec<usize> = Vec::new();
    let total_used = dir_sizes["/"];
    let total_space: usize = 70_000_000;
    let needed_space: usize = 30_000_000;
    for (_name, size) in dir_sizes {
        if total_space - total_used + size > needed_space {
            dir_choices.push(size);
        }
    }
    return *dir_choices.iter().min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();
        let disk: HashMap<String, HashSet<String>> = get_disk(&input);
        assert_eq!(part1(&disk), 95437);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();
        let disk: HashMap<String, HashSet<String>> = get_disk(&input);
        assert_eq!(part2(&disk), 24933642);
    }
}
