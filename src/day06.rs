use pathfinding::matrix::{directions, Matrix};
use test_case::test_case;

fn parse_input(filename: &str) -> Matrix<u8> {
    let input = std::fs::read_to_string(filename).unwrap();
    Matrix::from_rows(input.lines().map(|l| l.bytes())).unwrap()
}

fn dir_char(dir: (isize, isize)) -> u8 {
    if dir == directions::N {
        b'^'
    } else if dir == directions::E {
        b'>'
    } else if dir == directions::S {
        b'v'
    } else if dir == directions::W {
        b'<'
    } else {
        unreachable!()
    }
}

fn turn_right(dir: (isize, isize)) -> (isize, isize) {
    if dir == directions::N {
        directions::E
    } else if dir == directions::E {
        directions::S
    } else if dir == directions::S {
        directions::W
    } else if dir == directions::W {
        directions::N
    } else {
        unreachable!()
    }
}

fn do_step(map: &mut Matrix<u8>, pos: &mut (usize, usize), dir: &mut (isize, isize)) -> bool {
    let Some(next_pos) = map.move_in_direction(*pos, *dir) else {
        map[*pos] = dir_char(*dir);
        return false;
    };
    if map[next_pos] == b'#' {
        *dir = turn_right(*dir);
    } else {
        map[*pos] = dir_char(*dir);
        *pos = next_pos;
    }
    true
}

#[test_case("inputs/input-06.txt" => 5444)]
#[test_case("inputs/example-06-1.txt" => 41)]
pub fn puzzle1(filename: &str) -> i64 {
    let mut map = parse_input(filename);
    let mut pos = map.items().find(|(_, &c)| c == b'^').unwrap().0;
    let mut dir = directions::N;
    while do_step(&mut map, &mut pos, &mut dir) {}
    let ret = map
        .values()
        .filter(|&&c| matches!(c, b'^' | b'>' | b'v' | b'<'))
        .count() as i64;
    ret
}

#[test_case("inputs/input-06.txt" => 1946)]
#[test_case("inputs/example-06-1.txt" => 6)]
pub fn puzzle2(filename: &str) -> i64 {
    let orig_map = parse_input(filename);
    let mut map = orig_map.clone();
    let pos = map.items().find(|(_, &c)| c == b'^').unwrap().0;
    let dir = directions::N;
    let mut path = Vec::new();
    {
        let mut pos = pos;
        let mut dir = dir;
        while do_step(&mut map, &mut pos, &mut dir) {
            if !path.contains(&pos) {
                path.push(pos);
            }
        }
    }
    let mut sum = 0;
    for obs in path {
        map.as_mut().copy_from_slice(orig_map.as_ref());
        let mut pos = pos;
        let mut dir = dir;
        map[obs] = b'#';
        while do_step(&mut map, &mut pos, &mut dir) {
            if map[pos] == dir_char(dir) {
                sum += 1;
                break;
            }
        }
    }
    sum
}
