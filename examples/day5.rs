use std::fs;
use itertools::Itertools;

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day5_input")
        .expect("The input file should be read.");

    let mut rules = Vec::<(i32,i32)>::new();
    let mut lines_iterator = input_file.lines().into_iter();

    while let Some(line) = lines_iterator.next() {
        if line.contains("|") {
            let pair :(i32, i32) = line.split("|").map(|x| x.parse::<i32>().expect("should be parsable")).collect_tuple().unwrap();
            rules.push(pair);
            continue;
        }

        if line.is_empty() {break}
    }


    let mut result = (0, 0);
    while let Some(line) = lines_iterator.next() {
        let printing_order :Vec<i32> = line.split(",").map(|x| x.parse::<i32>().expect("should be parsable")).collect();

        match get_result(&rules, &printing_order, false) {
            (r, false) => {
                result.0 += r;
            },
            (r, true) => {
                result.1 += r;
            }
        }
    }

    print!("{}, {}", result.0, result.1);
}

fn get_result(rules:&Vec<(i32,i32)>, printing_order:&Vec<i32>, is_fixup: bool) -> (i32, bool)  {
    let mut cant_see = Vec::<(usize,i32)>::new();
    for (index_in_list, to_print) in printing_order.iter().enumerate() {
        for (first,second) in rules {
            if *to_print == *second {
                cant_see.push((index_in_list, *first))
            }
        }

        if let Some((broken_rule_index, _)) = cant_see.iter().find(|(_, x)| *x == *to_print) {
            let mut copy = printing_order.clone();
            copy[index_in_list] = copy[*broken_rule_index];
            copy[*broken_rule_index] = *to_print;

            return get_result(rules, &copy, true);
        }
    }

    (printing_order[&printing_order.len()/2], is_fixup)
}