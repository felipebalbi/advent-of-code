use day12::{part1, part2};

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_benchmark() {
    part1(divan::black_box(include_str!("../input.txt",))).unwrap();
}

#[divan::bench]
fn part2_benchmark() {
    part2(divan::black_box(include_str!("../input.txt",))).unwrap();
}
