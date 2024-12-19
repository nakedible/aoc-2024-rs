use std::collections::HashMap;

use test_case::test_case;
use trie_rs::Trie;

fn parse_input(filename: &str) -> (Vec<String>, Vec<String>) {
    let input = std::fs::read_to_string(filename).unwrap();
    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels = towels.trim().split(", ").map(str::to_string).collect();
    let designs = designs.lines().map(str::to_string).collect();
    (towels, designs)
}

fn can_match(towels: &Trie<u8>, design: &[u8]) -> bool {
    if design.is_empty() {
        return true;
    }
    let mut inc_search = towels.inc_search();
    for i in 0..design.len() {
        if let Some(next) = inc_search.query(&design[i]) {
            if next.is_match() {
                if can_match(towels, &design[i + 1..]) {
                    return true;
                }
            }
        } else {
            break;
        }
    }
    false
}

#[test_case("inputs/input-19.txt" => 240)]
#[test_case("inputs/example-19-1.txt" => 6)]
pub fn puzzle1(filename: &str) -> i64 {
    let (towels, designs) = parse_input(filename);
    let trie = Trie::from_iter(towels);
    let mut ret = 0;
    for design in designs {
        if can_match(&trie, design.as_bytes()) {
            ret += 1;
        }
    }
    ret
}

fn count_match(towels: &Trie<u8>, design: &[u8], cache: &mut HashMap<Vec<u8>, i64>) -> i64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&ret) = cache.get(design) {
        return ret;
    }
    let mut ret = 0;
    let mut inc_search = towels.inc_search();
    for i in 0..design.len() {
        if let Some(next) = inc_search.query(&design[i]) {
            if next.is_match() {
                ret += count_match(towels, &design[i + 1..], cache);
            }
        } else {
            break;
        }
    }
    cache.insert(design.to_vec(), ret);
    ret
}

#[test_case("inputs/input-19.txt" => 848076019766013)]
#[test_case("inputs/example-19-1.txt" => 16)]
pub fn puzzle2(filename: &str) -> i64 {
    let (towels, designs) = parse_input(filename);
    let trie = Trie::from_iter(towels);
    let mut ret = 0;
    let mut cache = HashMap::new();
    for design in designs {
        ret += count_match(&trie, design.as_bytes(), &mut cache);
    }
    ret
}
