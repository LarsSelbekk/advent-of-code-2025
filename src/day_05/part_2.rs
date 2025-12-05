use crate::day_05::part_1;

#[allow(unused)]
fn solve(input: &str) -> usize {
    part_1::parse_ranges(&mut input.lines())
        .iter()
        .map(|range| range.1 - range.0 + 1)
        .sum()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 352716206375547);
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
        14
    )
}
