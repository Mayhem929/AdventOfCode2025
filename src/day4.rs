use std::fs;

pub fn problem1(input: &str) -> i128 {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| if c == '@' {1} else {0}).collect())
        .collect();

    let h = map.len();
    let w = map[0].len();

    let dirs = [
        (-1,-1), (-1,0), (-1,1),
        (0,-1),         (0,1),
        (1,-1),  (1,0), (1,1),
    ];

    let mut rolls = 0;

    for y in 0..h {
        for x in 0..w {
            let mut cnt = 0;
            for (dy, dx) in dirs {
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                if ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize {
                    cnt += map[ny as usize][nx as usize];
                }
            }
            rolls += if map[y][x] == 1 && cnt < 4 {1} else {0} ;
        }
    }

    return rolls;

}

pub fn problem2(input: &str) -> i128 {
    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| if c == '@' {1} else {0}).collect())
        .collect();

    let h = map.len();
    let w = map[0].len();

    let dirs = [
        (-1,-1), (-1,0), (-1,1),
        (0,-1),         (0,1),
        (1,-1),  (1,0), (1,1),
    ];

    let mut total = 0;
    let mut go_on = true;

    while go_on {
        let mut rolls = 0;
        let mut to_remove: Vec<(usize, usize)> = Vec::new();
        
        for y in 0..h {
            for x in 0..w {
                let mut cnt = 0;
                for (dy, dx) in dirs {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;
                    if ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize {
                        cnt += map[ny as usize][nx as usize];
                    }
                }
                if map[y][x] == 1 && cnt < 4 {
                    rolls += 1;
                    to_remove.push((y,x));
                }
            }
        }

        total += rolls;
        
        if to_remove.len() == 0 {go_on = false};

        for (y,x) in to_remove {
            map[y][x] = 0;
        }
    }

    return total;
}

pub fn run() {
    let input = fs::read_to_string("data/day4.txt").unwrap();

    println!("{}", problem1(&input));
    println!("{}", problem2(&input));

}