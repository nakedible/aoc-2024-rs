use std::collections::HashSet;

use itertools::Itertools;
use test_case::test_case;

fn parse_input(filename: &str) -> ((isize, isize), Vec<(u8, (isize, isize))>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut nodes = vec![];
    let mut rows = 0;
    let mut cols = 0;
    for (row, line) in input.lines().enumerate() {
        rows = row + 1;
        for (col, val) in line.chars().enumerate() {
            cols = col + 1;
            if val == '.' {
                continue;
            }
            nodes.push((val as u8, (row as isize, col as isize)));
        }
    }
    ((rows as isize, cols as isize), nodes)
}

fn in_bounds(size: (isize, isize), pos: (isize, isize)) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < size.0 && pos.1 < size.1
}

#[test_case("inputs/input-08.txt" => 354)]
#[test_case("inputs/example-08-1.txt" => 14)]
pub fn puzzle1(filename: &str) -> i64 {
    let (size, mut nodes) = parse_input(filename);
    println!("{:?}", size);
    nodes.sort();
    let mut seent = HashSet::new();
    for (_, poss) in &nodes.iter().chunk_by(|x| x.0) {
        let poss = poss.map(|x| x.1).collect::<Vec<_>>();
        for (a, b) in poss.into_iter().tuple_combinations() {
            let diff = ((b.0 as isize - a.0 as isize), (b.1 as isize - a.1 as isize));
            let antia = (a.0 - diff.0, a.1 - diff.1);
            if in_bounds(size, antia) {
                seent.insert(antia);
            }
            let antib = (b.0 + diff.0, b.1 + diff.1);
            if in_bounds(size, antib) {
                seent.insert(antib);
            }
        }
    }
    seent.len() as i64
}

#[test_case("inputs/input-08.txt" => 1263)]
#[test_case("inputs/example-08-1.txt" => 34)]
pub fn puzzle2(filename: &str) -> i64 {
    let (size, mut nodes) = parse_input(filename);
    println!("{:?}", size);
    nodes.sort();
    let mut seent = HashSet::new();
    for (_, poss) in &nodes.iter().chunk_by(|x| x.0) {
        let poss = poss.map(|x| x.1).collect::<Vec<_>>();
        for (a, b) in poss.into_iter().tuple_combinations() {
            let diff = ((b.0 as isize - a.0 as isize), (b.1 as isize - a.1 as isize));
            let mut newpos = a;
            while in_bounds(size, newpos) {
                seent.insert(newpos);
                newpos = (newpos.0 - diff.0, newpos.1 - diff.1);
            }
            newpos = b;
            while in_bounds(size, newpos) {
                seent.insert(newpos);
                newpos = (newpos.0 + diff.0, newpos.1 + diff.1);
            }
        }
    }
    seent.len() as i64
}
