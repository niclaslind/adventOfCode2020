use std::collections::{HashMap, HashSet};
use std::fs;

fn part1() -> usize {
    fs::read_to_string("src/input/day6.txt")
        .expect("Could not found file")
        .split("\n\n")
        .map(|x| check_where_anyone_said_yes(x))
        .sum()
}

fn part2() -> usize {
    fs::read_to_string("src/input/day6.txt")
        .expect("Could not found file")
        .split("\n\n")
        .map(|x| check_where_everyone_said_yes(x))
        .sum()
}

fn check_where_anyone_said_yes(answer: &str) -> usize {
    let mut answers = HashMap::new();

    answer.trim().chars().for_each(|s| {
        if !answers.contains_key(&s) && !s.is_whitespace() {
            answers.insert(s, 1);
        }
    });

    answers.len()
}

fn check_where_everyone_said_yes(answer: &str) -> usize {
    answer
        .lines()
        .fold(('a'..='z').collect(), |group, person| {
            &group & &person.chars().collect::<HashSet<_>>()
        })
        .len()
}

#[cfg(test)]
mod tests {
    use crate::day6::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 6662)
    }
    
    #[test]
    fn test_part() {
        assert_eq!(part2(), 3382)
    }
}
