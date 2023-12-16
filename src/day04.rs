
const INPUT : &'static str = include_str!("../data/day04.txt");
type Data = Vec<Card>;
use std::collections::{HashSet, hash_map::RandomState, VecDeque};

use itertools::Itertools;
struct Card {
    id: usize,
    winning_numbers : Vec<usize>,
    actual_numbers : Vec<usize>,
}

fn parse(input : &str) -> Data {
    input.trim().lines()
    .map(|line| {
        let (id,numbers) = line.split_once(':').unwrap();
        let id = id[5..].trim().parse().unwrap();
        let (winning_numbers,actual_numbers) = numbers.trim().split_once(" | ").unwrap();
        let winning_numbers = winning_numbers.trim().split_whitespace().map(str::parse).map(Result::unwrap).collect();
        let actual_numbers = actual_numbers.trim().split_whitespace().map(str::parse).map(Result::unwrap).collect();

        Card {
            id,
            winning_numbers,
            actual_numbers,
        }
    })
    .collect()
}

fn part1_impl(data:Data) -> usize {
    data.into_iter()
    .map(|card| {
        let matches = HashSet::<usize>::from_iter(card.actual_numbers)
        .intersection(&HashSet::from_iter(card.winning_numbers))
        .count();
        
        if matches != 0 {
            2.0f32.powi((matches - 1) as i32) as usize
        } else {
            0
        }
    }).sum()
}

fn apply_n_times(tracking:&mut VecDeque<usize>, k:usize, n:usize) {
    if n >= tracking.len() {
        for item in tracking.iter_mut() {
            *item += k;
        }
        tracking.extend(vec![k;n-tracking.len()]);
    } else {
        for item in tracking.iter_mut().take(n) {
            *item += k;
        }
    }
}
fn part2_impl(data:Data) -> usize {
    let mut copies = VecDeque::with_capacity(data.capacity());
    data.into_iter()
    .fold(0, |accum,card|{
        let matches = HashSet::<usize>::from_iter(card.actual_numbers)
        .intersection(&HashSet::from_iter(card.winning_numbers))
        .count();
        let copies_self = copies.pop_front().map_or(1, |copies| copies + 1);
        apply_n_times(&mut copies, copies_self, matches);
        accum + copies_self
    })
}

pub fn part1() {
    let data = parse(INPUT);
    println!("total number of points is {}", part1_impl(data));
}

pub fn part2() {
    let data = parse(INPUT);
    println!("total number of cards is {}", part2_impl(data));
}


#[cfg(test)]
mod tests {
    const SAMPLE :&'static str = r"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1() {
        let data = super::parse(SAMPLE);
        assert_eq!(super::part1_impl(data),13)
    }

    #[test]
    fn part2() {
        let data = super::parse(SAMPLE);
        assert_eq!(super::part2_impl(data),30)
    }
}