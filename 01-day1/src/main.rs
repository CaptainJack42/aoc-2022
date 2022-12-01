use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|l| l.to_string())
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<String>) -> usize {
    let mut cals: Vec<usize> = Vec::new();
    for elf in input {
        cals.push(elf.lines().map(|x| x.parse::<usize>().unwrap()).sum());
    }
    return *cals.iter().max().unwrap();
}

fn part2(input: &Vec<String>) -> usize {
    let mut cals: Vec<usize> = Vec::new();
    for elf in input {
        cals.push(elf.lines().map(|x| x.parse::<usize>().unwrap()).sum());
    }
    cals.sort_unstable_by(|a, b| b.cmp(a));
    return cals[0] + cals[1] + cals[2];
}
