use std::collections::BTreeSet;

#[allow(unused)]
fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let start_position = lines.next().unwrap().find('S').unwrap();
    lines
        .map(|line| line.match_indices('^').map(|(i, _)| i))
        .fold(
            (BTreeSet::from([start_position]), 0),
            |(mut beams, num_splits), splitters| {
                let hit_splits: BTreeSet<usize> = BTreeSet::from_iter(splitters)
                    .intersection(&beams)
                    .map(|b| *b)
                    .collect();
                beams.extend(hit_splits.iter().flat_map(|&split| [split - 1, split + 1]));
                for hit_split in hit_splits.iter() {
                    beams.remove(hit_split);
                }
                (beams, num_splits + hit_splits.len())
            },
        )
        .1
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 1587);
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
        ),
        21
    )
}
