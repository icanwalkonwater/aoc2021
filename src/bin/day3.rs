use aoc2021::answer;

const INPUT: &str = include_str!("../../inputs/day3.txt");

fn count_ones_in_column(diagnostic: &[&str], position: usize) -> usize {
    diagnostic
        .iter()
        .map(|l| l.chars().nth(position).unwrap_or('0'))
        .filter(|&c| c == '1')
        .count()
}

fn part1(diagnostic: &[&str]) -> u64 {
    let width = diagnostic[0].len();

    let mut output = String::new();

    for i in 0..width {
        let ones = count_ones_in_column(diagnostic, i);
        if ones > diagnostic.len() / 2 {
            output.push('1')
        } else {
            output.push('0');
        }
    }

    let gamma_rate = u64::from_str_radix(&output, 2).unwrap();
    let epsilon_rate = u64::from_str_radix(
        &output
            .chars()
            .map(|c| match c {
                '0' => '1',
                '1' => '0',
                _ => unreachable!(),
            })
            .collect::<String>(),
        2,
    )
    .unwrap();

    gamma_rate * epsilon_rate
}

fn part2(diagnostic: &[&str]) -> u32 {
    let width = diagnostic[0].len();

    let mut oxygen_generator_candidates = diagnostic.to_vec();
    let mut co2_scrubber_candidates = diagnostic.to_vec();

    for i in 0..width {
        if oxygen_generator_candidates.len() > 1 {
            handle_list_at_position(&mut oxygen_generator_candidates, i, ['1', '0']);
        }
        if co2_scrubber_candidates.len() > 1 {
            handle_list_at_position(&mut co2_scrubber_candidates, i, ['0', '1']);
        }
    }

    assert_eq!(oxygen_generator_candidates.len(), 1);
    assert_eq!(co2_scrubber_candidates.len(), 1);

    let oxygen_rating = u32::from_str_radix(oxygen_generator_candidates[0], 2).unwrap();
    let co2_rating = u32::from_str_radix(co2_scrubber_candidates[0], 2).unwrap();

    dbg!(oxygen_rating);
    dbg!(co2_rating);

    oxygen_rating * co2_rating
}

fn handle_list_at_position(candidates: &mut Vec<&str>, position: usize, criteria: [char; 2]) {
    let ones = count_ones_in_column(&candidates, position);
    let keep = if ones * 2 >= candidates.len() {
        criteria[0]
    } else {
        criteria[1]
    };

    candidates.retain(|line| line.chars().nth(position).unwrap_or('0') == keep);
}

fn main() {
    let diagnostic = INPUT.trim().split("\n").collect::<Vec<_>>();

    answer(3, &part1(&diagnostic), &part2(&diagnostic));
}
