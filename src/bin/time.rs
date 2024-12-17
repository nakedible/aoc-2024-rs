use aoc_2024_rs::*;

use rayon::prelude::*;

fn main() {
    let fns = vec![
        || assert_eq!(day01::puzzle1("inputs/input-01.txt"), 1197984),
        || assert_eq!(day01::puzzle2("inputs/input-01.txt"), 23387399),
        || assert_eq!(day02::puzzle1("inputs/input-02.txt"), 534),
        || assert_eq!(day02::puzzle2("inputs/input-02.txt"), 577),
        || assert_eq!(day03::puzzle1("inputs/input-03.txt"), 159833790),
        || assert_eq!(day03::puzzle2("inputs/input-03.txt"), 89349241),
        || assert_eq!(day04::puzzle1("inputs/input-04.txt"), 2297),
        || assert_eq!(day04::puzzle2("inputs/input-04.txt"), 1745),
        || assert_eq!(day05::puzzle1("inputs/input-05.txt"), 5087),
        || assert_eq!(day05::puzzle2("inputs/input-05.txt"), 4971),
        || assert_eq!(day06::puzzle1("inputs/input-06.txt"), 5444),
        || assert_eq!(day06::puzzle2("inputs/input-06.txt"), 1946),
        || assert_eq!(day07::puzzle1("inputs/input-07.txt"), 5540634308362),
        || assert_eq!(day07::puzzle2("inputs/input-07.txt"), 472290821152397),
        || assert_eq!(day08::puzzle1("inputs/input-08.txt"), 354),
        || assert_eq!(day08::puzzle2("inputs/input-08.txt"), 1263),
        || assert_eq!(day09::puzzle1("inputs/input-09.txt"), 6299243228569),
        || assert_eq!(day09::puzzle2("inputs/input-09.txt"), 6326952672104),
        || assert_eq!(day10::puzzle1("inputs/input-10.txt"), 796),
        || assert_eq!(day10::puzzle2("inputs/input-10.txt"), 1942),
        || assert_eq!(day11::puzzle1("inputs/input-11.txt"), 218079),
        || assert_eq!(day11::puzzle2("inputs/input-11.txt"), 259755538429618),
        || assert_eq!(day12::puzzle1("inputs/input-12.txt"), 1396562),
        || assert_eq!(day12::puzzle2("inputs/input-12.txt"), 844132),
        || assert_eq!(day13::puzzle1("inputs/input-13.txt"), 36870),
        || assert_eq!(day13::puzzle2("inputs/input-13.txt"), 78101482023732),
        || assert_eq!(day14::puzzle1("inputs/input-14.txt", (103, 101)), 219512160),
        || assert_eq!(day14::puzzle2("inputs/input-14.txt", (103, 101)), 6398),
        || assert_eq!(day15::puzzle1("inputs/input-15.txt"), 1383666),
        || assert_eq!(day15::puzzle2("inputs/input-15.txt"), 1412866),
        || assert_eq!(day16::puzzle1("inputs/input-16.txt"), 106512),
        || assert_eq!(day16::puzzle2("inputs/input-16.txt"), 563),
        || assert_eq!(day17::puzzle1("inputs/input-17.txt"), "3,7,1,7,2,1,0,6,3"),
        || assert_eq!(day17::puzzle2("inputs/input-17.txt"), 37221334433268),
    ];
    fns.par_iter().for_each(|f| f());
}
