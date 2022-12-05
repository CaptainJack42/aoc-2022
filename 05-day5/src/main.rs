use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let (stacks, ops) = parse_input(&input);
    println!("Part 1: {}", part1(stacks.clone(), ops.clone()));
    println!("Part 2: {}", part2(stacks.clone(), ops.clone()));
}

fn parse_input(input: &String) -> (HashMap<usize, VecDeque<char>>, Vec<Operation>) {
    let mut parts = input.split("\n\n");
    let stacks_str = parts.next().unwrap();
    let mut stacks_list: Vec<&str> = stacks_str.lines().collect();
    stacks_list.pop(); // remove the last line
    let mut stacks: HashMap<usize, VecDeque<char>> = HashMap::new();
    for line in stacks_list {
        for c_pos in (1..line.len()).step_by(4) {
            let ch = line.as_bytes()[c_pos] as char;
            if ch != ' ' {
                stacks
                    .entry(c_pos / 4 + 1)
                    .and_modify(|s| s.push_back(ch))
                    .or_insert(VecDeque::from([ch]));
            }
        }
    }

    let ops_str = parts.next().unwrap();
    let mut ops: Vec<Operation> = Vec::new();
    let re_op = Regex::new(r"move (?P<cnt>\d+) from (?P<fro>\d+) to (?P<to>\d+)").unwrap();
    for line in ops_str.lines() {
        let matches = re_op.captures(line).unwrap();
        ops.push(Operation {
            cnt: matches
                .name("cnt")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            fro: matches
                .name("fro")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
            to: matches
                .name("to")
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap(),
        });
    }

    return (stacks, ops);
}

#[derive(Debug, Clone)]
struct Operation {
    cnt: usize,
    fro: usize,
    to: usize,
}

fn part1(mut stacks: HashMap<usize, VecDeque<char>>, ops: Vec<Operation>) -> String {
    for op in ops {
        for _ in 0..op.cnt {
            let mut elem: char = ' ';
            stacks
                .entry(op.fro)
                .and_modify(|s| elem = s.pop_front().unwrap());
            stacks.entry(op.to).and_modify(|s| s.push_front(elem));
        }
    }

    let mut ret: String = String::from("");
    for key in 1..stacks.len() + 1 {
        ret.push(stacks.get(&key).unwrap()[0]);
    }
    return ret;
}

fn part2(mut stacks: HashMap<usize, VecDeque<char>>, ops: Vec<Operation>) -> String {
    for op in ops {
        let mut mov_stack: VecDeque<char> = VecDeque::new();
        for _ in 0..op.cnt {
            stacks
                .entry(op.fro)
                .and_modify(|s| mov_stack.push_back(s.pop_front().unwrap()));
        }
        let mut curr_elem = mov_stack.pop_back();
        while curr_elem != None {
            stacks
                .entry(op.to)
                .and_modify(|s| s.push_front(curr_elem.unwrap()));
            curr_elem = mov_stack.pop_back();
        }
    }

    let mut ret: String = String::from("");
    for key in 1..stacks.len() + 1 {
        ret.push(stacks.get(&key).unwrap()[0]);
    }
    return ret;
}
