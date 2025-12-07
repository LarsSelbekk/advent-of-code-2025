use itertools::Itertools;
use std::str::Lines;

#[allow(unused)]
fn solve(input: &str) -> usize {
    let mut it = input.lines();
    let ranges = parse_ranges(&mut it);
    it.next();

    it.map(|line| line.parse::<usize>().unwrap())
        .filter(|to_check| {
            (ranges
                .binary_search(&(*to_check, usize::MAX))
                .map_or_else(|i| i.checked_sub(1), |i| Some(i))
                .and_then(|i| ranges.get(i))
                .is_some_and(|(lower, upper)| lower <= to_check && to_check <= upper))
        })
        .count()
}

pub fn parse_ranges(it: &mut Lines) -> Vec<(usize, usize)> {
    it.take_while_ref(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .map(|(lower, upper)| {
            (
                lower.parse::<usize>().unwrap(),
                upper.parse::<usize>().unwrap(),
            )
        })
        .sorted()
        .fold(vec![], |mut ranges, range| {
            match ranges.last_mut() {
                None => ranges.push(range),
                Some(last_range) => {
                    if last_range.1 >= range.0 {
                        last_range.1 = last_range.1.max(range.1);
                    } else {
                        ranges.push(range);
                    }
                }
            }
            ranges
        })
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 694);
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
        ),
        3
    )
}
