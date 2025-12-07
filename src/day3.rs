use std::fs;

fn problem1() {
    let input = fs::read_to_string("data/day3.txt").unwrap();
    
    let mut total: i64 = 0;

    for line in input.lines(){

        let first = line[..line.len()-1]
            .chars()
            .max()
            .unwrap();

        let second = line[(line.find(first).unwrap()+1)..]
            .chars()
            .max()
            .unwrap();

        total += format!("{}{}", first, second).parse::<i64>().unwrap();
        
        println!("{}{}", first, second);
    }

    println!("{}", total);
}

fn problem2(num: usize) {
    let input = fs::read_to_string("data/day3.txt").unwrap();
    
    let mut total: i64 = 0;

    for line in input.lines(){
        
        let mut left = num;
        let mut joltages_left = line;
        let mut batteries = String::new();

        while left > 0 {
            if joltages_left.len() == left {
                batteries.push_str(joltages_left);
                left = 0;
            }
            else {
                let to_add: char = joltages_left[..joltages_left.len()-left+1]
                    .chars()
                    .max()
                    .unwrap();

                batteries.push(to_add);
                left -= 1;
                joltages_left = &(joltages_left.split_at(joltages_left.find(to_add).unwrap()+1).1);
            }
        }
        
        total += batteries.parse::<i64>().unwrap();
    }

    println!("{}", total);
}

pub fn run() {
    
    problem1();

    problem2(12);
    
}