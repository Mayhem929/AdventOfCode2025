use std::fs;

fn problem1() {
    let input = fs::read_to_string("data/day7.txt").unwrap();
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

    println!("{}", sum);

}


fn problem2() {
    let input = fs::read_to_string("data/day7.txt").unwrap();
    let mut lines = input.lines();
    let start = lines.next().unwrap();

    let mut dp: Vec<u128> = start
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

    println!("{}", dp.iter().sum::<u128>());
}

pub fn run() {

    problem1();
    problem2();

}