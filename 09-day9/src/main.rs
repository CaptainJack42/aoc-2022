use std::collections::HashSet;
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
    let mut head: Coordinate = Coordinate { x: 0, y: 0 };
    let mut tail: Coordinate = Coordinate { x: 0, y: 0 };
    let mut visited: HashSet<Coordinate> = HashSet::from([tail]);
    for line in input {
        let (direction, len): (char, isize) = get_instruction(line);
        for _ in 0..len {
            match direction {
                'U' => {
                    head.y += 1;
                    if head.y - tail.y > 1 {
                        tail.y += 1;
                        tail.x = head.x;
                    }
                }
                'D' => {
                    head.y -= 1;
                    if tail.y - head.y > 1 {
                        tail.y -= 1;
                        tail.x = head.x;
                    }
                }
                'R' => {
                    head.x += 1;
                    if head.x - tail.x > 1 {
                        tail.x += 1;
                        tail.y = head.y;
                    }
                }
                'L' => {
                    head.x -= 1;
                    if tail.x - head.x > 1 {
                        tail.x -= 1;
                        tail.y = head.y;
                    }
                }
                _ => panic!("Unhandled direction: {}", direction),
            }
            visited.insert(tail);
        }
    }
    return visited.len();
}

fn part2(input: &Vec<String>) -> usize {
    let mut rope: Vec<Coordinate> = vec![Coordinate { x: 0, y: 0 }; 10];
    let mut visited: HashSet<Coordinate> = HashSet::from([rope[9]]);

    for line in input {
        let (direction, len): (char, isize) = get_instruction(line);
        for _ in 0..len {
            match direction {
                'U' => rope[0].y += 1,
                'D' => rope[0].y -= 1,
                'R' => rope[0].x += 1,
                'L' => rope[0].x -= 1,
                _ => panic!("Unhandled direction: {}", direction),
            }
            for i in 1..rope.len() {
                // Check if we need to move at all
                let delta_x = rope[i - 1].x - rope[i].x;
                let delta_y = rope[i - 1].y - rope[i].y;
                if delta_x.abs() > 1 || delta_y.abs() > 1 {
                    // move diagonally
                    if delta_x != 0 && delta_y != 0 {
                        rope[i].x += delta_x.signum();
                        rope[i].y += delta_y.signum();
                    //move straight
                    } else {
                        rope[i].x += if delta_x.abs() > 1 { delta_x.signum() } else { 0 };
                        rope[i].y += if delta_y.abs() > 1 { delta_y.signum() } else { 0 };
                    }
                } else {
                    // if we reach the first knot that doesn't move, all knots behind it don't move
                    // either.
                    break;
                }
            }
            visited.insert(rope[9]);
        }
    }

    return visited.len();
}

fn get_instruction(ins: &str) -> (char, isize) {
    let mut sp = ins.split_whitespace();
    let c = sp.next().unwrap().parse().unwrap();
    let n = sp.next().unwrap().parse().unwrap();
    return (c, n);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
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
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();
        let input2: Vec<String> = fs::read_to_string("sample2")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();
        // assert_eq!(part2(&input), 1);
        assert_eq!(part2(&input2), 36);
    }
}
