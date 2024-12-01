use test_case::test_case;

pub fn parse_input(filename: &str) -> (Vec<i64>, Vec<i64>) {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for line in data.lines() {
        let parts: Vec<i64> = line.split("   ").map(|x| x.parse().unwrap()).collect();
        a.push(parts[0]);
        b.push(parts[1]);
    }
    (a, b)
}

#[test_case("inputs/input-01.txt" => 1197984)]
#[test_case("inputs/example-01-1.txt" => 0)]
pub fn puzzle1(filename: &str) -> i64 {
    let (mut a, mut b) = parse_input(filename);

    a.sort();
    b.sort();
    let ret = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();

    ret
}


#[test_case("inputs/input-01.txt" => 0)]
pub fn puzzle2(filename: &str) -> i64 {
    let (a, b) = parse_input(filename);

    let mut sum = 0;
    for v in a.iter() {
        sum += v * (b.iter().filter(|&x| x == v).count()) as i64;
    }
 
    sum
}
