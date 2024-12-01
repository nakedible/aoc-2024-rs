use test_case::test_case;

pub fn parse_input(filename: &str) -> (Vec<i64>, Vec<i64>) {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in data.lines() {
        let (av, bv) = line.split_once("   ").unwrap();
        a.push(av.parse().unwrap());
        b.push(bv.parse().unwrap());
    }
    (a, b)
}

#[test_case("inputs/input-01.txt" => 1197984)]
#[test_case("inputs/example-01-1.txt" => 11)]
pub fn puzzle1(filename: &str) -> i64 {
    let (mut a, mut b) = parse_input(filename);

    a.sort();
    b.sort();
    let ret = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();
    ret
}


#[test_case("inputs/input-01.txt" => 23387399)]
#[test_case("inputs/example-01-1.txt" => 31)]
pub fn puzzle2(filename: &str) -> i64 {
    let (a, b) = parse_input(filename);

    let counts = b.iter().fold(std::collections::HashMap::new(), |mut acc, x| {
        *acc.entry(*x).or_insert(0i64) += 1;
        acc
    });
    let ret = a.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum();
    ret
}
