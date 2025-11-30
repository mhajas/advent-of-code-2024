use std::collections::{HashMap, HashSet};
use gcd::Gcd;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day8")
        .expect("The input file should be read.");

    let mut map_original = Vec::<String>::new();
    for (_y, line_orig) in input_file.lines().into_iter().enumerate() {
        let line = line_orig.trim().to_string();
        map_original.push(line);
    }
    let dimensions = (map_original[0].len(), map_original.len());
    let mut antennas = HashMap::<char, Vec<(usize, usize)>>::new();

    for (i, s) in map_original.iter().enumerate() {
        for (j, c) in s.chars().enumerate() {
            if c == '.' {
                continue;
            }

            if !antennas.contains_key(&c) {
                antennas.insert(c, Vec::<(usize, usize)>::new());
            }

            antennas.get_mut(&c).unwrap().push((i, j));
        }
    }

    println!("{:?}", antennas);

    let mut antinodes = HashSet::<(usize, usize)>::new();

    for freq in antennas.keys() {
        let locations = antennas.get(freq).unwrap();
        for (i, antenna1) in locations.iter().enumerate() {
            if i == locations.len() - 1 {continue}
            for antenna2 in locations.as_slice()[i + 1..].iter() {
                let lcs = antinodes_locations(antenna1, antenna2, &dimensions);
                println!("{:?}", lcs);

                antinodes.extend(&lcs)
            }
        }
    }

    println!("{:?}", antinodes);

    let mut map1 = map_original.clone();

    for (x, y) in &antinodes {
        unsafe { *map1[*x].as_mut_ptr().add(*y) = '#' as u8; }
    }


    for l in map1 {
        println!("{}", l);
    }

    println!("Result: {:?}", antinodes.len());


    antinodes.clear();
    for freq in antennas.keys() {
        let locations = antennas.get(freq).unwrap();
        for (i, antenna1) in locations.iter().enumerate() {
            if i == locations.len() - 1 {continue}
            for antenna2 in locations.as_slice()[i + 1..].iter() {
                let lcs = antinodes_locations_part2(antenna1, antenna2, &dimensions);
                println!("{:?}", lcs);

                antinodes.extend(&lcs)
            }
        }
    }

    let mut map1 = map_original.clone();

    for (x, y) in &antinodes {
        unsafe { *map1[*x].as_mut_ptr().add(*y) = '#' as u8; }
    }


    for l in map1 {
        println!("{}", l);
    }

    println!("Result2: {:?}", antinodes.len());
}

fn antinodes_locations(antenna1: &(usize, usize), antenna2: &(usize, usize), dimension: &(usize, usize)) -> Vec<(usize, usize)> {
    let vec = (antenna1.0 as i32 - antenna2.0 as i32, antenna1.1 as i32 - antenna2.1 as i32);

    let mut resulting_vector = Vec::<(i32, i32)>::new();
    resulting_vector.append(&mut antinodes_locations_for_antenna(antenna1, &vec));
    resulting_vector.append(&mut antinodes_locations_for_antenna(antenna2, &vec));

    resulting_vector.iter()
        .filter(|(x, y)| *x >= 0 && *x < dimension.0 as i32 && *y >= 0 && *y < dimension.1 as i32 )
        .map(|(x, y)| (*x as usize, *y as usize))
        .filter(|loc| *loc != *antenna1 && *loc != *antenna2)
        .collect()
}

fn antinodes_locations_for_antenna(antenna: &(usize, usize), vector: &(i32, i32)) -> Vec<(i32, i32)> {
    vec![(antenna.0 as i32 + vector.0, antenna.1 as i32 + vector.1), (antenna.0 as i32 - vector.0, antenna.1 as i32 - vector.1)]
}

fn antinodes_locations_part2(antenna1: &(usize, usize), antenna2: &(usize, usize), dimension: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut vec = (antenna1.0 as i32 - antenna2.0 as i32, antenna1.1 as i32 - antenna2.1 as i32);
    let greatest_common_divisor = (vec.0.abs() as u32) .gcd(vec.1 as u32) as i32;
    vec = (vec.0 / greatest_common_divisor, vec.1 / greatest_common_divisor);

    let mut resulting_vector = Vec::<(usize, usize)>::new();
    resulting_vector.push(*antenna1);

    let mut previous = (antenna1.0 as i32, antenna1.1 as i32);
    while let Some(x) = in_bound(&(previous.0 + vec.0, previous.1 + vec.1), dimension) {
        resulting_vector.push(x);
        previous = (x.0 as i32, x.1 as i32);
    }

    previous = (antenna1.0 as i32, antenna1.1 as i32);
    while let Some(x) = in_bound(&(previous.0 - vec.0, previous.1 - vec.1), dimension) {
        resulting_vector.push(x);
        previous = (x.0 as i32, x.1 as i32);
    }

    println!("{:?}", resulting_vector);
    resulting_vector
}

fn in_bound(in_question: &(i32, i32), (dx, dy): &(usize, usize)) -> Option<(usize, usize)> {
    match in_question {
        (x,y) if *x >= 0 && *x < *dx as i32 && *y >= 0 && *y < *dy as i32 => Some((*x as usize, *y as usize)),
        _ => None,
    }
}

