use itertools::Itertools;

const DESIRED_DIGITS: u32 = 12;

#[allow(unused)]
fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            (1..=DESIRED_DIGITS)
                .fold(
                    State {
                        number_so_far: 0,
                        remaining_digits: &line
                            .chars()
                            .map(|c| c.to_string().parse::<usize>().unwrap())
                            .collect_vec()[..],
                    },
                    |state, digit_number| {
                        let (next_digit_index, &next_digit) = state
                            .remaining_digits
                            .iter()
                            .enumerate()
                            .dropping_back((DESIRED_DIGITS - digit_number) as usize)
                            .max_by_key(|&(i, &x)| (x, usize::MAX - i))
                            .unwrap();

                        State {
                            number_so_far: state.number_so_far
                                + next_digit * 10_usize.pow(DESIRED_DIGITS - digit_number),
                            remaining_digits: &state.remaining_digits[next_digit_index + 1..],
                        }
                    },
                )
                .number_so_far
        })
        .sum()
}

struct State<'a> {
    number_so_far: usize,
    remaining_digits: &'a [usize],
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 172886048065379);
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
        3121910778619
    )
}
