use std::fs;

pub fn problem1(input: &str) -> i128 {
    let mut ranges: Vec<(i128, i128)> = Vec::new();
    let mut ids: Vec<i128> = Vec::new();

    for line in input.lines().filter(|l| !l.is_empty()) {
        if line.contains('-') {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse::<i128>().unwrap();
            let end = parts.next().unwrap().parse::<i128>().unwrap();
            ranges.push((start, end));
        } else {
            ids.push(line.parse::<i128>().unwrap());
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

    return fresh;
}


pub fn problem2(input: &str) -> i128 {
    let mut ranges: Vec<(i128,i128)> = Vec::new();

    for line in input.lines() {
        if line.contains('-') {
            let mut parts = line.split('-');
            let start: i128 = parts.next().unwrap().parse().unwrap();
            let end: i128 = parts.next().unwrap().parse().unwrap();
            ranges.push((start, end));
        }
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(i128,i128)> = Vec::new();

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

    let total: i128 = merged.iter().map(|(s,e)| e - s + 1).sum();

    return total;
}

pub fn run() {
    let input = fs::read_to_string("data/day5.txt").unwrap();

    println!("{}", problem1(&input));
    println!("{}", problem2(&input));

}