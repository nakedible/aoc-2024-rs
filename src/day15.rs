use pathfinding::matrix::{directions, Matrix};
use test_case::test_case;

fn parse_input(filename: &str) -> (Matrix<u8>, Vec<u8>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let (map, moves) = input.split_once("\n\n").unwrap();
    let map = Matrix::from_rows(map.lines().map(|l| l.bytes())).unwrap();
    let moves = moves.bytes().filter(|&b| b != b'\n').collect();
    (map, moves)
}

fn dir_from_char(c: u8) -> (isize, isize) {
    match c {
        b'^' => directions::N,
        b'<' => directions::W,
        b'>' => directions::E,
        b'v' => directions::S,
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn print_map(map: &Matrix<u8>) {
    for y in 0..map.rows {
        for x in 0..map.columns {
            print!("{}", map[(y as usize, x as usize)] as char);
        }
        println!();
    }
}

fn can_push(map: &Matrix<u8>, pos: (usize, usize), dir: (isize, isize)) -> bool {
    let next_pos = map.move_in_direction(pos, dir).unwrap();
    match map[next_pos] {
        b'#' => false,
        b'.' => true,
        b'O' => can_push(map, next_pos, dir),
        b'[' | b']' if dir.0 == 0 => can_push(map, next_pos, dir),
        c @ (b'[' | b']') => {
            let other_dir = if c == b'[' {
                directions::E
            } else {
                directions::W
            };
            let other = map.move_in_direction(next_pos, other_dir).unwrap();
            can_push(map, next_pos, dir) && can_push(map, other, dir)
        }
        _ => unreachable!(),
    }
}

fn do_push(map: &mut Matrix<u8>, pos: (usize, usize), dir: (isize, isize)) {
    let next_pos = map.move_in_direction(pos, dir).unwrap();
    match map[next_pos] {
        b'.' => {}
        b'O' => {
            do_push(map, next_pos, dir);
            map[next_pos] = b'.';
            let next_next_pos = map.move_in_direction(next_pos, dir).unwrap();
            map[next_next_pos] = b'O';
        }
        c @ (b'[' | b']') if dir.0 == 0 => {
            do_push(map, next_pos, dir);
            map[next_pos] = b'.';
            let next_next_pos = map.move_in_direction(next_pos, dir).unwrap();
            map[next_next_pos] = c;
        }
        c @ (b'[' | b']') => {
            let other_dir = if c == b'[' {
                directions::E
            } else {
                directions::W
            };
            let other = map.move_in_direction(next_pos, other_dir).unwrap();
            do_push(map, next_pos, dir);
            do_push(map, other, dir);
            map[next_pos] = b'.';
            map[other] = b'.';
            let next_next_pos = map.move_in_direction(next_pos, dir).unwrap();
            let next_next_other = map.move_in_direction(other, dir).unwrap();
            map[next_next_pos] = c;
            map[next_next_other] = if c == b'[' { b']' } else { b'[' };
        }
        _ => unreachable!(),
    }
}

fn push(map: &mut Matrix<u8>, pos: (usize, usize), dir: (isize, isize)) -> bool {
    if !can_push(map, pos, dir) {
        return false;
    }
    do_push(map, pos, dir);
    true
}

#[test_case("inputs/input-15.txt" => 1383666)]
#[test_case("inputs/example-15-1.txt" => 2028)]
#[test_case("inputs/example-15-2.txt" => 10092)]
pub fn puzzle1(filename: &str) -> i64 {
    let (mut map, dirs) = parse_input(filename);
    let mut pos = map.items().find(|(_, &c)| c == b'@').unwrap().0;
    map[pos] = b'.';
    for dir in dirs {
        let dir = dir_from_char(dir);
        if push(&mut map, pos, dir) {
            pos = map.move_in_direction(pos, dir).unwrap();
        }
    }
    map[pos] = b'@';
    //print_map(&map);
    map.items()
        .filter(|&(_, &c)| c == b'O')
        .map(|(p, _)| p.0 * 100 + p.1)
        .sum::<usize>() as i64
}

fn double_map(map: &Matrix<u8>) -> Matrix<u8> {
    let mut new_map = Matrix::new(map.rows, map.columns * 2, b'#');
    for y in 0..map.rows {
        for x in 0..map.columns {
            match map[(y, x)] {
                b'#' => {
                    new_map[(y, x * 2)] = b'#';
                    new_map[(y, x * 2 + 1)] = b'#';
                }
                b'.' => {
                    new_map[(y, x * 2)] = b'.';
                    new_map[(y, x * 2 + 1)] = b'.';
                }
                b'@' => {
                    new_map[(y, x * 2)] = b'@';
                    new_map[(y, x * 2 + 1)] = b'.';
                }
                b'O' => {
                    new_map[(y, x * 2)] = b'[';
                    new_map[(y, x * 2 + 1)] = b']';
                }
                _ => unreachable!(),
            }
        }
    }
    new_map
}

#[test_case("inputs/input-15.txt" => 1412866)]
#[test_case("inputs/example-15-1.txt" => 1751)]
#[test_case("inputs/example-15-2.txt" => 9021)]
#[test_case("inputs/example-15-3.txt" => 618)]
pub fn puzzle2(filename: &str) -> i64 {
    let (map, dirs) = parse_input(filename);
    let mut map = double_map(&map);
    let mut pos = map.items().find(|(_, &c)| c == b'@').unwrap().0;
    map[pos] = b'.';
    for dir in dirs {
        let dir = dir_from_char(dir);
        if push(&mut map, pos, dir) {
            pos = map.move_in_direction(pos, dir).unwrap();
        }
    }
    map[pos] = b'@';
    //print_map(&map);
    map.items()
        .filter(|&(_, &c)| c == b'[')
        .map(|(p, _)| p.0 * 100 + p.1)
        .sum::<usize>() as i64
}
