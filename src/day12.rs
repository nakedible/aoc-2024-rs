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
    let mut edges = Matrix::new(garden.rows * 3, garden.columns * 3, b' ');
    while let Some(pos) = queue.pop_front() {
        if seent.contains(&pos) {
            continue;
        }
        seent.insert(pos);
        area += 1;
        edges[(pos.0 * 3 + 1 - 1, pos.1 * 3 + 1 + 0)] = b'#';
        edges[(pos.0 * 3 + 1 + 1, pos.1 * 3 + 1 + 0)] = b'#';
        edges[(pos.0 * 3 + 1 + 0, pos.1 * 3 + 1 - 1)] = b'#';
        edges[(pos.0 * 3 + 1 + 0, pos.1 * 3 + 1 + 1)] = b'#';
        for dir in DIRECTIONS_4.iter() {
            let Some(neigh) = garden.move_in_direction(pos, *dir) else {
                continue;
            };
            if garden[neigh] == garden[pos] {
                edges[(
                    pos.0 * 3 + (1 + dir.0) as usize,
                    pos.1 * 3 + (1 + dir.1) as usize,
                )] = b' ';
                queue.push_back(neigh);
            }
        }
    }
    for i in 0..garden.rows * 3 {
        for j in 0..garden.columns * 3 {
            if j >= 3 && edges[(i, j)] == b'#' && edges[(i, j - 3)] == b'#' {
                edges[(i, j - 3)] = b' ';
            }
        }
    }
    for j in 0..garden.columns * 3 {
        for i in 0..garden.rows * 3 {
            if i >= 3 && edges[(i, j)] == b'#' && edges[(i - 3, j)] == b'#' {
                edges[(i - 3, j)] = b' ';
            }
        }
    }
    area * edges.values().filter(|&&b| b == b'#').count() as i64
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
