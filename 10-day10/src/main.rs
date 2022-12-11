use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: \n{}", part2(&input));
}

fn part1(input: &Vec<String>) -> isize {
    let mut sum: isize = 0;
    let mut cycle: isize = 0;
    let mut reg_val: isize = 1;
    for line in input {
        let mut splits = line.split(' ');
        let instr: &str = splits.next().unwrap();
        match instr {
            "noop" => {
                cycle += 1;
                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    sum += reg_val * cycle;
                }
            }
            "addx" => {
                let val: isize = splits.next().unwrap().parse::<isize>().unwrap();
                cycle += 2;
                if cycle == 20 || cycle == 21 || (cycle - 20) % 40 == 0 || (cycle - 20) % 40 == 1 {
                    sum += reg_val * (cycle / 20) * 20;
                }
                reg_val += val;
            }
            &_ => panic!("unhandled instruction: {}", instr),
        }
    }
    return sum;
}

fn part2(input: &Vec<String>) -> String {
    let mut cycle: isize = 0;
    let mut reg_val: isize = 1;
    let mut output: String = "".to_string();
    for line in input {
        let mut splits = line.split(' ');
        let instr: &str = splits.next().unwrap();
        match instr {
            "noop" => {
                if (reg_val - cycle % 40).abs() <= 1 {
                    output.push('#');
                } else {
                    output.push('.');
                }
                if (cycle + 1) % 40 == 0 {
                    output.push('\n');
                }
                cycle += 1;
            }
            "addx" => {
                let val: isize = splits.next().unwrap().parse::<isize>().unwrap();
                for _ in 0..2 {
                    if (reg_val - cycle % 40).abs() <= 1 {
                        output.push('#');
                    } else {
                        output.push('.');
                    }
                    if (cycle + 1) % 40 == 0 && cycle != 0 {
                        output.push('\n');
                    }
                    cycle += 1;
                }
                reg_val += val;
            }
            &_ => panic!("unhandled instruction: {}", instr),
        }
    }
    output.pop();
    return output;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();

        assert_eq!(part1(&input), 13140);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();
        assert_eq!(
            part2(&input),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string()
        );
    }
}
