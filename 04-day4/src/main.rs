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
        let mut splits = line.split(',');
        let mut sp_f = splits.next().unwrap().split('-');
        let ran_f: Range = Range {
            lo: sp_f.next().unwrap().parse::<usize>().unwrap(),
            hi: sp_f.next().unwrap().parse::<usize>().unwrap(),
        };
        let mut sp_s = splits.next().unwrap().split('-');
        let ran_s: Range = Range {
            lo: sp_s.next().unwrap().parse::<usize>().unwrap(),
            hi: sp_s.next().unwrap().parse::<usize>().unwrap(),
        };
        if (ran_f.lo <= ran_s.lo && ran_f.hi >= ran_s.hi)
            || (ran_s.lo <= ran_f.lo && ran_s.hi >= ran_f.hi)
        {
            ret += 1;
        }
    }
    return ret;
}

fn part2(input: &Vec<String>) -> usize {
    let mut ret: usize = 0;
    for line in input {
        let mut splits = line.split(',');
        let mut sp_f = splits.next().unwrap().split('-');
        let ran_f: Range = Range {
            lo: sp_f.next().unwrap().parse::<usize>().unwrap(),
            hi: sp_f.next().unwrap().parse::<usize>().unwrap(),
        };
        let mut sp_s = splits.next().unwrap().split('-');
        let ran_s: Range = Range {
            lo: sp_s.next().unwrap().parse::<usize>().unwrap(),
            hi: sp_s.next().unwrap().parse::<usize>().unwrap(),
        };
        if (ran_f.hi >= ran_s.lo && ran_f.lo <= ran_s.hi) || (ran_s.hi >= ran_f.lo && ran_s.lo <= ran_f.hi) {
            ret += 1;
        }
    }
    return ret;
}

struct Range {
    lo: usize,
    hi: usize,
}
