use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_coords(input: &str) -> Vec<Coord> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut it = line.split(',');
            Coord {
                x: it.next().unwrap().parse().unwrap(),
                y: it.next().unwrap().parse().unwrap(),
                z: it.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn dist2(a: Coord, b: Coord) -> i64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dz = a.z - b.z;
    dx * dx + dy * dy + dz * dz
}

struct UF {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UF {
    fn new(n: usize) -> UF {
        UF {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            if self.size[ra] < self.size[rb] {
                self.parent[ra] = rb;
                self.size[rb] += self.size[ra];
            } else {
                self.parent[rb] = ra;
                self.size[ra] += self.size[rb];
            }
        }
    }
}

pub fn problem1(input: &str) -> i128 {
    let coords = parse_coords(input);
    let n = coords.len();

    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((dist2(coords[i], coords[j]), i, j));
        }
    }

    edges.sort_by_key(|e| e.0);

    let mut uf = UF::new(n);

    for &(_, a, b) in edges.iter().take(1000) {
        uf.union(a, b);
    }

    let mut counts = std::collections::HashMap::<usize, usize>::new();

    for i in 0..n {
        let root = uf.find(i);
        *counts.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = counts.values().copied().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    let p = sizes.iter().take(3).product::<usize>();

    p as i128
}

pub fn problem2(input: &str) -> i128 {
    let coords = parse_coords(input);
    let n = coords.len();

    // Generamos todos los edges igual que antes
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((dist2(coords[i], coords[j]), i, j));
        }
    }

    edges.sort_by_key(|e| e.0);

    let mut uf = UF::new(n);
    let mut components = n;

    for &(_, a, b) in edges.iter() {
        let ra = uf.find(a);
        let rb = uf.find(b);

        if ra != rb {
            uf.union(a, b);
            components -= 1;

            if components == 1 {
                let xa = coords[a].x as i128;
                let xb = coords[b].x as i128;
                return xa * xb;
            }
        }
    }

    0
}

pub fn run() {
    let input = fs::read_to_string("data/day8.txt").unwrap();

    println!("{}", problem1(&input));
    println!("{}", problem2(&input));
}