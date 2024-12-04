use pathfinding::matrix::Matrix;
use test_case::test_case;

fn parse_input(filename: &str) -> Matrix<char> {
    let input = std::fs::read_to_string(filename).unwrap();
    let matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();
    matrix
}

#[test_case("inputs/input-04.txt" => 2297)]
pub fn puzzle1(filename: &str) -> i64 {
    let input = parse_input(filename);
    let mut ret = 0;
    for dir in pathfinding::matrix::directions::DIRECTIONS_8.iter() {
        for pos in input.keys() {
            if input[pos] != 'X' {
                continue;
            }
            let Some(pos) = input.move_in_direction(pos, *dir) else {
                continue;
            };
            if input[pos] != 'M' {
                continue;
            }
            let Some(pos) = input.move_in_direction(pos, *dir) else {
                continue;
            };
            if input[pos] != 'A' {
                continue;
            }
            let Some(pos) = input.move_in_direction(pos, *dir) else {
                continue;
            };
            if input[pos] != 'S' {
                continue;
            }
            ret += 1;
        }
    }
    ret
}

#[test_case("inputs/input-04.txt" => 1745)]
pub fn puzzle2(filename: &str) -> i64 {
    let input = parse_input(filename);
    let mut hits = std::collections::HashMap::new();
    for dir in [
        pathfinding::matrix::directions::NE,
        pathfinding::matrix::directions::NW,
        pathfinding::matrix::directions::SE,
        pathfinding::matrix::directions::SW,
    ]
    .iter()
    {
        for s in input.keys() {
            if input[s] != 'M' {
                continue;
            }
            let Some(m) = input.move_in_direction(s, *dir) else {
                continue;
            };
            if input[m] != 'A' {
                continue;
            }
            let Some(e) = input.move_in_direction(m, *dir) else {
                continue;
            };
            if input[e] != 'S' {
                continue;
            }
            hits.entry(m).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    hits.values().filter(|&&v| v == 2).count() as i64
}
