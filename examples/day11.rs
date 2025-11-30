#![feature(f128)]

use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day11")
        .expect("The input file should be read.");


    let input = input_file.lines().find_or_first(|_|true);

    let input = input.unwrap()
        .split(" ")
        .into_iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let iterations = 35;


    let cache = HashMap::<(u64, u32), u128>::new();
    let mut start = Instant::now();
    //println!("Result1 {:?}", input.iter().map(|n| recursion(&mut cache, *n, 0, iterations)).fold(0, |a, b| a+b));
    println!("in {:?}, cache size: {}", Instant::now().duration_since(start).as_millis(), cache.len());

    let mut cache_log10 = HashMap::<u64, u32>::new();
    let mut stones = input.clone();
    start = Instant::now();
    for _ in 0..iterations {
        for i in 0..stones.len() {
            let mut add = 0_u64;
            match stones.get_mut(i).unwrap() {
                x if *x == 0 => *x = 1,
                x => {
                    // This is not nice as u64 may not fit into f64, still it works with the day11 input
                    let digits = digits(&mut cache_log10, *x);
                    if digits % 2 == 0 {
                        let halver = 10_u64.pow(digits / 2);
                        add = *x / halver;
                        *x %= halver;
                    } else {
                        *x *= 2024;
                    }
                }
            }
            if add != 0_u64 {
                stones.push(add);
            }
        }
    }

    println!("Result {:?}", stones.len());
    println!("in {:?}, cache size: {}", Instant::now().duration_since(start).as_millis(), cache_log10.len());
}


#[allow(dead_code)]
fn recursion(cache:&mut HashMap<(u64, u32), u128>, n: u64, i: u32, s: u32) -> u128 {
    if i >= s {
        return 1;
    }

    let key = (n, s - i);
    if let Some(i) = cache.get(&key) {
        return *i;
    }

    let result = match n {
        0 => recursion(cache, 1, i + 1, s),
        x => {
            // This is not nice as u64 may not fit into f64, still it works with the day11 input
            let digits = ((x as f64).log10().floor() as u64 + 1) as u32;
            if digits % 2 == 0 {
                let halver = 10_u64.pow(digits / 2);
                recursion(cache, x / halver, i + 1, s) + recursion(cache, x % halver, i + 1, s)
            } else {
                recursion(cache, x * 2024, i + 1, s)
            }
        }
    };

    cache.insert(key, result);
    result
}

#[allow(dead_code)]
fn cached_process_number(cache: &mut HashMap<u64, Vec<u64>>, n: &u64) -> Vec<u64> {
    cache.entry(*n).or_insert_with_key(process_number).to_owned()
}

fn process_number(n: &u64) -> Vec<u64> {
    match n {
        0 => vec![1],
        x => {
            let digits = ((*x as f64).log10().floor() as u64 + 1) as u32;
            if digits % 2 == 0 {
                let halver = 10_u64.pow(digits / 2);
                vec![*x / halver, *x % halver]
            } else {
                vec![*x * 2024]
            }
        }
    }
}

fn digits(cache: &mut HashMap<u64, u32>, n: u64) -> u32 {
    if let Some(x) = cache.get(&n) {
        *x
    } else {
        let digits = ((n as f64).log10().floor() as u64 + 1) as u32;
        cache.insert(n, digits);
        digits
    }
}