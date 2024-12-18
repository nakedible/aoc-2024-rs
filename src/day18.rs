use pathfinding::{directed::astar, matrix::Matrix};
use test_case::test_case;

fn parse_input(filename: &str) -> Vec<(i64, i64)> {
    let input = std::fs::read_to_string(filename).unwrap();
    input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

#[allow(dead_code)]
fn print_map(map: &Matrix<u8>) {
    for row in 0..map.rows {
        for column in 0..map.columns {
            print!("{}", map[(row, column)] as char);
        }
        println!();
    }
}

#[test_case("inputs/input-18.txt", 71, 1024 => 356)]
#[test_case("inputs/example-18-1.txt", 7, 12 => 22)]
pub fn puzzle1(filename: &str, size: usize, count: usize) -> i64 {
    let bytes = parse_input(filename);
    let mut map = Matrix::new(size, size, b'.');
    for i in 0..count {
        let (x, y) = bytes[i];
        map[(y as usize, x as usize)] = b'#';
    }
    let (_, cost) = astar::astar(
        &(0, 0),
        |&(x, y)| {
            let mut result = Vec::new();
            if x > 0 && map[(y, x - 1)] != b'#' {
                result.push(((x - 1, y), 1));
            }
            if y > 0 && map[(y - 1, x)] != b'#' {
                result.push(((x, y - 1), 1));
            }
            if x < size - 1 && map[(y, x + 1)] != b'#' {
                result.push(((x + 1, y), 1));
            }
            if y < size - 1 && map[(y + 1, x)] != b'#' {
                result.push(((x, y + 1), 1));
            }
            result
        },
        |&(x, y)| (size - x - 1) + (size - y - 1),
        |&(x, y)| x == size - 1 && y == size - 1,
    )
    .unwrap();
    cost as i64
}

#[test_case("inputs/input-18.txt", 71, 1024 => (22, 33))]
#[test_case("inputs/example-18-1.txt", 7, 12 => (6, 1))]
pub fn puzzle2(filename: &str, size: usize, count: usize) -> (i64, i64) {
    let bytes = parse_input(filename);
    let mut map = Matrix::new(size, size, b'.');
    let mut prevpath = Vec::new();
    for i in 0..bytes.len() {
        let (x, y) = bytes[i];
        map[(y as usize, x as usize)] = b'#';
        if i < count {
            continue;
        }
        if !prevpath.contains(&(x as usize, y as usize)) && !prevpath.is_empty() {
            continue;
        }
        let Some((path, _)) = astar::astar(
            &(0, 0),
            |&(x, y)| {
                let mut result = Vec::new();
                if x > 0 && map[(y, x - 1)] != b'#' {
                    result.push(((x - 1, y), 1));
                }
                if y > 0 && map[(y - 1, x)] != b'#' {
                    result.push(((x, y - 1), 1));
                }
                if x < size - 1 && map[(y, x + 1)] != b'#' {
                    result.push(((x + 1, y), 1));
                }
                if y < size - 1 && map[(y + 1, x)] != b'#' {
                    result.push(((x, y + 1), 1));
                }
                result
            },
            |&(x, y)| (size - x - 1) + (size - y - 1),
            |&(x, y)| x == size - 1 && y == size - 1,
        ) else {
            return (x, y);
        };
        prevpath = path;
    }
    unreachable!()
}
