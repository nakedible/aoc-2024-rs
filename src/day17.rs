use test_case::test_case;

fn parse_input(filename: &str) -> ((u64, u64, u64), Vec<u8>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let (reg, prg) = input.split_once("\n\n").unwrap();
    let mut reglines = reg.lines();
    let (_, a) = reglines.next().unwrap().split_once(": ").unwrap();
    let a = a.parse().unwrap();
    let (_, b) = reglines.next().unwrap().split_once(": ").unwrap();
    let b = b.parse().unwrap();
    let (_, c) = reglines.next().unwrap().split_once(": ").unwrap();
    let c = c.parse().unwrap();
    let prg = prg.split_once(": ").unwrap().1;
    let prg = prg.trim().split(',').map(|s| s.parse().unwrap()).collect();
    ((a, b, c), prg)
}

fn combo(c: u8, reg: (u64, u64, u64)) -> u64 {
    match c {
        0 | 1 | 2 | 3 => c as u64,
        4 => reg.0,
        5 => reg.1,
        6 => reg.2,
        _ => unreachable!(),
    }
}

fn run(mut reg: (u64, u64, u64), prg: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut pc = 0;
    while pc < prg.len() {
        match prg[pc] {
            0 => {
                reg.0 = reg.0 / 2u64.pow(combo(prg[pc + 1], reg) as u32);
                pc += 2;
            }
            1 => {
                reg.1 = (reg.1 ^ prg[pc + 1] as u64) % 8;
                pc += 2;
            }
            2 => {
                reg.1 = combo(prg[pc + 1], reg) % 8;
                pc += 2;
            }
            3 => {
                if reg.0 != 0 {
                    pc = prg[pc + 1] as usize;
                } else {
                    pc += 2;
                }
            }
            4 => {
                reg.1 = reg.1 ^ reg.2;
                pc += 2;
            }
            5 => {
                output.push((combo(prg[pc + 1], reg) % 8) as u8);
                pc += 2;
            }
            6 => {
                reg.1 = reg.0 / 2u64.pow(combo(prg[pc + 1], reg) as u32);
                pc += 2;
            }
            7 => {
                reg.2 = reg.0 / 2u64.pow(combo(prg[pc + 1], reg) as u32);
                pc += 2;
            }
            _ => unreachable!(),
        }
    }
    output
}

#[test_case("inputs/input-17.txt" => "3,7,1,7,2,1,0,6,3")]
#[test_case("inputs/example-17-1.txt" => "4,6,3,5,6,3,5,2,1,0")]
pub fn puzzle1(filename: &str) -> String {
    let (reg, prg) = parse_input(filename);
    let output = run(reg, &prg);
    output
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn try_all(a: u64, p: usize, prg: &[u8]) -> Option<u64> {
    for i in 0..=7 {
        let try_a = a << 3 | i;
        let out = run((try_a, 0, 0), prg);
        if out[0] == prg[p] {
            if p == 0 {
                return Some(try_a);
            }
            if let Some(a) = try_all(try_a, p - 1, prg) {
                return Some(a);
            }
        }
    }
    None
}

#[test_case("inputs/input-17.txt" => 37221334433268)]
#[test_case("inputs/example-17-2.txt" => 117440)]
pub fn puzzle2(filename: &str) -> u64 {
    let (_, prg) = parse_input(filename);
    let Some(a) = try_all(0, prg.len() - 1, &prg) else {
        unreachable!("not found")
    };
    a
}
