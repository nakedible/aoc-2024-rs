use pathfinding::matrix::{directions, Matrix};
use test_case::test_case;

fn parse_input(filename: &str) -> Matrix<u8> {
    let input = std::fs::read_to_string(filename).unwrap();
    Matrix::from_rows(input.lines().map(|l| l.bytes().map(|c| c - b'0'))).unwrap()
}

fn neighbours(map: &Matrix<u8>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let ret = directions::DIRECTIONS_4
        .iter()
        .flat_map(|dir| map.move_in_direction(pos, *dir))
        .filter(|&p| map[p] == map[pos] + 1)
        .collect();
    ret
}

#[test_case("inputs/input-10.txt" => 796)]
#[test_case("inputs/example-10-1.txt" => 36)]
pub fn puzzle1(filename: &str) -> i64 {
    let map = parse_input(filename);
    let heads = map.items().filter(|(_, &c)| c == 0).collect::<Vec<_>>();
    let mut total = 0;
    for (head, _) in heads.iter() {
        let count = pathfinding::directed::bfs::bfs_reach(*head, |&pos| neighbours(&map, pos))
            .filter(|&pos| map[pos] == 9)
            .count();
        total += count;
    }
    total as i64
}

#[test_case("inputs/input-10.txt" => 1942)]
#[test_case("inputs/example-10-1.txt" => 81)]
pub fn puzzle2(filename: &str) -> i64 {
    let map = parse_input(filename);
    let heads = map.items().filter(|(_, &c)| c == 0).collect::<Vec<_>>();
    let mut total = 0;
    for (head, _) in heads.iter() {
        let count = pathfinding::directed::count_paths::count_paths(
            *head,
            |&pos| neighbours(&map, pos),
            |&pos| map[pos] == 9,
        );
        total += count
    }
    total as i64
}
