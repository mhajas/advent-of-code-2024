use std::fs;

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day4_input")
        .expect("The input file should be read.");

    let searched_string = "XMAS";
    let mut horizontal = Vec::<String>::new();
    let mut size = 0;
    for line in input_file.lines() {
        if size == 0 {
            size = line.len();
        }
        horizontal.push(line.trim().chars().collect());
    }

    let mut vertical = Vec::<String>::with_capacity(size);
    let mut diagonal = Vec::<String>::with_capacity(size * 2);
    let mut diagonal2 = Vec::<String>::with_capacity(size * 2);

    for i in 0..size {
        for j in 0..size {
            if i == 0 {
                vertical.push("".to_string());
            }
            vertical[j].push(horizontal[i].as_bytes()[j] as char);

            let diagonal_index = i + j;
            if i == 0 || j == size - 1 {
                diagonal.push("".to_string());
                diagonal2.push("".to_string());
            }
            diagonal[diagonal_index].push(horizontal[i].as_bytes()[j] as char);
            diagonal2[diagonal_index].push(horizontal[size - i - 1].as_bytes()[j] as char);
        }
    }

    let mut result_count = 0;
    count_in_vec(&horizontal, searched_string, &mut result_count);
    println!();
    count_in_vec(&vertical, searched_string, &mut result_count);
    println!();
    count_in_vec(&diagonal, searched_string, &mut result_count);
    println!();
    count_in_vec(&diagonal2, searched_string, &mut result_count);

    println!("Answer: {}", result_count);

    let mut second_result_count = 0;
    for i in 1..(size - 1) {
        for j in 1..(size - 1) {
            if at(&horizontal, i, j) == 'A' {
                if
                match (at(&horizontal, i - 1, j - 1), at(&horizontal, i + 1, j + 1)) {
                    ('M', 'S') => true,
                    ('S', 'M') => true,
                    _ => false,
                }
                &&
                match (at(&horizontal, i + 1, j - 1), at(&horizontal, i - 1, j + 1)) {
                    ('M', 'S') => true,
                    ('S', 'M') => true,
                    _ => false,
                } {
                    second_result_count += 1;
                }
            }
        }
    }

    println!("Second answer: {}", second_result_count);
}

fn count_string(s: &str, searched:&str) -> usize {
    s.matches(searched).count() + s.chars().rev().collect::<String>().matches(searched).count()
}

fn count_in_vec(v: &Vec<String>, searched:&str, r: &mut usize) {
    for i in 0..v.len() {
        let result = count_string(v[i].as_str(), searched);
        println!("{} {}", v[i], result);
        *r += result;

    }
}

fn at(v: &Vec<String>, i: usize, j:usize) -> char {
    v[i].chars().nth(j).unwrap()
}