use crate::day_01::part_1;

const MAX: i32 = 100;
const INITIAL: i32 = 50;

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(part_1::parse_line)
        .fold((INITIAL, 0), |(state, count), step| {
            let after_step = state + step;
            (
                after_step.rem_euclid(MAX),
                count + times_to_pass_mod(state, after_step, MAX) as usize,
            )
        })
        .1
}

fn times_to_pass_mod(from: i32, to: i32, modus: i32) -> i32 {
    // Breakpoints
    // from    0:   -100 -200 -300 ... 100 200 300
    // from 1-99: 0 -100 -200 -300 ... 100 200 300
    (to / modus).abs() + if from > 0 && to <= 0 { 1 } else { 0 }
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 6634);
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
        ),
        6
    )
}

#[test]
fn test_dummy() {
    assert_eq!(solve("R50"), 1);
    assert_eq!(solve("L50"), 1);
    assert_eq!(
        solve(
            "L50
L1"
        ),
        1
    );
    assert_eq!(
        solve(
            "L50
R1"
        ),
        1
    );
    assert_eq!(
        solve(
            "L50
L100"
        ),
        2
    );
    assert_eq!(solve("L50"), 1);
}

#[test]
fn test_pass_mod() {
    assert_eq!(times_to_pass_mod(2, 0, MAX), 1);
    assert_eq!(times_to_pass_mod(2, -MAX, MAX), 2);
    assert_eq!(times_to_pass_mod(0, MAX, MAX), 1);
    assert_eq!(times_to_pass_mod(2, -2, MAX), 1);
    assert_eq!(times_to_pass_mod(0, -399, MAX), 3);
    assert_eq!(times_to_pass_mod(0, -1, MAX), 0);
    assert_eq!(times_to_pass_mod(0, 830, MAX), 8);
    assert_eq!(times_to_pass_mod(99, -1, MAX), 1);
    assert_eq!(times_to_pass_mod(0, -800, MAX), 8);
}
