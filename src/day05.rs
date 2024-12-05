use test_case::test_case;

fn parse_input(filename: &str) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();
    let updates = updates
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

#[test_case("inputs/input-05.txt" => 5087)]
#[test_case("inputs/example-05-1.txt" => 143)]
pub fn puzzle1(filename: &str) -> i64 {
    let (rules, updates) = parse_input(filename);
    let mut sum = 0;
    'outer: for update in updates {
        for i in 0..update.len() - 1 {
            for (a, b) in &rules {
                if update[i] == *b {
                    for z in &update[i + 1..] {
                        if *z == *a {
                            continue 'outer;
                        }
                    }
                }
            }
        }
        sum += update[update.len() / 2];
    }
    sum
}

#[test_case("inputs/input-05.txt" => 4971)]
#[test_case("inputs/example-05-1.txt" => 123)]
pub fn puzzle2(filename: &str) -> i64 {
    let (rules, updates) = parse_input(filename);
    let mut sum = 0;
    'outer: for update in updates {
        for i in 0..update.len() - 1 {
            for (a, b) in &rules {
                if update[i] == *b {
                    for z in &update[i + 1..] {
                        if *z == *a {
                            let mut sorted = update.clone();
                            sorted.sort_by(|a, b| {
                                for (ra, rb) in rules.iter() {
                                    if *a == *ra && *b == *rb {
                                        return std::cmp::Ordering::Greater;
                                    } else if *a == *rb && *b == *ra {
                                        return std::cmp::Ordering::Less;
                                    }
                                }
                                std::cmp::Ordering::Equal
                            });
                            sum += sorted[sorted.len() / 2];
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }
    sum
}
