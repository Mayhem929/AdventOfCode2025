use std::fs;

fn problem1() {
    let input = fs::read_to_string("data/day6.txt").unwrap();

    let mut numbers = input.lines().collect::<Vec<&str>>();
    let mut operations = numbers.split_off(numbers.len()-1);
    operations = operations[0].split_whitespace().collect();
    let mat: Vec<_> = numbers.iter().map(|row| row.split_whitespace().collect::<Vec<_>>()).collect();
    
    let mut total = 0;
    for i in 0..operations.len() {
        
        if operations[i] == "*" {
            let mut to_add = 1;
            for j in 0..mat.len() {
                to_add *= mat[j][i].parse::<i64>().unwrap();
            }
            total += to_add;
        }
        else {
            let mut to_add = 0;
            for j in 0..mat.len() {
                to_add += mat[j][i].parse::<i64>().unwrap();
            }
            total += to_add;
        }
    }

    println!("{:?}", total);

}


fn problem2() {
    let input = fs::read_to_string("data/day6.txt").unwrap();

    let mut numbers = input.lines().collect::<Vec<&str>>();
    let mut operations = numbers.split_off(numbers.len()-1);
    operations = operations[0].split_whitespace().collect();
    let mat = numbers.iter().map(|row: &&str| row.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let mut op = 0;
    let mut total = 0;
    let mut to_add = if operations[0] == "*" {1} else {0};
    
    for i in 0..mat[0].len() {
        let mut num_str = String::new();

        for j in 0..mat.len() {
            if mat[j][i].is_numeric() {
                num_str.push(mat[j][i]);
            }
        }

        if let Ok(num) = num_str.parse::<i64>() {
            if operations[op] == "*" {
                to_add *= num;
            }
            else {
                to_add += num;
            }
        }
        else {
            total += to_add;
            op += 1;
            to_add = if operations[op] == "*" {1} else {0};
        }
    }

    total += to_add;

    println!("{:?}", total);
}

fn main() {

    problem1();
    problem2();

}