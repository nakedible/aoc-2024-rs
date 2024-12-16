use std::collections::HashSet;

use pathfinding::{
    directed::dijkstra,
    matrix::{directions, Matrix},
    prelude::astar_bag,
};
use test_case::test_case;

fn parse_input(filename: &str) -> Matrix<u8> {
    let input = std::fs::read_to_string(filename).unwrap();
    Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap()
}

#[allow(dead_code)]
fn print_map(map: &Matrix<u8>) {
    for row in 0..map.rows {
        for col in 0..map.columns {
            print!("{}", map[(row as usize, col as usize)] as char);
        }
        println!();
    }
}

fn turn_right(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        directions::N => directions::E,
        directions::E => directions::S,
        directions::S => directions::W,
        directions::W => directions::N,
        _ => unreachable!(),
    }
}

fn turn_left(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        directions::N => directions::W,
        directions::E => directions::N,
        directions::S => directions::E,
        directions::W => directions::S,
        _ => unreachable!(),
    }
}

fn neighbours(
    map: &Matrix<u8>,
    pos: (usize, usize),
    dir: (isize, isize),
) -> impl IntoIterator<Item = (((usize, usize), (isize, isize)), i64)> {
    let left = Some(((pos, turn_left(dir)), 1000));
    let right = Some(((pos, turn_right(dir)), 1000));
    let straight = map
        .move_in_direction(pos, dir)
        .filter(|&next_pos| map[next_pos] != b'#')
        .map(|next_pos| ((next_pos, dir), 1));
    straight.into_iter().chain(left).chain(right)
}

#[test_case("inputs/input-16.txt" => 106512)]
#[test_case("inputs/example-16-1.txt" => 7036)]
#[test_case("inputs/example-16-2.txt" => 11048)]
pub fn puzzle1(filename: &str) -> i64 {
    let map = parse_input(filename);
    //print_map(&map);
    let goal = map.items().find(|(_, &c)| c == b'E').unwrap().0;
    let pos = map.items().find(|(_, &c)| c == b'S').unwrap().0;
    let dir = directions::E;
    let Some((_, cost)) = dijkstra::dijkstra(
        &(pos, dir),
        |&(pos, dir)| neighbours(&map, pos, dir),
        |&(pos, _)| pos == goal,
    ) else {
        unreachable!("no path found");
    };
    cost
}

#[test_case("inputs/input-16.txt" => 563)]
#[test_case("inputs/example-16-1.txt" => 45)]
#[test_case("inputs/example-16-2.txt" => 64)]
pub fn puzzle2(filename: &str) -> i64 {
    let map = parse_input(filename);
    //print_map(&map);
    let goal = map.items().find(|(_, &c)| c == b'E').unwrap().0;
    let pos = map.items().find(|(_, &c)| c == b'S').unwrap().0;
    let dir = directions::E;
    let mut pathposs = HashSet::new();
    let Some((bag, _)) = astar_bag(
        &(pos, dir),
        |&(pos, dir)| neighbours(&map, pos, dir),
        |&(_, _)| 1,
        |&(pos, _)| pos == goal,
    ) else {
        unreachable!("no path found");
    };
    for path in bag {
        for (pos, _) in path {
            pathposs.insert(pos);
        }
    }
    pathposs.len() as i64
}
