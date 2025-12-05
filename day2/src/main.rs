use std::fs;

fn problem1() {
    let input = fs::read_to_string("data/day2.txt").unwrap();
    let ranges: Vec<&str> = input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.trim())
        .collect();

    let mut total: u128 = 0;

    for range in ranges {
        let (l_bound, u_bound) = range.split_once('-').unwrap();
        let l_bound_int: u128 = l_bound.parse().unwrap();
        let u_bound_int: u128 = u_bound.parse().unwrap();
        
        let lower_half = l_bound.len() / 2;
        let mut start: u128 = if lower_half == 0 {l_bound[..1].parse().unwrap()} else {l_bound[..lower_half].parse().unwrap()};

        let upper_half = u_bound.len() / 2 + if u_bound.len() % 2 == 1 {1} else {0};
        let end: u128 = if upper_half == 0 {u_bound[..1].parse().unwrap()} else {u_bound[..upper_half].parse().unwrap()};


        while start <= end {
            let doubled: u128 = format!("{}{}", start, start).parse().unwrap();
            if doubled >= l_bound_int && doubled <= u_bound_int {
                total += doubled;
            }
            start += 1;
        }
    }
    println!("{}", total);
}


fn is_repeated_pattern(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Un patrón válido debe repetirse al menos 2 veces
    for pat_len in 1..=(len / 2) {
        if len % pat_len != 0 {
            continue;
        }
        let pattern = &s[..pat_len];
        let mut repeated = String::new();
        for _ in 0..(len / pat_len) {
            repeated.push_str(pattern);
        }
        if repeated == s {
            return true;
        }
    }
    false
}


fn problem2() {
    let input = fs::read_to_string("data/day2.txt").unwrap();

    // Limpia y separa los rangos
    let ranges: Vec<&str> = input
        .lines()
        .flat_map(|line| line.split(','))
        .map(|s| s.trim())
        .collect();

    let mut total: u128 = 0;

    for range in ranges {
        let (l_bound, u_bound) = range.split_once('-').unwrap();
        let l_bound: u128 = l_bound.parse().unwrap();
        let u_bound: u128 = u_bound.parse().unwrap();

        for n in l_bound..=u_bound {
            if is_repeated_pattern(n) {
                total += n;
            }
        }
    }

    println!("{}", total);
}

fn main() {
    
    problem1();

    problem2();
    
}