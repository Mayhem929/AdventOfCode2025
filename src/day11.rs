use std::fs;
use std::collections::{VecDeque, HashMap};


fn bfs (adj_mat: &HashMap<String, Vec<String>>, start: &str, end: &str) -> i128{

    let adj_mat_clone = adj_mat.clone();
    let mut paths = 0;
    let mut frontier: VecDeque<Vec<String>> = VecDeque::new();
    frontier.push_back(vec!(start.to_string()));

    while frontier.len() > 0 {

        let next = frontier.pop_front().unwrap();
        let current_idx= &next[next.len()-1];
        if current_idx.to_string() == end {
            paths += 1;
        }

        if let Some(neighbors) = adj_mat_clone.get(current_idx) {
            for new_idx in neighbors {
                let mut new_path = next.clone();
                new_path.push(new_idx.clone());
                frontier.push_back(new_path);
            }
        }
    }
    
    paths
}

fn count_paths_recursive<'a>(
    adj_mat: &'a HashMap<String, Vec<String>>,
    current: &'a str,
    end: &'a str,
    memo: &mut HashMap<&'a str, i128>,
) -> i128 {
    
    if let Some(&count) = memo.get(current) {
        return count;
    }

    if current == end {
        memo.insert(current, 1);
        return 1;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = adj_mat.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths_recursive(adj_mat, neighbor, end, memo);
        }
    }
    
    memo.insert(current, total_paths);
    total_paths
}

fn count_paths(adj_mat: &HashMap<String, Vec<String>>, start: &str, end: &str) -> i128 {
    let mut memo: HashMap<&str, i128> = HashMap::new();
    count_paths_recursive(adj_mat, start, end, &mut memo)
}

pub fn problem1(input: &str) -> i128 {

    let mut adj_mat: HashMap<String, Vec<String>>= HashMap::new();
    
    for line in input.lines() {
        let devices: Vec<&str> = line.split_whitespace().collect();
        let idx = devices[0].trim_matches(':').to_string();
        let entry = adj_mat.entry(idx).or_insert(Vec::new());
        
        for device in devices[1..].iter() {
            entry.push(device.to_string());
        }

    }

    bfs(&adj_mat, "you", "out")

}


pub fn problem2(input: &str) -> i128 {
    let mut adj_mat: HashMap<String, Vec<String>>= HashMap::new();
    
    for line in input.lines() {
        let devices: Vec<&str> = line.split_whitespace().collect();
        let idx = devices[0].trim_matches(':').to_string();
        let entry = adj_mat.entry(idx).or_insert(Vec::new());
        
        for device in devices[1..].iter() {
            entry.push(device.to_string());
        }

    }

    let mut total = 0;

    total += count_paths(&adj_mat, "svr", "fft") * count_paths(&adj_mat, "fft", "dac") * count_paths(&adj_mat, "dac", "out");
    total += count_paths(&adj_mat, "svr", "dac") * count_paths(&adj_mat, "dac", "fft") * count_paths(&adj_mat, "fft", "out");


    total
}


pub fn run() {
    let input = fs::read_to_string("data/day11.txt").unwrap();

    println!("{}", problem1(&input));
    println!("{}", problem2(&input));
}