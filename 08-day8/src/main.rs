use std::collections::BTreeMap;
use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();
    let map = gen_map(&input);
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn part1(map: &BTreeMap<Coordinate, Tree>) -> usize {
    map.values()
        .filter(|tree| tree.vis_n || tree.vis_e || tree.vis_s || tree.vis_w)
        .count()
}

fn part2(map: &BTreeMap<Coordinate, Tree>) -> usize {
    let max_c: Coordinate = *map.keys().max().unwrap();
    let vis_scores: Vec<usize> = (1..max_c.y)
        .flat_map(|y| (1..max_c.x).map(move |x| Coordinate { x, y }))
        .map(|c| {
            let e = explore_e(map, &c);
            let s = explore_s(map, &c);
            let w = explore_w(map, &c);
            explore_n(map, &c) * e * s * w
        })
        .collect();
    *vis_scores.iter().max().unwrap()
}

fn explore_n(map: &BTreeMap<Coordinate, Tree>, c: &Coordinate) -> usize {
    let mut i: usize = 1;
    let h: u32 = map.get(&c).unwrap().height;
    while let Some(t) = map.get(&Coordinate { x: c.x, y: c.y - i }) {
        if t.height >= h || c.y - i == 0 {
            return i;
        }
        i += 1;
    }
    return i - 1;
}

fn explore_e(map: &BTreeMap<Coordinate, Tree>, c: &Coordinate) -> usize {
    let mut i: usize = 1;
    let h: u32 = map.get(&c).unwrap().height;
    while let Some(t) = map.get(&Coordinate { x: c.x + i, y: c.y }) {
        if t.height >= h {
            return i;
        }
        i += 1;
    }
    return i - 1;
}

fn explore_s(map: &BTreeMap<Coordinate, Tree>, c: &Coordinate) -> usize {
    let mut i: usize = 1;
    let h: u32 = map.get(&c).unwrap().height;
    while let Some(t) = map.get(&Coordinate { x: c.x, y: c.y + i }) {
        if t.height >= h {
            return i;
        }
        i += 1;
    }
    return i - 1;
}

fn explore_w(map: &BTreeMap<Coordinate, Tree>, c: &Coordinate) -> usize {
    let mut i: usize = 1;
    let h: u32 = map.get(&c).unwrap().height;
    while let Some(t) = map.get(&Coordinate { x: c.x - i, y: c.y }) {
        if t.height >= h || c.x - i == 0 {
            return i;
        }
        i += 1;
    }
    return i - 1;
}

fn gen_map(input: &Vec<String>) -> BTreeMap<Coordinate, Tree> {
    let max_c = Coordinate {
        x: input.len() - 1,
        y: input[0].len() - 1,
    };
    let mut map: BTreeMap<Coordinate, Tree> = BTreeMap::new();
    let mut col_max_h: Vec<u32> = vec![0; max_c.x + 1];
    for (y, line) in input.iter().enumerate() {
        let mut line_max_h: u32 = 0;
        for (x, height) in line.chars().enumerate() {
            let h = height.to_digit(10).unwrap();
            let mut c_tree: Tree = Tree {
                height: h,
                vis_n: false,
                vis_e: false,
                vis_s: false,
                vis_w: false,
            };
            // border trees
            if x == 0 || h > line_max_h {
                c_tree.vis_w = true;
            } else if y == 0 || h > col_max_h[x] {
                c_tree.vis_n = true;
            } else if x == max_c.x {
                c_tree.vis_e = true;
            } else if y == max_c.y {
                c_tree.vis_s = true;
            }
            map.insert(Coordinate { x, y }, c_tree);
            line_max_h = if h <= line_max_h { line_max_h } else { h };
            col_max_h[x] = if h <= col_max_h[x] { col_max_h[x] } else { h };
        }
    }
    // at this point all visibilities are checked, except for south and east direction
    col_max_h = vec![0; max_c.x + 1];
    for y in (0..max_c.y + 1).rev() {
        let mut line_max_h = 0;
        for x in (0..max_c.x + 1).rev() {
            let mut c_tree = map.get_mut(&Coordinate { x, y }).unwrap();
            if c_tree.height > col_max_h[x] {
                c_tree.vis_s = true;
            }
            if c_tree.height > line_max_h {
                c_tree.vis_e = true;
            }
            line_max_h = if c_tree.height <= line_max_h {
                line_max_h
            } else {
                c_tree.height
            };
            col_max_h[x] = if c_tree.height <= col_max_h[x] {
                col_max_h[x]
            } else {
                c_tree.height
            };
        }
    }

    return map;
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct Tree {
    height: u32,
    vis_n: bool,
    vis_e: bool,
    vis_s: bool,
    vis_w: bool,
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
        let map = gen_map(&input);

        assert_eq!(part1(&map), 21);
    }

    #[test]
    fn test_part2() {
        let input: Vec<String> = fs::read_to_string("sample")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();
        let map = gen_map(&input);

        assert_eq!(part2(&map), 8);
    }
}
