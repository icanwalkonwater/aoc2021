use aoc2021::answer;

const INPUT: &str = include_str!("../../inputs/day6.txt");

fn simulate(fishes: &[i32], days: usize) -> u64 {
    let mut fishes = {
        let mut tmp = [0; 9];
        for &f in fishes {
            tmp[f as usize] += 1;
        }
        tmp
    };


    for day in 0..days {
        let prego = fishes[0];
        for i in 1..=8 {
            fishes[i-1] = fishes[i];
        }

        fishes[6] += prego;
        fishes[8] = prego;
    }

    fishes.iter().sum()
}

fn part1(fishes: &[i32]) -> u64 {
    simulate(fishes, 80)
}

fn part2(fishes: &[i32]) -> u64 {
    simulate(fishes, 256)
}

fn main() {
    let fishes = INPUT
        .trim()
        .split(",")
        .map(|f| f.parse().unwrap())
        .collect::<Vec<i32>>();

    answer(6, part1(&fishes), part2(&fishes));
}
