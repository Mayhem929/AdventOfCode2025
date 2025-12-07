use std::fs;

pub fn problem1(input: &str) -> i128 {
    let mut lines = input.lines();
    let start = lines.next();

    let mut tachyons = start.unwrap().chars().map(|c| if c == '.' {0} else {1}).collect::<Vec<_>>();
    let mut sum = 0;

    let mut line_opt = lines.next();
    while line_opt != None {
        let line = line_opt.unwrap().chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < line.len() {
            if line[i] == '^' && tachyons[i] == 1 {
                tachyons[i-1] = 1;
                tachyons[i] = 0;
                tachyons[i+1] = 1;
                i += 1;
                sum += 1
            }
            i += 1;
        }

        line_opt = lines.next();
    }

    sum

}


pub fn problem2(input: &str) -> i128 {
    let mut lines = input.lines();
    let start = lines.next().unwrap();

    let mut dp: Vec<i128> = start
        .chars()
        .map(|c| if c == '.' {0} else {1})
        .collect(); // cada celda representa el numero de formas de llegar a esa casilla

    while let Some(line) = lines.next() {
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '^' && dp[i] > 0 {
                dp[i-1] += dp[i];
                dp[i+1] += dp[i];
                dp[i] = 0;
                i += 1;
            }
            i += 1;
        }
    }

    dp.iter().sum::<i128>()
}

pub fn run() {
    let input = fs::read_to_string("data/day7.txt").unwrap();

    println!("{}", problem1(&input));
    println!("{}", problem2(&input));

}