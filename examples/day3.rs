use std::fs;
use regex::Regex;

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day3_input")
        .expect("The input file should be read.");

    let mul_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let dont_indices = input_file
        .match_indices("don't()")
        .map(|x| i32::try_from(x.0).unwrap())
        .collect::<Vec<i32>>();

    let mut do_indices = input_file
        .match_indices("do()")
        .map(|x| i32::try_from(x.0).unwrap())
        .collect::<Vec<i32>>();

    do_indices.push(i32::try_from(input_file.len()).unwrap());

    println!("{dont_indices:?}");
    println!("{do_indices:?}");

    let mut ignored_intervals :Vec<(i32, i32)> = Vec::new();

    for dont_start in dont_indices {
        let do_start = do_indices.iter().find(|&&do_start| do_start > dont_start).unwrap();
        ignored_intervals.push((dont_start, *do_start));
    }

    println!("{ignored_intervals:?}");

    let mut result = 0;
    let mut result_with_ignored_intervals = 0;
    for captures in mul_regex.captures_iter(&input_file) {
        println!("{:#?}", captures);
        let capture_index = captures.get(0).unwrap().start();
        let mul_result = captures.get(1).unwrap().as_str().parse::<i32>().unwrap() * captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        result += mul_result;
        if ignored_intervals.iter().find(|&&(start,end)| capture_index > start as usize && capture_index < end as usize).is_none() {
            result_with_ignored_intervals += mul_result;
        }
    }

    println!("Answer is: {}", result);
    println!("Answer with ignored intervals: {}", result_with_ignored_intervals);
}