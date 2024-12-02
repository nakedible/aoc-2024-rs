use test_case::test_case;

fn parse_input(filename: &str) -> Vec<Vec<i64>> {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut ret = Vec::new();
    for line in data.lines() {
        ret.push(
            line.split(' ')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i64>>(),
        );
    }
    ret
}

#[test_case("inputs/input-02.txt" => 534)]
#[test_case("inputs/example-02-1.txt" => 2)]
pub fn puzzle1(filename: &str) -> i64 {
    let reports = parse_input(filename);
    let mut ret = 0;
    for report in reports {
        let diffs: Vec<i64> = report.windows(2).map(|x| x[1] - x[0]).collect();
        if diffs.iter().all(|x| *x >= 1 && *x <= 3) {
            ret += 1;
        }
        if diffs.iter().all(|x| *x >= -3 && *x <= -1) {
            ret += 1;
        }
    }
    ret
}

#[test_case("inputs/input-02.txt" => 577)]
#[test_case("inputs/example-02-1.txt" => 4)]
pub fn puzzle2(filename: &str) -> i64 {
    let reports = parse_input(filename);
    let mut ret = 0;
    for report in reports {
        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);
            let diffs: Vec<i64> = report.windows(2).map(|x| x[1] - x[0]).collect();
            let mut safe = false;
            if diffs.iter().all(|x| *x >= 1 && *x <= 3) {
                safe = true;
            }
            if diffs.iter().all(|x| *x >= -3 && *x <= -1) {
                safe = true;
            }
            if safe {
                ret += 1;
                break;
            }
        }
    }
    ret
}
