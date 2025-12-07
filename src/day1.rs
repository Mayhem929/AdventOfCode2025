use std::fs;

pub fn problem1(input: &str) -> i128 {
    let mut pos = 50;
    let mut hits = 0;

    for line in input.lines() {
        if line.is_empty() { continue; }

        let dir = line.chars().next().unwrap();
        let n: i32 = line[1..].parse().unwrap();

        let delta = if dir == 'L' { -n } else { n };
        pos = (pos + delta).rem_euclid(100);

        if pos == 0 {
            hits += 1;
        }
    }

    return hits;
}

pub fn problem2(input: &str) -> i128 {
    let mut pos = 50;
    let mut hits = 0;

    for line in input.lines() {
        if line.is_empty() { continue; }

        let dir = line.chars().next().unwrap();
        let n: i128 = line[1..].parse().unwrap();

        let delta = if dir == 'L' { -n } else { n };
        
        if pos == 0 {
            hits += (pos + delta).abs()/100;
        }
        else {
            hits += ((pos + delta).abs()/100) + if pos + delta < 0 && pos > 0 {1} else {0};
        }

        if pos + delta == 0 && (pos + delta).abs()/100 == 0 {
            hits += 1;
        }

        pos = (pos + delta).rem_euclid(100);
    }

    return hits;
}

pub fn run() {
    let input = fs::read_to_string("../data/day1.txt").unwrap();
    println!("{}", problem1(&input));
    println!("{}", problem2(&input));
}
