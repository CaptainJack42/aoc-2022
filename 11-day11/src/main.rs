use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref RE_NUM: Regex = Regex::new(r"(?P<num>\d+)").unwrap();
    static ref RE_OP: Regex = Regex::new(r".* new = old (?P<op>[+*]) (?P<arg>.+)").unwrap();
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|m| m.to_string())
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test_case: u128,
    test_true: usize,
    test_false: usize,
    worry_level_divisor: u128,
    num_inspections: u128,
}

impl Monkey {
    fn new(worry_level_divisor: u128) -> Monkey {
        Monkey {
            items: Vec::new(),
            operation: Operation::Add(0),
            test_case: 0,
            test_true: 0,
            test_false: 0,
            worry_level_divisor,
            num_inspections: 0,
        }
    }
    fn add_item(&mut self, item: u128) {
        self.items.push(item);
    }

    fn process_turn(&mut self) -> (u128, usize) {
        let mut item = self.inspect();
        item = item / self.worry_level_divisor;
        self.num_inspections += 1;
        if item % self.test_case == 0 {
            (item, self.test_true)
        } else {
            (item, self.test_false)
        }
    }

    fn inspect(&mut self) -> u128 {
        match self.operation {
            Operation::Add(i) => self.add(i),
            Operation::Multiply(factor) => self.multiply(factor),
            Operation::AddOld => self.add_old(),
            Operation::MultiplyOld => self.multiply_old(),
        }
    }

    fn add(&mut self, i: u128) -> u128 {
        let item = self.items.pop().unwrap();
        item + i
    }

    fn add_old(&mut self) -> u128 {
        let item = self.items.pop().unwrap();
        item + item
    }

    fn multiply(&mut self, i: u128) -> u128 {
        let item = self.items.pop().unwrap();
        item * i
    }

    fn multiply_old(&mut self) -> u128 {
        let item = self.items.pop().unwrap();
        item * item
    }
}

enum Operation {
    Multiply(u128),
    Add(u128),
    MultiplyOld,
    AddOld,
}

fn part1(input: &Vec<String>) -> u128 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monk in input {
        let mut c_monkey: Monkey = Monkey::new(3);
        let mut lines = monk.lines();
        // Skip monkey name
        lines.next();
        for item in RE_NUM.captures_iter(lines.next().unwrap()) {
            c_monkey.add_item(item["num"].parse::<u128>().unwrap());
        }
        // parse operation
        let cap = RE_OP.captures(lines.next().unwrap()).unwrap();
        c_monkey.operation = match &cap["op"] {
            "*" => match &cap["arg"] {
                "old" => Operation::MultiplyOld,
                _ => Operation::Multiply(cap["arg"].parse::<u128>().unwrap()),
            },
            "+" => match &cap["arg"] {
                "old" => Operation::AddOld,
                _ => Operation::Add(cap["arg"].parse::<u128>().unwrap()),
            },
            &_ => panic!("unexpected operation"),
        };
        c_monkey.test_case = RE_NUM.captures(lines.next().unwrap()).unwrap()["num"]
            .parse::<u128>()
            .unwrap();
        c_monkey.test_true = RE_NUM.captures(lines.next().unwrap()).unwrap()["num"]
            .parse::<usize>()
            .unwrap();
        c_monkey.test_false = RE_NUM.captures(lines.next().unwrap()).unwrap()["num"]
            .parse::<usize>()
            .unwrap();
        monkeys.push(c_monkey);
    }

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let (item, target): (u128, usize) = monkeys[i].process_turn();
                monkeys[target].add_item(item);
            }
        }
    }

    let mut inspections: Vec<u128> = Vec::new();
    for mk in monkeys {
        inspections.push(mk.num_inspections);
    }
    inspections.sort_by(|a, b| b.cmp(a));
    return inspections[0] * inspections[1];
}

fn part2(input: &Vec<String>) -> u128 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monk in input {
        let mut c_monkey: Monkey = Monkey::new(1);
        let mut lines = monk.lines();
        // Skip monkey name
        lines.next();
        for item in RE_NUM.captures_iter(lines.next().unwrap()) {
            c_monkey.add_item(item["num"].parse::<u128>().unwrap());
        }
        // parse operation
        let cap = RE_OP.captures(lines.next().unwrap()).unwrap();
        c_monkey.operation = match &cap["op"] {
            "*" => match &cap["arg"] {
                "old" => Operation::MultiplyOld,
                _ => Operation::Multiply(cap["arg"].parse::<u128>().unwrap()),
            },
            "+" => match &cap["arg"] {
                "old" => Operation::AddOld,
                _ => Operation::Add(cap["arg"].parse::<u128>().unwrap()),
            },
            &_ => panic!("unexpected operation"),
        };
        c_monkey.test_case = RE_NUM.captures(lines.next().unwrap()).unwrap()["num"]
            .parse::<u128>()
            .unwrap();
        c_monkey.test_true = RE_NUM.captures(lines.next().unwrap()).unwrap()["num"]
            .parse::<usize>()
            .unwrap();
        c_monkey.test_false = RE_NUM.captures(lines.next().unwrap()).unwrap()["num"]
            .parse::<usize>()
            .unwrap();
        monkeys.push(c_monkey);
    }

    let modulo: u128 = monkeys.iter().map(|x| x.test_case).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let (mut item, target): (u128, usize) = monkeys[i].process_turn();
                item %= modulo;
                monkeys[target].add_item(item);
            }
        }
    }

    let mut inspections: Vec<u128> = Vec::new();
    for mk in monkeys {
        inspections.push(mk.num_inspections);
    }
    inspections.sort_by(|a, b| b.cmp(a));
    return inspections[0] * inspections[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .split("\n\n")
            .map(|m| m.to_string())
            .collect();
        assert_eq!(part1(&input), 10605);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .split("\n\n")
            .map(|m| m.to_string())
            .collect();
        assert_eq!(part2(&input), 2713310158);
    }
}
