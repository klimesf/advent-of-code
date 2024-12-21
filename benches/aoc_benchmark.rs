use criterion::{criterion_group, criterion_main, Criterion};
use adventofcode::y2024::day01::day01;
use adventofcode::y2024::day02::day02;
use adventofcode::y2024::day03::day03;
use adventofcode::y2024::day04::day04;
use adventofcode::y2024::day05::day05;
use adventofcode::y2024::day06::day06;
use adventofcode::y2024::day07::day07;
use adventofcode::y2024::day08::day08;
use adventofcode::y2024::day09::day09;
use adventofcode::y2024::day10::day10;
use adventofcode::y2024::day11::day11;
use adventofcode::y2024::day12::day12;
use adventofcode::y2024::day13::day13;
use adventofcode::y2024::day14::day14;
use adventofcode::y2024::day15::day15;
use adventofcode::y2024::day16::day16;
use adventofcode::y2024::day17::day17;
use adventofcode::y2024::day18::day18;
use adventofcode::y2024::day19::day19;
use adventofcode::y2024::day20::day20;
use adventofcode::y2024::day21::day21;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("2024 day 01", |b| b.iter(|| day01(|_| {})));
    c.bench_function("2024 day 02", |b| b.iter(|| day02(|_| {})));
    c.bench_function("2024 day 03", |b| b.iter(|| day03(|_| {})));
    c.bench_function("2024 day 04", |b| b.iter(|| day04(|_| {})));
    c.bench_function("2024 day 05", |b| b.iter(|| day05(|_| {})));
    c.bench_function("2024 day 06", |b| b.iter(|| day06(|_| {})));
    c.bench_function("2024 day 07", |b| b.iter(|| day07(|_| {})));
    c.bench_function("2024 day 08", |b| b.iter(|| day08(|_| {})));
    c.bench_function("2024 day 09", |b| b.iter(|| day09(|_| {})));
    c.bench_function("2024 day 10", |b| b.iter(|| day10(|_| {})));
    c.bench_function("2024 day 11", |b| b.iter(|| day11(|_| {})));
    c.bench_function("2024 day 12", |b| b.iter(|| day12(|_| {})));
    c.bench_function("2024 day 13", |b| b.iter(|| day13(|_| {})));
    c.bench_function("2024 day 14", |b| b.iter(|| day14(|_| {})));
    c.bench_function("2024 day 15", |b| b.iter(|| day15(|_| {})));
    c.bench_function("2024 day 16", |b| b.iter(|| day16(|_| {})));
    c.bench_function("2024 day 17", |b| b.iter(|| day17(|_| {})));
    c.bench_function("2024 day 18", |b| b.iter(|| day18(|_| {})));
    c.bench_function("2024 day 19", |b| b.iter(|| day19(|_| {})));
    c.bench_function("2024 day 20", |b| b.iter(|| day20(|_| {})));
    c.bench_function("2024 day 21", |b| b.iter(|| day21(|_| {})));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
