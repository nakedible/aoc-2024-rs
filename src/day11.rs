use test_case::test_case;
use std::collections::HashMap;

fn parse_input(filename: &str) -> Vec<i64> {
    let input = std::fs::read_to_string(filename).unwrap();
    input.trim().split(' ').map(|l| l.parse().unwrap()).collect()
}

//#[test_case(1234 => (12, 34))]
//#[test_case(1000 => (10, 0))]
//#[test_case(1234225555 => (12342, 25555))]
fn halve(i: i64) -> (i64, i64) {
    let digits = i.ilog10() + 1;
    let half = 10i64.pow(digits / 2);
    (i / half, i % half)
}

fn blink(cache: &mut HashMap<(i64, usize), i64>, i: i64, count: usize) -> i64 {
    if let Some(rocks) = cache.get(&(i, count)) {
        return *rocks;
    }
    if count == 0 {
        cache.insert((i, count), 1);
        return 1;
    }
    if i == 0 {
        let ret = blink(cache, 1, count - 1);
        cache.insert((i, count), ret);
        return ret;
    }
    let digits = i.ilog10() + 1;
    if digits % 2 == 0 {
        let (a, b) = halve(i);
        let ret = blink(cache, a, count - 1) + blink(cache, b, count - 1);
        cache.insert((i, count), ret);
        return ret;
    }
    let ret = blink(cache, i * 2024, count - 1);
    cache.insert((i, count), ret);
    ret
}

#[test_case("inputs/input-11.txt" => 218079)]
#[test_case("inputs/example-11-1.txt" => 55312)]
pub fn puzzle1(filename: &str) -> i64 {
    let input = parse_input(filename);
    let mut cache = HashMap::new();
    input.iter().map(|&i| blink(&mut cache, i, 25)).sum()
}

#[test_case("inputs/input-11.txt" => 259755538429618)]
pub fn puzzle2(filename: &str) -> i64 {
    let input = parse_input(filename);
    let mut cache = HashMap::new();
    input.iter().map(|&i| blink(&mut cache, i, 75)).sum()
}
