use crate::day_08::part_1::{Position, squared_distance};
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[allow(unused)]
fn solve(input: &str) -> usize {
    let positions = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|word| word.parse().unwrap())
                .collect_tuple::<Position>()
                .unwrap()
        })
        .collect_vec();
    let mut distances =
        BinaryHeap::from_iter(positions.iter().copied().combinations(2).map(|positions| {
            (
                Reverse(squared_distance(positions[0], positions[1])),
                positions,
            )
        }));
    let mut representatives: HashMap<Position, Position> =
        positions.iter().copied().map(|p| (p, p)).collect();

    let mut last_positions = vec![];
    while let Some((distance, positions_to_check)) = distances.pop()
        && positions
            .iter()
            .copied()
            .filter(|&p| p == get_representative(p, &mut representatives))
            .nth(1)
            .is_some()
    {
        last_positions = positions_to_check.clone();
        let a_rep = get_representative(positions_to_check[0], &mut representatives);
        let b_rep = get_representative(positions_to_check[1], &mut representatives);
        representatives.insert(b_rep, a_rep);
    }
    last_positions.iter().map(|position| position.0).product()
}

fn get_representative(
    position: Position,
    representatives: &mut HashMap<Position, Position>,
) -> Position {
    if representatives[&position] == position {
        position
    } else {
        let representative = get_representative(representatives[&position], representatives);
        representatives.insert(position, representative);
        representative
    }
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 107256172);
}

#[test]
pub fn test_example() {
    assert_eq!(
        solve(
            "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
        ),
        25272
    )
}
