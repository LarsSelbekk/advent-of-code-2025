use crate::day_04::part_1;
use crate::day_04::part_1::get_removable;
use itertools::Itertools;

#[allow(unused)]
fn solve(input: &str) -> usize {
    let mut board = part_1::parse_board(input);
    std::iter::from_fn(|| {
        let mut count = 0;
        for (row, column) in get_removable(&board).collect_vec() {
            board[row][column] = '.';
            count += 1;
        }
        if count == 0 { None } else { Some(count) }
    })
    .fold(0, |total_removed, removed| total_removed + removed)
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 9173);
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
        ),
        43
    )
}
