use aoc2020::parse;

use std::{collections::HashSet, path::Path};
use thiserror::Error;

fn calc_tuple(data: &HashSet<i32>, sum: i32) -> Option<(i32, i32)> {
    for datum in data {
        let remainder = sum - datum;

        if data.contains(&remainder) {
            return Some((remainder, *datum));
        }
    }
    None
}

fn calc_triple(data: &HashSet<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    for datum in data {
        let remainder = sum - datum;
        match calc_tuple(data, remainder) {
            Some((x, y)) => {
                if x != y && x != *datum && y != *datum {
                    return Some((x, y, *datum));
                }
            },
            None => ()
        }
    }
    None
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let data: HashSet<i32> = parse(input)?.collect();
    match calc_tuple(&data, 2020) {
        Some((x, y)) => println!("{} * {} = {}", x, y, x*y),
        None => eprintln!("No solution found!"),
    }

    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let data: HashSet<i32> = parse(input)?.collect();
    match calc_triple(&data, 2020) {
        Some((x, y, z)) => println!("{} * {} * {} = {}", x, y, z, x*y*z),
        None => eprintln!("No solution found!"),
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
