use aoc2021::answer;
use rayon::prelude::*;

const INPUT: &str = include_str!("../../inputs/day6.txt");

type Fish = i8;

fn simulate(fishes: &[Fish], days: usize) -> usize {
    let mut fishes = fishes.to_vec();

    for day in 0..days {
        println!("Day {}, fishes to process: {}", day, fishes.len());

        let to_spawn = fishes
            .par_chunks_mut(1_048_576)
            .map(|fishes| {
                let mut to_spawn_local = 0;
                for f in fishes {
                    *f -= 1;
                    if *f < 0 {
                        *f = 6;
                        to_spawn_local += 1;
                    }
                }
                to_spawn_local
            })
            .sum();

        fishes.reserve(to_spawn);
        fishes.extend(std::iter::repeat(8).take(to_spawn));
    }

    fishes.len()
}

fn part1(fishes: &[Fish]) -> usize {
    simulate(fishes, 80)
}

fn part2(fishes: &[Fish]) -> usize {
    simulate(fishes, 256)
}

fn main() {
    let fishes = INPUT
        .trim()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect::<Vec<Fish>>();

    // answer(6, part1(&fishes), part2(&fishes));
    answer(6, "Skip", part2(&fishes));
}
