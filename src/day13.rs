use test_case::test_case;

fn parse_button_line(line: &str) -> (i64, i64) {
    let (x, y) = line.split_once(", ").unwrap();
    let x = x.split_once("+").unwrap().1.parse().unwrap();
    let y = y.split_once("+").unwrap().1.parse().unwrap();
    (x, y)
}

fn parse_prize_line(line: &str) -> (i64, i64) {
    let (x, y) = line.split_once(", ").unwrap();
    let x = x.split_once("=").unwrap().1.parse().unwrap();
    let y = y.split_once("=").unwrap().1.parse().unwrap();
    (x, y)
}

fn parse_input(filename: &str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    let input = std::fs::read_to_string(filename).unwrap();
    let mut machines = Vec::new();
    for machine in input.split("\n\n") {
        let mut lines = machine.lines();
        let button_a = parse_button_line(lines.next().unwrap());
        let button_b = parse_button_line(lines.next().unwrap());
        let prize = parse_prize_line(lines.next().unwrap());
        machines.push((button_a, button_b, prize));
    }
    machines
}

#[test_case("inputs/input-13.txt" => 36870)]
#[test_case("inputs/example-13-1.txt" => 480)]
pub fn puzzle1(filename: &str) -> i64 {
    let machines = parse_input(filename);
    let mut sum = 0;
    for (button_a, button_b, prize) in machines {
        'outer: for a in 0..=100 {
            for b in 0..=100 {
                let x = button_a.0 * a + button_b.0 * b;
                let y = button_a.1 * a + button_b.1 * b;
                if x == prize.0 && y == prize.1 {
                    let cost = a * 3 + b;
                    sum += cost;
                    break 'outer;
                }
            }
        }
    }
    sum
}

#[test_case("inputs/input-13.txt" => 78101482023732)]
#[test_case("inputs/example-13-1.txt" => 875318608908)]
pub fn puzzle2(filename: &str) -> i64 {
    let machines = parse_input(filename);
    let mut sum = 0;
    for (button_a, button_b, prize) in machines {
        let prize = (prize.0 + 10000000000000, prize.1 + 10000000000000);
        let b = (prize.1 * button_a.0 - prize.0 * button_a.1)
            / (button_b.1 * button_a.0 - button_b.0 * button_a.1);
        let a = (prize.0 - b * button_b.0) / button_a.0;
        if prize.0 == button_a.0 * a + button_b.0 * b && prize.1 == button_a.1 * a + button_b.1 * b
        {
            let cost = a * 3 + b;
            sum += cost;
        }
    }
    sum
}
