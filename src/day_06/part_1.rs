use itertools::Itertools;

#[allow(unused)]
fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let columns = lines
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|operator| {
            if operator == "+" {
                Column {
                    partial_result: 0,
                    operator: Box::new(std::ops::Add::add),
                }
            } else {
                Column {
                    partial_result: 1,
                    operator: Box::new(std::ops::Mul::mul),
                }
            }
        })
        .collect_vec();
    lines
        .map(|line| {
            line.split_whitespace()
                .map(|operand| operand.parse().unwrap())
        })
        .fold(columns, |mut columns, operands| {
            for (column, next_operand) in columns.iter_mut().zip(operands) {
                column.partial_result = (column.operator)(column.partial_result, next_operand)
            }
            columns
        })
        .iter()
        .map(|column| column.partial_result)
        .sum()
}

struct Column {
    partial_result: usize,
    operator: Box<dyn Fn(usize, usize) -> usize>,
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 4648618073226);
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
        ),
        4277556
    )
}
