use std::fs;

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day2_input")
        .expect("The input file should be read.");

    let mut safe_count = 0;
    let mut safe_count_with_dampener = 0;

    for line in input_file.lines() {
        let split : Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().expect("should be parsable")).collect();

        if is_safe(&split) {
            println!("{} - Safe", line);
            safe_count += 1;
        } else {
            println!("{} - Unsafe", line);

            for i in 0..split.len() {
                let mut copy = split.to_vec();
                copy.remove(i);

                if is_safe(&copy) {
                    safe_count_with_dampener += 1;
                    println!("{} - Safe with dampener though, removed {i} - {}", line, split[i]);
                    break;
                }
            }
        }
    }

    println!("Safe count: {}", safe_count);
    println!("Safe with dampener: {}", safe_count + safe_count_with_dampener);
}

fn is_safe(numbers: &Vec<i32>) -> bool {
    let mut iterator = numbers.iter();
    let mut first = *iterator.next().unwrap();
    let mut second = *iterator.next().unwrap();
    let increasing = first < second;

    loop {
        let diff = first - second;
        if diff == 0 || (diff > 0 && increasing) || (diff < 0 && !increasing) || diff.abs() > 3 {
            break false;
        }

        match iterator.next() {
            None => break true,
            Some(value) => {
                first = second;
                second = *value;
            }
        }
    }
}
