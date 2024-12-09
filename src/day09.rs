use test_case::test_case;

fn parse_input(filename: &str) -> Vec<i64> {
    let input = std::fs::read_to_string(filename).unwrap();
    input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

#[test_case("inputs/input-09.txt" => 6299243228569)]
#[test_case("inputs/example-09-1.txt" => 1928)]
pub fn puzzle1(filename: &str) -> i64 {
    let map = parse_input(filename);
    let mut free = false;
    let mut disk = Vec::new();
    let mut id = 0;
    for v in map {
        if free {
            for _ in 0..v {
                disk.push(-1);
            }
        } else {
            for _ in 0..v {
                disk.push(id);
            }
            id += 1;
        }
        free = !free;
    }
    let mut freepos = 0;
    let mut usedpos = disk.len() - 1;
    loop {
        while disk[freepos] != -1 {
            freepos += 1;
        }
        while disk[usedpos] == -1 {
            usedpos -= 1;
        }
        if freepos >= usedpos {
            break;
        }
        disk.swap(freepos, usedpos);
    }
    let mut checksum = 0;
    for (i, &v) in disk.iter().enumerate() {
        if v == -1 {
            continue;
        }
        checksum += (i as i64) * v;
    }
    checksum
}

#[test_case("inputs/input-09.txt" => 6326952672104)]
#[test_case("inputs/example-09-1.txt" => 2858)]
pub fn puzzle2(filename: &str) -> i64 {
    let map = parse_input(filename);
    let mut free = false;
    let mut disk = Vec::new();
    let mut id = 0;
    for v in map {
        if free {
            for _ in 0..v {
                disk.push(-1);
            }
        } else {
            for _ in 0..v {
                disk.push(id);
            }
            id += 1;
        }
        free = !free;
    }
    let mut last_id = -1;
    let mut last_end = disk.len() - 1;
    let mut usedpos = disk.len() - 1;
    loop {
        if usedpos == 0 {
            break;
        }
        if disk[usedpos] == last_id {
            usedpos -= 1;
            continue;
        }
        if last_id != -1 {
            let size = last_end - usedpos;
            let mut freepos = 0;
            let mut freecount = 0;
            while freepos <= usedpos {
                if disk[freepos] == -1 {
                    freecount += 1;
                    freepos += 1;
                    if freecount >= size {
                        for i in 0..size {
                            disk[freepos - 1 - i] = last_id;
                        }
                        for i in 0..size {
                            disk[usedpos + 1 + i] = -1;
                        }
                        break;
                    }
                } else {
                    freecount = 0;
                    freepos += 1;
                }
            }
        }
        last_end = usedpos;
        last_id = disk[usedpos];
        usedpos -= 1;
    }
    let mut checksum = 0;
    for (pos, &id) in disk.iter().enumerate() {
        if id == -1 {
            continue;
        }
        checksum += (pos as i64) * id;
    }
    checksum
}

// 00992111777.44.333....5555.6666.....8888..
