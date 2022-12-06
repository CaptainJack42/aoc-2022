use std::collections::{HashSet, VecDeque};
use std::fs;
use std::hash::Hash;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    println!("Part 1: {}", solution(&input, 4));
    println!("Part 2: {}", solution(&input, 14));
}

fn solution(input: &String, num_unique: usize) -> usize {
    let mut buf: VecDeque<char> = VecDeque::new();
    for (idx, ch) in input.chars().enumerate() {
        buf.push_back(ch);
        if buf.len() < num_unique {
            continue;
        } else if buf.len() > num_unique {
            buf.pop_front();
        }
        if has_unique_elements(buf.clone()) {
            return idx + 1;
        }
    }
    return 0;
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut unique = HashSet::new();
    iter.into_iter().all(move |x| unique.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string1() {
        assert_eq!(
            solution(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4),
            7
        );
        assert_eq!(
            solution(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14),
            19
        );
    }
    #[test]
    fn string2() {
        assert_eq!(solution(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4), 5);
        assert_eq!(
            solution(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),
            23
        );
    }
    #[test]
    fn string3() {
        assert_eq!(solution(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4), 6);
        assert_eq!(
            solution(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14),
            23
        );
    }
    #[test]
    fn string4() {
        assert_eq!(
            solution(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4),
            10
        );
        assert_eq!(
            solution(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14),
            29
        );
    }
    #[test]
    fn string5() {
        assert_eq!(
            solution(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4),
            11
        );
        assert_eq!(
            solution(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14),
            26
        );
    }
}
