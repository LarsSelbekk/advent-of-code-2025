#[allow(unused)]
fn solve(input: &str) -> usize {
    let board = parse_board(input);
    get_removable(&board).count()
}

pub fn parse_board(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn get_removable(board: &Vec<Vec<char>>) -> impl Iterator<Item = (usize, usize)> {
    board.iter().enumerate().flat_map(move |(row_i, row)| {
        row.iter().enumerate().filter_map(move |(column_i, tile)| {
            if *tile == '@'
                && (-1_isize..=1)
                    .flat_map(|row_offset| {
                        (-1_isize..=1)
                            .filter(move |column_offset| (*column_offset, row_offset) != (0, 0))
                            .flat_map(move |column_offset| {
                                board
                                    .get(row_i.wrapping_add_signed(row_offset))
                                    .and_then(|row| {
                                        row.get(column_i.wrapping_add_signed(column_offset))
                                    })
                                    .filter(|&&tile| tile == '@')
                            })
                    })
                    .count()
                    < 4
            {
                Some((row_i, column_i))
            } else {
                None
            }
        })
    })
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 1626);
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
        13
    )
}
