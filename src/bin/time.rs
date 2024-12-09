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
    ];
    fns.par_iter().for_each(|f| f());
}