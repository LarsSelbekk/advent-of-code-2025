const MAX: i32 = 100;
const INITIAL: i32 = 50;

#[allow(unused)]
fn solve(input: &str) -> usize {
    let mut state = INITIAL;
    let mut count = 0;
    for step in input.lines().map(parse_line) {
        state = (state + step) % MAX;

        if state == 0 {
            count += 1;
        }
    }
    count
}

fn solve_functional(input: &str) -> usize {
    input.lines().map(parse_line).fold((INITIAL, 0), |(state, count), step| {
        let state = (state + step) % MAX;
        (state, if state == 0 { count + 1 } else { count })
    }).1
}

pub(crate) fn parse_line(input: &str) -> i32 {
    (if input.chars().next().unwrap() == 'R' {
        1
    } else {
        -1
    }) * input
        .chars()
        .skip(1)
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve_functional(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve_functional(include_str!("input.txt")), 1141);
}

#[test]
fn test_example() {
    assert_eq!(
        solve_functional(
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
        3
    )
}

#[test]
fn test_mod() {
    assert_eq!((-6 + 10) % 5, 4);
    assert_eq!((-5 + 10) % 5, 0);
    assert_eq!((-2 + 10) % 5, 3);
    assert_eq!((0 + 10) % 5, 0);
    assert_eq!((5 + 10) % 5, 0);
}
