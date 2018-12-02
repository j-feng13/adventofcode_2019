use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let q1 = q_1();
    let q2 = q_2();

    println!("q1 result: {}, qr2 result: {}", q1, q2);
}

fn q_1() -> i32 {
    let mut input = File::open("day1_input.txt").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    let waves: Vec<&str> = contents.lines().collect();
    let mut counter = 0;
    for wave in waves {
        let valence = &wave[1..].parse::<i32>().unwrap();
        match &wave[0..1] {
            "+" => counter += valence,
            "-" => counter -= valence,
            _ => {}
        }
    }
    return counter;
}

fn q_2() -> i32 {
    let mut input = File::open("day1_input.txt").unwrap();
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    let waves: Vec<&str> = contents.lines().collect();
    let mut counter = 0;
    let mut i = 0;
    let mut counters = HashSet::new();
    loop {
        let wave = waves[i];
        let valence = &wave[1..].parse::<i32>().unwrap();
        match &wave[0..1] {
            "+" => counter += valence,
            "-" => counter -= valence,
            _ => {}
        }

        if counters.contains(&counter) {
            return counter;
        }

        counters.insert(counter);

        if i == waves.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
    }
}
