use pathfinding::matrix::Matrix;
use test_case::test_case;

fn parse_input(filename: &str) -> Vec<((i64, i64), (i64, i64))> {
    let input = std::fs::read_to_string(filename).unwrap();
    // example line: p=0,4 v=3,-3
    input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(' ').unwrap();
            let (px, py) = p.split_once('=').unwrap().1.split_once(',').unwrap();
            let (vx, vy) = v.split_once('=').unwrap().1.split_once(',').unwrap();
            // flipped
            (
                (py.trim().parse().unwrap(), px.trim().parse().unwrap()),
                (vy.trim().parse().unwrap(), vx.trim().parse().unwrap()),
            )
        })
        .collect()
}

fn step(robots: &mut Vec<((i64, i64), (i64, i64))>, s: (i64, i64)) {
    for (p, v) in robots.iter_mut() {
        *p = ((p.0 + v.0).rem_euclid(s.0), (p.1 + v.1).rem_euclid(s.1));
    }
}

#[test_case("inputs/input-14.txt", (103, 101) => 219512160)]
#[test_case("inputs/example-14-1.txt", (7, 11) => 12)]
pub fn puzzle1(filename: &str, s: (usize, usize)) -> i64 {
    let mut robots = parse_input(filename);
    for _ in 0..100 {
        step(&mut robots, (s.0 as i64, s.1 as i64));
    }
    let mut quadrants = Matrix::new(2, 2, 0i64);
    let mid = (s.0 as i64 / 2, s.1 as i64 / 2);
    for (p, _) in robots.iter() {
        if p.0 == mid.0 || p.1 == mid.1 {
            continue;
        }
        let q = (
            if p.0 < mid.0 { 0 } else { 1 },
            if p.1 < mid.1 { 0 } else { 1 },
        );
        quadrants[q] += 1;
    }
    quadrants.values().product()
}

#[test_case("inputs/input-14.txt", (103, 101) => 6398)]
pub fn puzzle2(filename: &str, s: (usize, usize)) -> i64 {
    let mut robots = parse_input(filename);
    // let mut map = Matrix::new(s.0, s.1, 0i64);
    let mut solution = 0;
    'outer: for i in 1..100000 {
        step(&mut robots, (s.0 as i64, s.1 as i64));
        robots.sort();
        let mut run = 0;
        let mut prev_x = 0;
        for (p, _) in robots.iter() {
            if p.1 == prev_x + 1 {
                run += 1;
                if run > 20 {
                    solution = i;
                    break 'outer;
                }
            } else {
                run = 0;
            }
            prev_x = p.1;
        }
    }
    // for (p, _) in robots.iter() {
    //     map[(p.0 as usize, p.1 as usize)] += 1;
    // }
    // for y in 0..map.rows {
    //     for x in 0..map.columns {
    //         print!("{:1}", if map[(y, x)] > 0 { '#' } else { ' ' });
    //     }
    //     println!();
    // }
    // map.fill(0);
    solution
}
