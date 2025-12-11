use criterion::{criterion_group, criterion_main, Criterion};
use aoc2025::{
    day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11
};

pub fn bench_day1(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day1.txt").unwrap();
    c.bench_function("day1_problem1", |b| b.iter(|| day1::problem1(&input)));
    c.bench_function("day1_problem2", |b| b.iter(|| day1::problem2(&input)));
}

pub fn bench_day2(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day2.txt").unwrap();
    c.bench_function("day2_problem1", |b| b.iter(|| day2::problem1(&input)));
    c.bench_function("day2_problem2", |b| b.iter(|| day2::problem2(&input)));
}

pub fn bench_day3(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day3.txt").unwrap();
    c.bench_function("day3_problem1", |b| b.iter(|| day3::problem1(&input)));
    c.bench_function("day3_problem2", |b| b.iter(|| day3::problem2(&input)));
}

pub fn bench_day4(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day4.txt").unwrap();
    c.bench_function("day4_problem1", |b| b.iter(|| day4::problem1(&input)));
    c.bench_function("day4_problem2", |b| b.iter(|| day4::problem2(&input)));
}

pub fn bench_day5(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();
    c.bench_function("day5_problem1", |b| b.iter(|| day5::problem1(&input)));
    c.bench_function("day5_problem2", |b| b.iter(|| day5::problem2(&input)));
}

pub fn bench_day6(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day6.txt").unwrap();
    c.bench_function("day6_problem1", |b| b.iter(|| day6::problem1(&input)));
    c.bench_function("day6_problem2", |b| b.iter(|| day6::problem2(&input)));
}

pub fn bench_day7(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day7.txt").unwrap();
    c.bench_function("day7_problem1", |b| b.iter(|| day7::problem1(&input)));
    c.bench_function("day7_problem2", |b| b.iter(|| day7::problem2(&input)));
}

pub fn bench_day8(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();
    c.bench_function("day8_problem1", |b| b.iter(|| day8::problem1(&input)));
    c.bench_function("day8_problem2", |b| b.iter(|| day8::problem2(&input)));
}

pub fn bench_day9(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day9.txt").unwrap();
    c.bench_function("day9_problem1", |b| b.iter(|| day9::problem1(&input)));
    c.bench_function("day9_problem2", |b| b.iter(|| day9::problem2(&input)));
}

pub fn bench_day10(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day10.txt").unwrap();
    c.bench_function("day10_problem1", |b| b.iter(|| day10::problem1(&input)));
    c.bench_function("day10_problem2", |b| b.iter(|| day10::problem2(&input)));
}

pub fn bench_day11(c: &mut Criterion) {
    let input = std::fs::read_to_string("data/day11.txt").unwrap();
    c.bench_function("day11_problem1", |b| b.iter(|| day11::problem1(&input)));
    c.bench_function("day11_problem2", |b| b.iter(|| day11::problem2(&input)));
}

criterion_group!(
    benches,
    // bench_day1,
    // bench_day2,
    // bench_day3,
    // bench_day4,
    // bench_day5,
    // bench_day6,
    // bench_day7,
    // bench_day8,
    // bench_day9,
    // bench_day10,
    bench_day11,
);
criterion_main!(benches);
