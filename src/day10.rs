use std::fs;
use good_lp::*;

pub fn problem1(input: &str) -> i128 {

    let mut total: i128 = 0;

    for line in input.lines() {

        let mut buttons: Vec<Vec<u64>> = Vec::new();
        let mut parts: Vec<&str> = line.split_whitespace().collect();

        let joltages_str: String = parts[0]
            .trim_matches(&['[', ']'][..])
            .chars()
            .rev()          // Tenemos que darle la vuelta a los joltajes porque los indices de nmumeros se leen de der a izq
            .map(|c| if c == '.' { '0' } else { '1' })
            .collect();
        let joltages = u64::from_str_radix(&joltages_str, 2).unwrap();

        parts.pop();

        for lights in &parts[1..] {
            let cleaned = lights.trim_matches(&['(', ')'][..]);
            let new_button: Vec<u64> = cleaned
                .split(',')
                .map(|v| 1 << v.parse::<u64>().unwrap())    // pasamos de cada num a que represente la posicion que dice en binario
                .collect();
            buttons.push(new_button);
        }

        let n = buttons.len();
        let mut min = n;

        let comb = 1usize << n; // Todas las combinaciones de indices representadas eb binario

        for mask in 0..comb {
            let mut current_jolt = joltages;

            for i in 0..n {
                if (mask & (1 << i)) != 0 {  // Si el indice ha sido cogido (0001001) -> indices 0 y 3
                    for j in 0..buttons[i].len() {
                        current_jolt ^= buttons[i][j]; // hacemos un XOR con cada boton elegido, si al final llegamos a 0 la combinacion es correcta.
                    }
                }
            }
            if current_jolt == 0 {
                let num = mask.count_ones() as usize;
                if num < min {min = num;}
            }
        }

        total += min as i128;
    }


    return total;

}

pub fn problem2(input: &str) -> i128 {
    use good_lp::*;

    let mut total: i128 = 0;

    for line in input.lines() {
        if line.trim().is_empty() { continue; }

        let mut parts: Vec<&str> = line.split_whitespace().collect();

        let req_str = parts.pop().unwrap();
        let req_clean = req_str.trim_matches(&['{', '}'][..]);
        let target: Vec<i64> = req_clean
            .split(',')
            .map(|v| v.parse::<i64>().unwrap())
            .collect();
        let d = target.len();

        let mut buttons: Vec<Vec<usize>> = Vec::new();

        for token in &parts[1..] {
            let cleaned = token.trim_matches(&['(', ')'][..]);
            if cleaned.is_empty() { continue; }

            let button: Vec<usize> = cleaned
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            buttons.push(button);
        }

        let m = buttons.len();

        let mut vars = variables!();

        let x: Vec<Variable> = vars.add_vector(variable().integer().min(0), m);

        let objective: Expression = x.iter().sum();
        let mut pb = vars.minimise(objective).using(default_solver);

        for j in 0..d {
            let expr: Expression = x.iter()
                .enumerate()
                .map(|(i, xi)| {
                    if buttons[i].contains(&j) {
                        (*xi).into()
                    } else {
                        Expression::from(0)
                    }
                })
                .sum();
            pb = pb.with(expr.eq(target[j] as i32));
        }

        let sol = pb.solve().expect("ILP solve failed");

        let presses: i128 = x.iter()
            .map(|v| sol.value(*v).round() as i128)
            .sum();

        total += presses;
    }

    total
}


pub fn run() {
    let input = fs::read_to_string("data/day10.txt").unwrap();

    println!("{}", problem1(&input));
    println!("{}", problem2(&input));
}