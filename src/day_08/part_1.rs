use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

pub type Position = (usize, usize, usize);

#[allow(unused)]
fn solve(input: &str, connections: usize) -> usize {
    let positions = input.lines().map(|line| {
        line.split(',')
            .map(|word| word.parse().unwrap())
            .collect_tuple::<Position>()
            .unwrap()
    });
    let mut distances = BinaryHeap::from_iter(positions.combinations(2).map(|positions| {
        (
            Reverse(squared_distance(positions[0], positions[1])),
            positions,
        )
    }));
    let mut sets: Vec<BTreeSet<Position>> = vec![];
    let mut count = 0;
    while let Some((distance, positions)) = distances.pop()
        && count < connections
    {
        count += 1;
        let set_a = sets.iter().find_position(|set| set.contains(&positions[0]));
        let set_b = sets.iter().find_position(|set| set.contains(&positions[1]));
        if let Some((a_i, _)) = set_a
            && let Some((b_i, _)) = set_b
            && a_i == b_i
        {
            continue;
        }
        match (set_a, set_b) {
            (Some((a_index, _)), Some((b_index, set_b))) => {
                let it = set_b.iter().copied().collect_vec().into_iter();
                sets[a_index].extend(it);
                sets.remove(b_index);
            }
            (Some((a_index, _)), None) => {
                sets[a_index].insert(positions[1]);
            }
            (None, Some((b_index, mut set_b))) => {
                sets[b_index].insert(positions[0]);
            }
            (None, None) => {
                sets.push(BTreeSet::from_iter(positions));
            }
        }
    }
    sets.into_iter()
        .map(|set| set.len())
        .k_largest_relaxed(3)
        .product()
}

pub fn squared_distance(a: Position, b: Position) -> usize {
    a.0.abs_diff(b.0).pow(2) + a.1.abs_diff(b.1).pow(2) + a.2.abs_diff(b.2).pow(2)
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt"), 1000));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt"), 1000), 54600);
}

#[test]
fn test_example() {
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
425,690,689",
            10
        ),
        40
    )
}
