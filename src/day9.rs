use std::fs;
use std::f64;

type Point = (f64, f64);

pub fn problem1(input: &str) -> i128 {
    let vertices:Vec<(u64, u64)> = input
        .lines()
        .map(|line| {
            let mut it: std::str::Split<'_, char> = line.split(',');
            (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap())
        })
        .collect();
    
    let mut biggest = 0;
    for i in 0..vertices.len() {
        for j in i+1..vertices.len() {
            let size = ((vertices[i].0).abs_diff(vertices[j].0)+1) * ((vertices[i].1).abs_diff(vertices[j].1)+1);
            biggest = if size > biggest {size} else {biggest};
        }
    }
    biggest as i128
}

fn calculate_area(p1: Point, p2: Point) -> i128 {
    let diff_x_f64 = (p1.0 - p2.0).abs() + 1.0;
    let diff_y_f64 = (p1.1 - p2.1).abs() + 1.0;
    
    // Convertir el resultado a i128 (la conversión de f64 a i128 trunca automáticamente)
    (diff_x_f64 * diff_y_f64) as i128
}

fn point_in_polygon_raycasting(mid_point: Point, edges: &[(Point, Point)]) -> bool {
    let (mid_x, mid_y) = mid_point;
    let mut intersections = 0;

    for (ep1, ep2) in edges {
        let edge_y_min = ep1.1.min(ep2.1);
        let edge_y_max = ep1.1.max(ep2.1);

        if edge_y_min < mid_y && mid_y < edge_y_max {
            let intersect_x = ep1.0 + (ep2.0 - ep1.0) * (mid_y - ep1.1) / (ep2.1 - ep1.1);
            
            if intersect_x > mid_x {
                intersections += 1;
            }
        }
    }

    intersections % 2 == 1
}

pub fn problem2(input: &str) -> i128 {
    let vertices: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            (it.next().unwrap().parse::<f64>().unwrap(), it.next().unwrap().parse::<f64>().unwrap())
        })
        .collect();

    let mut max_area = 0;
    let n = vertices.len();
    
    let mut edges = Vec::new();
    for i in 0..n {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % n];
        edges.push((p1, p2));
    }

    for i in 0..n {
        for j in i + 1..n {
            let p1 = vertices[i];
            let p2 = vertices[j];
            
            let (x1, y1) = p1;
            let (x2, y2) = p2;

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            
            let area = calculate_area(p1, p2);
            
            if area <= max_area {
                continue;
            }

            let mut is_valid = true;
            
            for v in &vertices {
                if min_x < v.0 && v.0 < max_x && min_y < v.1 && v.1 < max_y {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                continue;
            }

            for (ep1, ep2) in &edges {
                let (ex1, ey1) = ep1;
                let (ex2, ey2) = ep2;

                if ex1 == ex2 {
                    let ex = ex1;
                    let ey_min = ey1.min(*ey2);
                    let ey_max = ey1.max(*ey2);
                    
                    if min_x < *ex && *ex < max_x && ey_min <= min_y && ey_max >= max_y {
                        is_valid = false;
                        break;
                    }
                } 
                else {
                    let ey = ey1;
                    let ex_min = ex1.min(*ex2);
                    let ex_max = ex1.max(*ex2);
                    
                    if min_y < *ey && *ey < max_y && ex_min <= min_x && ex_max >= max_x {
                        is_valid = false;
                        break;
                    }
                }
            }
            if !is_valid {
                continue;
            }

            let mid_y = min_y + 0.5;
            let mid_x = min_x + 0.5;
            let mid_point = (mid_x, mid_y);

            if point_in_polygon_raycasting(mid_point, &edges) {
                max_area = area;
            }
        }
    }
    max_area
}

// fn point_in_polygon(point: (u64, u64), vertical_edges: &HashMap<u64, Vec<u64>>) -> bool {
    
//     let mut xs: Vec<_> = vertical_edges.keys().cloned().collect();
//     xs.sort_unstable();
//     // Comprobamos el numero de saltos hacia la derecha
//     let mut jumps = 0;
//     for x in xs {
//         if point.0 < x {
//             if let Some(ys) = vertical_edges.get(&x) {
//                 for i in 0..ys.len()/2 {
//                     if ys[2*i] <= point.1 && point.1 <= ys[2*i + 1] {
//                         jumps += 1;
//                     }
//                 }
//             }
//         }
//         if point.0 == x {
//             let ys = vertical_edges.get(&x).unwrap();
//             if ys[0] <= point.1 && point.1 <= ys[1] {
//                 return true;
//             }
//         }
//     }
    
//     jumps % 2 == 1  // Si los saltos son pares esta fuera, dentro si son impares
// }

// pub fn problem2(input: &str) -> i128 {
//     let vertices:Vec<(u64, u64)> = input
//         .lines()
//         .map(|line| {
//             let mut it: std::str::Split<'_, char> = line.split(',');
//             (it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap())
//         })
//         .collect();
    
//     // un borde se representa por coordenada x, rango de la y
//     let mut vertical_edges: HashMap<u64,Vec<u64>> = HashMap::new();
    
//     for vertice in vertices.as_slice() {
//         vertical_edges
//             .entry(vertice.0)
//             .or_insert_with(Vec::new)
//             .push(vertice.1);
//     }

//     for ys in vertical_edges.values_mut() {
//         ys.sort_unstable();
//     }
    

//     let mut biggest = 0;
//     for i in 0..vertices.len() {
//         for j in i+1..vertices.len() {
//             let size = ((vertices[i].0).abs_diff(vertices[j].0)+1) * ((vertices[i].1).abs_diff(vertices[j].1)+1);
//             let p1 = (vertices[i].0, vertices[j].1);
//             let p2 = (vertices[j].0, vertices[i].1);
//             biggest = if size > biggest && 
//                 (point_in_polygon(p1, &vertical_edges)) &&
//                 (point_in_polygon(p2, &vertical_edges)) {size} else {biggest};
//         }
//     }
//     biggest as i128
// }
pub fn run() {
    let input = fs::read_to_string("data/day9.txt").unwrap();
    println!("{}", problem1(&input));
    println!("{}", problem2(&input));
}