use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let (map, start, goal): (HashMap<Coordinate, isize>, Coordinate, Coordinate) = gen_map(&input);
    println!("Part 1: {}", part1(&map, &start, &goal));
    println!("Part 2: {}", part2(&map, &goal));
}

fn gen_map(input: &Vec<String>) -> (HashMap<Coordinate, isize>, Coordinate, Coordinate) {
    let mut start: Coordinate = Coordinate { x: 0, y: 0 };
    let mut goal: Coordinate = Coordinate { x: 0, y: 0 };
    let mut map: HashMap<Coordinate, isize> = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start.x = x as isize;
                start.y = y as isize;
                map.insert(
                    Coordinate {
                        x: x as isize,
                        y: y as isize,
                    },
                    'a' as isize - 97,
                );
                continue;
            } else if ch == 'E' {
                goal.x = x as isize;
                goal.y = y as isize;
                map.insert(
                    Coordinate {
                        x: x as isize,
                        y: y as isize,
                    },
                    'z' as isize - 97,
                );
                continue;
            }
            map.insert(
                Coordinate {
                    x: x as isize,
                    y: y as isize,
                },
                ch as isize - 97,
            );
        }
    }
    return (map, start, goal);
}

fn part1(map: &HashMap<Coordinate, isize>, start: &Coordinate, goal: &Coordinate) -> usize {
    let mut q: VecDeque<(Coordinate, usize)> = VecDeque::from([(*start, 0)]);
    let mut explored: HashSet<Coordinate> = HashSet::from([*start]);
    while let Some((c, cost)) = q.pop_front() {
        if c == *goal {
            return cost;
        }
        let neighboors: Vec<Coordinate> = vec![
            Coordinate { x: c.x - 1, y: c.y },
            Coordinate { x: c.x + 1, y: c.y },
            Coordinate { x: c.x, y: c.y - 1 },
            Coordinate { x: c.x, y: c.y + 1 },
        ];
        for n in neighboors {
            if !explored.contains(&n)
                && map.contains_key(&n)
                && map.get(&n).unwrap() - map.get(&c).unwrap() <= 1
            {
                q.push_back((n, cost + 1));
                explored.insert(n);
            }
        }
    }
    return 0;
}

fn part2(map: &HashMap<Coordinate, isize>, goal: &Coordinate) -> usize {
    let mut path_costs: Vec<usize> = Vec::new();
    for (s, h) in map.iter() {
        if *h != 'a' as isize - 97 {
            continue;
        }
        let mut q: VecDeque<(Coordinate, usize)> = VecDeque::from([(*s, 0)]);
        let mut explored: HashSet<Coordinate> = HashSet::from([*s]);
        while let Some((c, cost)) = q.pop_front() {
            if c == *goal {
                path_costs.push(cost);
                break;
            }
            let neighboors: Vec<Coordinate> = vec![
                Coordinate { x: c.x - 1, y: c.y },
                Coordinate { x: c.x + 1, y: c.y },
                Coordinate { x: c.x, y: c.y - 1 },
                Coordinate { x: c.x, y: c.y + 1 },
            ];
            for n in neighboors {
                if !explored.contains(&n)
                    && map.contains_key(&n)
                    && map.get(&n).unwrap() - map.get(&c).unwrap() <= 1
                {
                    q.push_back((n, cost + 1));
                    explored.insert(n);
                }
            }
        }
    }
    return *path_costs.iter().min().unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
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
        let (map, start, goal): (HashMap<Coordinate, isize>, Coordinate, Coordinate) =
            gen_map(&input);
        assert_eq!(part1(&map, &start, &goal), 31);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();
        let (map, start, goal): (HashMap<Coordinate, isize>, Coordinate, Coordinate) =
            gen_map(&input);
        assert_eq!(part2(&map, &goal), 29);
    }
}
