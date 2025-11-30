use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day1_input_simple")
        .expect("The input file should be read.");

    let mut location_list1: Vec<i32> = Vec::new();
    let mut location_list2: Vec<i32> = Vec::new();

    for line in input_file.lines() {
        let inputs = line.split_whitespace().collect::<Vec<&str>>();
        location_list1.push(inputs[0].parse::<i32>().expect("Could not parse number"));
        location_list2.push(inputs[1].parse::<i32>().expect("Could not parse number"));
    }

    location_list1.sort();
    location_list2.sort();

    let mut list_distance = 0;
    for i in 0..location_list1.len() {
        list_distance += (location_list1[i] - location_list2[i]).abs();
    }

    println!("List distance: {}", list_distance);

    let mut occurrences_in_list2 = HashMap::new();

    for location in location_list2 {
        *occurrences_in_list2.entry(location).or_insert(0) += 1;
    }

    let mut result_distance : i64 = 0;
    for location_id in location_list1 {
        result_distance = result_distance + i64::from(location_id * *occurrences_in_list2.entry(location_id).or_insert(0));
    }

    println!("Result distance: {}", result_distance);
}
