use itertools::Itertools;
use std::{ env, fs::File, io::{BufRead, BufReader}, collections::HashSet};

pub fn run1(reader: BufReader<File>) {
    let mut sum = 0;

    for line in reader.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();
        let conts = chars.split_at(chars.len() / 2);

        sum += conts.0.iter()
            .filter(|i| conts.1.iter().any(|j| i == &j))
            .map(|c| get_char_priority(*c))
            .sorted().dedup().sum::<i32>();
    }
    println!("{}", sum);
}

pub fn run2(reader: BufReader<File>) -> i32 {
   
    let chars: Vec<HashSet<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect::<HashSet<char>>())
        .collect();

    let mut items: Vec<char> = Vec::new();

    for chunk in chars.chunks(3)
    {   
        let mut it = chunk.iter();
        let hs1: HashSet<char> = it.next().unwrap().to_owned();
        let hs2: HashSet<char> = it.next().unwrap().to_owned();
        let hs3: HashSet<char> = it.next().unwrap().to_owned();

        let mut item = None;
        for c in hs1 {
            if hs2.contains(&c) && hs3.contains(&c) {
                item = Some(c);
                break;
            }
        }
        items.push(item.unwrap());

    }
    let sum: i32 = items.into_iter().map(get_char_priority).sum();
    sum
}

fn get_char_priority(c: char) -> i32
{
    if c.is_lowercase() { (c) as i32 - 96 } else { (c) as i32 - 38 }
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn run2_test()
    {
        let result = run2(aoc22::read_file("example", 3));
        assert_eq!(result, 70);
    }
}
