use test_case::test_case;

fn parse_input(filename: &str) -> Vec<(i64, Vec<i64>)> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut equations = Vec::new();
    for line in input.lines() {
        let (test, rest) = line.split_once(": ").unwrap();
        let test = test.parse().unwrap();
        let values = rest.split(" ").map(|s| s.parse().unwrap()).collect();
        equations.push((test, values));
    }
    equations
}

fn calc_value(test: i64, cur: i64, equations: &[i64]) -> bool {
    if equations.is_empty() {
        return test == cur;
    }
    calc_value(test, cur + equations[0], &equations[1..])
        || calc_value(test, cur * equations[0], &equations[1..])
}

#[test_case("inputs/input-07.txt" => 5540634308362)]
#[test_case("inputs/example-07-1.txt" => 3749)]
pub fn puzzle1(filename: &str) -> i64 {
    let equations = parse_input(filename);
    let mut sum = 0;
    for (test, values) in equations {
        if calc_value(test, values[0], &values[1..]) {
            sum += test;
        }
    }
    sum
}

// #[test_case(123, 456 => 123456)]
// #[test_case(1, 234 => 1234)]
// #[test_case(15, 6 => 156)]
fn concat(a: i64, b: i64) -> i64 {
    a * 10i64.pow(b.ilog10() + 1) + b
}

fn calc_value2(test: i64, cur: i64, equations: &[i64]) -> bool {
    if cur > test {
        return false;
    }
    if equations.is_empty() {
        return test == cur;
    }
    calc_value2(test, cur + equations[0], &equations[1..])
        || calc_value2(test, cur * equations[0], &equations[1..])
        || calc_value2(test, concat(cur, equations[0]), &equations[1..])
}

#[test_case("inputs/input-07.txt" => 472290821152397)]
#[test_case("inputs/example-07-1.txt" => 11387)]
pub fn puzzle2(filename: &str) -> i64 {
    let equations = parse_input(filename);
    let mut sum = 0;
    for (test, values) in equations {
        if calc_value2(test, values[0], &values[1..]) {
            sum += test;
        }
    }
    sum
}
