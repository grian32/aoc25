use aoc25::days::day::Day;
use criterion::{criterion_group, criterion_main, Criterion};
use aoc25::days::day1::Day1;
use aoc25::days::day2::Day2;
use aoc25::days::day3::Day3;
use aoc25::days::day4::Day4;
use aoc25::days::day5::Day5;

fn bench_day1(c: &mut Criterion) {
    let day = Day1{};
    let input = day.get_input();
    c.bench_function("day1 part1", |b| {
        b.iter(|| {
            day.part1(&input);
        })
    });
    c.bench_function("day1 part2", |b| {
        b.iter(|| {
            day.part2(&input);
        })
    });
}

fn bench_day2(c: &mut Criterion) {
    let day = Day2{};
    let input = day.get_input();
    c.bench_function("day2 part1", |b| {
        b.iter(|| {
            day.part1(&input);
        })
    });
    c.bench_function("day2 part2", |b| {
        b.iter(|| {
            day.part2(&input);
        })
    });
}

fn bench_day3(c: &mut Criterion) {
    let day = Day3{};
    let input = day.get_input();
    c.bench_function("day3 part1", |b| {
        b.iter(|| {
            day.part1(&input);
        })
    });
    c.bench_function("day3 part2", |b| {
        b.iter(|| {
            day.part2(&input);
        })
    });
}

fn bench_day4(c: &mut Criterion) {
    let day = Day4{};
    let input = day.get_input();
    c.bench_function("day4 part1", |b| {
        b.iter(|| {
            day.part1(&input);
        })
    });
    c.bench_function("day4 part2", |b| {
        b.iter(|| {
            day.part2(&input);
        })
    });
}

fn bench_day5(c: &mut Criterion) {
    let day = Day5{};
    let input = day.get_input();
    c.bench_function("day5 part1", |b| {
        b.iter(|| {
            day.part1(&input);
        })
    });
    c.bench_function("day5 part2", |b| {
        b.iter(|| {
            day.part2(&input);
        })
    });
}
criterion_group!(benches, bench_day5);
criterion_main!(benches);