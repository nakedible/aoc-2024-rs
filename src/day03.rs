use test_case::test_case;

#[test_case("inputs/input-03.txt" => 159833790)]
#[test_case("inputs/example-03-1.txt" => 161)]
pub fn puzzle1(filename: &str) -> i64 {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut sum = 0;
    for (ind, _) in data.match_indices("mul(") {
        let Some(end) = data[ind..].find(')') else {
            continue;
        };
        let Some((a, b)) = data[ind + 4..ind + end].split_once(',') else {
            continue;
        };
        let Ok(a) = a.trim().parse::<i64>() else {
            continue;
        };
        let Ok(b) = b.trim().parse::<i64>() else {
            continue;
        };
        let res = a * b;
        sum += res;
    }
    sum
}

#[test_case("inputs/input-03.txt" => 89349241)]
#[test_case("inputs/example-03-2.txt" => 48)]
pub fn puzzle2(filename: &str) -> i64 {
    let data = std::fs::read_to_string(filename).unwrap();
    let mut ind = 0;
    let mut sum = 0;
    let mut enabled = true;
    while ind < data.len() {
        if data[ind..].starts_with("don't()") {
            ind += 7;
            enabled = false;
        } else if data[ind..].starts_with("do()") {
            ind += 4;
            enabled = true;
        } else if enabled && data[ind..].starts_with("mul(") {
            ind += 4;
            let Some(end) = data[ind..].find(')') else {
                continue;
            };
            let Some((a, b)) = data[ind..ind + end].split_once(',') else {
                continue;
            };
            let Ok(a) = a.trim().parse::<i64>() else {
                continue;
            };
            let Ok(b) = b.trim().parse::<i64>() else {
                continue;
            };
            let res = a * b;
            sum += res;
            ind += end;
        } else {
            ind += 1;
        }
    }
    sum
}
