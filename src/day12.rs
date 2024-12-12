use pathfinding::matrix::{directions::DIRECTIONS_4, Matrix};
use std::collections::{HashSet, VecDeque};
use test_case::test_case;

fn parse_input(filename: &str) -> Matrix<u8> {
    let input = std::fs::read_to_string(filename).unwrap();
    Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap()
}

fn count_plot(
    garden: &Matrix<u8>,
    seent: &mut HashSet<(usize, usize)>,
    pos: (usize, usize),
) -> i64 {
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    let mut area = 0;
    let mut edges = 0;
    while let Some(pos) = queue.pop_front() {
        if seent.contains(&pos) {
            continue;
        }
        seent.insert(pos);
        area += 1;
        edges += 4;
        for neigh in garden
            .neighbours(pos, false)
            .filter(|&p| garden[p] == garden[pos])
        {
            edges -= 1;
            queue.push_back(neigh);
        }
    }
    area * edges
}

#[test_case("inputs/input-12.txt" => 1396562)]
#[test_case("inputs/example-12-1.txt" => 140)]
#[test_case("inputs/example-12-2.txt" => 772)]
#[test_case("inputs/example-12-3.txt" => 1930)]
pub fn puzzle1(filename: &str) -> i64 {
    let garden = parse_input(filename);
    let mut seent = HashSet::new();
    let mut cost = 0;
    for pos in garden.keys() {
        if seent.contains(&pos) {
            continue;
        }
        let c = count_plot(&garden, &mut seent, pos);
        cost += c;
    }
    cost
}

fn count_plot2(
    garden: &Matrix<u8>,
    seent: &mut HashSet<(usize, usize)>,
    pos: (usize, usize),
) -> i64 {
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    let mut area = 0;
    let mut edges: HashSet<(isize, isize)> = HashSet::new();
    while let Some(pos) = queue.pop_front() {
        if seent.contains(&pos) {
            continue;
        }
        seent.insert(pos);
        area += 1;
        edges.insert((pos.0 as isize * 3 - 1, pos.1 as isize * 3 + 0));
        edges.insert((pos.0 as isize * 3 + 0, pos.1 as isize * 3 + 1));
        edges.insert((pos.0 as isize * 3 + 1, pos.1 as isize * 3 + 0));
        edges.insert((pos.0 as isize * 3 + 0, pos.1 as isize * 3 - 1));
        for dir in DIRECTIONS_4.iter() {
            let Some(neigh) = garden.move_in_direction(pos, *dir) else {
                continue;
            };
            if garden[neigh] == garden[pos] {
                edges.remove(&(pos.0 as isize * 3 + dir.0, pos.1 as isize * 3 + dir.1));
                queue.push_back(neigh);
            }
        }
    }
    for i in -1..garden.rows as isize * 3 {
        for j in -1..garden.columns as isize * 3 {
            if edges.contains(&(i, j)) && edges.contains(&(i, j - 3)) {
                edges.remove(&(i, j - 3));
            }
        }
    }
    for j in -1..garden.columns as isize * 3 {
        for i in -1..garden.rows as isize * 3 {
            if edges.contains(&(i, j)) && edges.contains(&(i - 3, j)) {
                edges.remove(&(i - 3, j));
            }
        }
    }
    area * edges.len() as i64
}

#[test_case("inputs/input-12.txt" => 844132)]
#[test_case("inputs/example-12-1.txt" => 80)]
#[test_case("inputs/example-12-2.txt" => 436)]
#[test_case("inputs/example-12-3.txt" => 1206)]
#[test_case("inputs/example-12-4.txt" => 236)]
#[test_case("inputs/example-12-5.txt" => 368)]
pub fn puzzle2(filename: &str) -> i64 {
    let garden = parse_input(filename);
    let mut seent = HashSet::new();
    let mut cost = 0;
    for pos in garden.keys() {
        if seent.contains(&pos) {
            continue;
        }
        let c = count_plot2(&garden, &mut seent, pos);
        cost += c;
    }
    cost
}
