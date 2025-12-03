use itertools::Itertools;

#[allow(unused)]
fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let batteries = line
                .chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect_vec();
            let (first_digit_pos, &first_digit) = batteries[..batteries.len() - 1]
                .iter()
                .enumerate()
                .max_by_key(|&(i, &x)| (x, usize::MAX - i))
                .unwrap();
            let &second_digit = batteries[first_digit_pos + 1..].iter().max().unwrap();
            batteries[first_digit_pos] * 10 + second_digit
        })
        .sum()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 17435);
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            "987654321111111
811111111111119
234234234234278
818181911112111"
        ),
        357
    )
}
