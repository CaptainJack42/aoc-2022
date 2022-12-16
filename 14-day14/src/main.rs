use std::collections::HashMap;
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

#[derive(Eq, PartialEq)]
enum Content {
    Rock,
    Sand,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn delta(self, c2: Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - c2.x,
            y: self.y - c2.y,
        }
    }
}

fn parse_input(input: &Vec<String>) -> (HashMap<Coordinate, Content>, isize) {
    let mut map: HashMap<Coordinate, Content> = HashMap::new();
    let (mut map_min, mut map_max): (Coordinate, Coordinate) = (
        Coordinate {
            x: isize::MAX,
            y: 0,
        },
        Coordinate {
            x: isize::MIN,
            y: isize::MIN,
        },
    );
    for line in input {
        let mut prev_coord: Option<Coordinate> = None;
        for cs in line.split(" -> ") {
            let mut splits = cs.split(',');
            let coord: Coordinate = Coordinate {
                x: splits.next().unwrap().parse::<isize>().unwrap(),
                y: splits.next().unwrap().parse::<isize>().unwrap(),
            };
            if let Some(pc) = prev_coord {
                match pc.delta(coord) {
                    Coordinate { x: 0, y: dy } => {
                        for i in 0..dy.abs() + 1 {
                            map.insert(
                                Coordinate {
                                    x: coord.x,
                                    y: coord.y + i * dy.signum(),
                                },
                                Content::Rock,
                            );
                        }
                    }
                    Coordinate { x: dx, y: 0 } => {
                        for i in 0..dx.abs() + 1 {
                            map.insert(
                                Coordinate {
                                    x: coord.x + i * dx.signum(),
                                    y: coord.y,
                                },
                                Content::Rock,
                            );
                        }
                    }
                    Coordinate { .. } => panic!("Unexpected delta"),
                }
            }
            map_min.x = coord.x.min(map_min.x);
            map_min.y = coord.y.min(map_min.y);
            map_max.x = coord.x.max(map_max.x);
            map_max.y = coord.y.max(map_max.y);
            prev_coord = Some(coord);
        }
    }
    // for y in map_min.y - 1..map_max.y + 2 {
    //     for x in map_min.x - 1..map_max.x + 2 {
    //         map.entry(Coordinate { x, y }).or_insert(Content::Air);
    //     }
    // }
    (map, map_max.y)
}

fn part1(input: &Vec<String>) -> isize {
    let mut c_grains: isize = 0;
    let origin: Coordinate = Coordinate { x: 500, y: 0 };
    let (mut map, floor_level) = parse_input(input);
    let mut curr_pos = origin;
    while let Some(dir) = find_fall_direction(curr_pos, &map, floor_level) {
        match dir {
            Direction::Down => {
                curr_pos.y += 1;
            }
            Direction::DownLeft => {
                curr_pos.x -= 1;
                curr_pos.y += 1;
            }
            Direction::DownRight => {
                curr_pos.x += 1;
                curr_pos.y += 1;
            }
            Direction::Stop => {
                map.entry(curr_pos).or_insert(Content::Sand);
                curr_pos = origin;
                c_grains += 1;
            }
            Direction::Outside => {
                break;
            }
        }
    }
    c_grains
}

enum Direction {
    Down,
    DownRight,
    DownLeft,
    Stop,
    Outside,
}

fn find_fall_direction(
    curr_pos: Coordinate,
    map: &HashMap<Coordinate, Content>,
    floor_level: isize,
) -> Option<Direction> {
    if curr_pos.y > floor_level {
        return Some(Direction::Outside);
    }
    if map.get(&Coordinate {
        x: curr_pos.x,
        y: curr_pos.y + 1,
    }) == None
    {
        return Some(Direction::Down);
    } else if map.get(&Coordinate {
        x: curr_pos.x - 1,
        y: curr_pos.y + 1,
    }) == None
    {
        return Some(Direction::DownLeft);
    } else if map.get(&Coordinate {
        x: curr_pos.x + 1,
        y: curr_pos.y + 1,
    }) == None
    {
        return Some(Direction::DownRight);
    } else if map.contains_key(&Coordinate {
        x: curr_pos.x,
        y: curr_pos.y + 1,
    }) {
        return Some(Direction::Stop);
    } else if curr_pos.y + 1 == floor_level {
        return Some(Direction::Stop);
    }
    None
}

fn part2(input: &Vec<String>) -> isize {
    let mut c_grains: isize = 0;
    let origin: Coordinate = Coordinate { x: 500, y: 0 };
    let (mut map, mut floor_level) = parse_input(input);
    floor_level += 2;
    let mut curr_pos = origin;
    while let Some(dir) = find_fall_direction2(curr_pos, &map, floor_level) {
        match dir {
            Direction::Down => {
                curr_pos.y += 1;
            }
            Direction::DownLeft => {
                curr_pos.x -= 1;
                curr_pos.y += 1;
            }
            Direction::DownRight => {
                curr_pos.x += 1;
                curr_pos.y += 1;
            }
            Direction::Stop => {
                if curr_pos == origin {
                    c_grains += 1;
                    break;
                }
                map.entry(curr_pos).or_insert(Content::Sand);
                curr_pos = origin;
                c_grains += 1;
            }
            Direction::Outside => {
                break;
            }
        }
    }
    c_grains
}

fn find_fall_direction2(
    curr_pos: Coordinate,
    map: &HashMap<Coordinate, Content>,
    floor_level: isize,
) -> Option<Direction> {
    if curr_pos.y + 1 == floor_level {
        return Some(Direction::Stop);
    }
    if map.get(&Coordinate {
        x: curr_pos.x,
        y: curr_pos.y + 1,
    }) == None
    {
        return Some(Direction::Down);
    } else if map.get(&Coordinate {
        x: curr_pos.x - 1,
        y: curr_pos.y + 1,
    }) == None
    {
        return Some(Direction::DownLeft);
    } else if map.get(&Coordinate {
        x: curr_pos.x + 1,
        y: curr_pos.y + 1,
    }) == None
    {
        return Some(Direction::DownRight);
    } else if map.contains_key(&Coordinate {
        x: curr_pos.x,
        y: curr_pos.y + 1,
    }) {
        return Some(Direction::Stop);
    } else if curr_pos.y + 1 == floor_level {
        return Some(Direction::Stop);
    }
    None
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
        assert_eq!(part1(&input), 24);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();
        assert_eq!(part2(&input), 93);
    }
}
