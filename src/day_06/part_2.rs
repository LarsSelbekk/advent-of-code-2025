use itertools::Itertools;
use std::iter;

#[allow(unused)]
fn solve(input: &str) -> usize {
    let mut lines = input.lines().map(|line| line.chars()).collect_vec();
    let mut operators_line = lines.pop().unwrap();

    let mut operators = iter::from_fn(|| {
        let operator = operators_line.next()?;
        operators_line
            .take_while_ref(|&c| c == ' ')
            .collect::<String>();
        Some(operator)
    });

    let mut operands = iter::from_fn(|| {
        Some(
            lines
                .iter_mut()
                .map(|it| it.next())
                .fold(None, |mut col, next| match (col, next) {
                    (None, Some(digit)) => Some(digit.to_string()),
                    (Some(number), None) => Some(number),
                    (Some(mut number), Some(digit)) => {
                        number.push(digit);
                        Some(number)
                    }
                    (None, None) => None,
                })?
                .trim()
                .parse::<usize>()
                .ok(),
        )
    })
    .peekable();

    let mut operand_groups = iter::from_fn(|| {
        let operands_in_group = operands
            .peeking_take_while(|operand| operand.is_some())
            .collect::<Option<Vec<usize>>>()?;
         operands.next();
        if operands_in_group.is_empty() {
            None
        } else {
            Some(operands_in_group)
        }
    });

    operand_groups
        .zip_eq(operators)
        .map(|(operands, operator)| {
            operands.iter().fold(
                if operator == '+' { 0 } else { 1 },
                if operator == '+' {
                    std::ops::Add::add
                } else {
                    std::ops::Mul::mul
                },
            )
        })
        .sum::<usize>()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 7329921182115);
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
        3263827
    )
}
