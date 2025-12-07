use std::collections::{BTreeMap, BTreeSet};

#[allow(unused)]
fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let start_x = lines.next().unwrap().find('S').unwrap();
    let splitters = lines
        .enumerate()
        .flat_map(|(y, line)| line.match_indices('^').map(move |(x, _)| (y, x)))
        .collect::<BTreeSet<Position>>();
    let max_y = input.lines().count() - 1;
    routes_from((0, start_x), &splitters, max_y, &mut BTreeMap::new())
}

type Position = (usize, usize);

fn routes_from(
    position: Position,
    splitters: &BTreeSet<Position>,
    max_y: usize,
    cache: &mut BTreeMap<Position, usize>,
) -> usize {
    if let Some(&result) = cache.get(&position) {
        result
    } else if position.0 >= max_y {
        1
    } else if splitters.contains(&position) {
        let left = routes_from((position.0 + 1, position.1 - 1), splitters, max_y, cache);
        let right = routes_from((position.0 + 1, position.1 + 1), splitters, max_y, cache);
        cache.insert(position, left + right);
        left + right
    } else {
        let down = routes_from((position.0 + 1, position.1), splitters, max_y, cache);
        cache.insert(position, down);
        down
    }
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}

#[test]
fn test_actual() {
    assert_eq!(solve(include_str!("input.txt")), 5748679033029);
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
        40
    )
}
