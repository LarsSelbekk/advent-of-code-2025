#[allow(unused)]
fn solve(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|n| n.split_once('-').unwrap())
        .map(|(lower_bound, upper_bound)| {
            (
                lower_bound.parse::<usize>().unwrap(),
                upper_bound.parse::<usize>().unwrap(),
            )
        })
        .flat_map(|(lower_bound, upper_bound)| (lower_bound..=upper_bound).filter(is_repeated))
        .sum()
}

fn is_repeated(number: &usize) -> bool {
    let num_digits = number.ilog10() + 1;
    let pivot = 10_usize.pow(num_digits / 2);
    num_digits % 2 == 0 && number / pivot == number % pivot
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 44854383294);
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
        ),
        1227775554
    )
}

#[test]
fn test_is_repeated() {
    assert_eq!(is_repeated(&11), true);
    assert_eq!(is_repeated(&12), false);
    assert_eq!(is_repeated(&1), false);
    assert_eq!(is_repeated(&1212), true);
    assert_eq!(is_repeated(&111), false);
    assert_eq!(is_repeated(&38593859), true);
}
