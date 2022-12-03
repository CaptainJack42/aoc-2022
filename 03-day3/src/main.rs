use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Vec<String>) -> usize {
    let mut ret: usize = 0;
    for line in input {
        let (first_comp, second_comp): (&str, &str) = line.split_at(line.len() / 2);
        for char in first_comp.chars() {
            if second_comp.chars().any(|c| c == char) {
                ret += ((char as u8 - 38) % 58) as usize;
                break;
            }
        }
    }
    return ret;
}

fn part2(input: &Vec<String>) -> usize {
    let mut ret: usize = 0;
    for group in input.chunks(3) {
        for char in group[0].chars() {
            if group[1].chars().any(|c| c == char) && group[2].chars().any(|c| c == char) {
                ret += ((char as u8 - 38) % 58) as usize;
                break;
            }
        }
    }
    return ret;
}
