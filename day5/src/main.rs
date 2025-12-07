use std::fs;
use std::collections::{HashMap, HashSet};

fn problem1() {
    let input = fs::read_to_string("data/day5.txt").unwrap();

    let mut ranges: Vec<(u128, u128)> = Vec::new();
    let mut ids: Vec<u128> = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        if line.contains('-') {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse::<u128>().unwrap();
            let end = parts.next().unwrap().parse::<u128>().unwrap();
            ranges.push((start, end));
        } else {
            ids.push(line.parse::<u128>().unwrap());
        }
    }
    
    let mut fresh = 0;

    let ranges_slice = ranges.as_slice();

    for id in ids {
        for range in ranges_slice {
            if id >= range.0 && id <= range.1 {
                fresh += 1;
                break;
            }
        }
    }

    println!("{fresh}");
}


fn problem2() {
    let input = fs::read_to_string("data/day5.txt").unwrap();

    let mut ranges: Vec<(u128,u128)> = Vec::new();

    for line in input.lines() {
        if line.contains('-') {
            let mut parts = line.split('-');
            let start: u128 = parts.next().unwrap().parse().unwrap();
            let end: u128 = parts.next().unwrap().parse().unwrap();
            ranges.push((start, end));
        }
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(u128,u128)> = Vec::new();

    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                if end > last.1 {
                    last.1 = end;
                }
            } else {
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    let total: u128 = merged.iter().map(|(s,e)| e - s + 1).sum();

    println!("{total}");
}

fn main() {

    problem1();
    problem2();

}