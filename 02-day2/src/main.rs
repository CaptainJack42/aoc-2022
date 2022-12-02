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

fn part1(input: &Vec<String>) -> isize {
    let mut overall_score: isize = 0;
    for line in input {
        let mut it = line.splitn(2, ' ');
        let opp = it.next().unwrap().parse::<char>().unwrap();
        let me = it.next().unwrap().parse::<char>().unwrap();
        let mut rps = RPS::new(opp, me);
        rps.play();
        overall_score += rps.score;
    }

    return overall_score;
}

struct RPS {
    opp: char,
    me: char,
    score: isize,
}

impl RPS {
    fn new(opp: char, me: char) -> RPS {
        RPS {
            opp: opp,
            me: me,
            score: 0,
        }
    }

    fn play(&mut self) {
        match self.me {
            'X' => {
                self.score += 1;
                match self.opp {
                    'A' => self.score += 3,
                    'B' => self.score += 0,
                    'C' => self.score += 6,
                    _ => panic!("Unexpected choice for me: {}", self.me),
                }
            }
            'Y' => {
                self.score += 2;
                match self.opp {
                    'A' => self.score += 6,
                    'B' => self.score += 3,
                    'C' => self.score += 0,
                    _ => panic!("Unexpected choice for me: {}", self.me),
                }
            }
            'Z' => {
                self.score += 3;
                match self.opp {
                    'A' => self.score += 0,
                    'B' => self.score += 6,
                    'C' => self.score += 3,
                    _ => panic!("Unexpected choice for me: {}", self.me),
                }
            }
            _ => panic!("Unexpected choice for opp: {}", self.opp),
        }
    }
}

fn part2(input: &Vec<String>) -> isize {
    let mut overall_score: isize = 0;
    for line in input {
        let mut it = line.splitn(2, ' ');
        let opp = it.next().unwrap().parse::<char>().unwrap();
        let outc = it.next().unwrap().parse::<char>().unwrap();
        let mut rps = RPS2::new(opp, outc);
        rps.play();
        overall_score += rps.score;
    }

    return overall_score;
}

struct RPS2 {
    opp: char,
    outc: char,
    score: isize,
}

impl RPS2 {
    fn new(opp: char, outc: char) -> RPS2 {
        RPS2 {
            opp: opp,
            outc: outc,
            score: 0,
        }
    }

    fn play(&mut self) {
        match self.outc {
            'X' => match self.opp {
                'A' => self.score += 3,
                'B' => self.score += 1,
                'C' => self.score += 2,
                _ => panic!("Unexpected choice for opp: {}", self.opp),
            },
            'Y' => {
                self.score += 3;
                match self.opp {
                    'A' => self.score += 1,
                    'B' => self.score += 2,
                    'C' => self.score += 3,
                    _ => panic!("Unexpected choice for opp: {}", self.opp),
                }
            }
            'Z' => {
                self.score += 6;
                match self.opp {
                    'A' => self.score += 2,
                    'B' => self.score += 3,
                    'C' => self.score += 1,
                    _ => panic!("Unexpected choice for opp: {}", self.opp),
                }
            }
            _ => panic!("Unexpected choice for outc: {}", self.outc),
        }
    }
}
