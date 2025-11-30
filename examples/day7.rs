use std::fs;


fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day7")
        .expect("The input file should be read.");

    let mut result = 0;
    let mut result_part_2 = 0;
    for line in input_file.lines() {
        let mut expected_result = 0;
        let mut operands = Vec::<u32>::new();
        for s in line.split_whitespace().collect::<Vec<&str>>() {
            if s.contains(":") {
                expected_result = s.split(":").collect::<Vec<&str>>()[0].parse::<u64>().unwrap();
                continue;
            }
            operands.push(s.parse::<u32>().unwrap());
        }

        println!("{:?} {:?}", operands, expected_result);
        let result1 = recursive_results(&operands);
        //println!("{:?}", result1);
        if result1.iter().find(|&&x| x == expected_result).is_some() {
            result += expected_result;
        }

        let result2 = recursive_results_with_concat(&operands);
        //println!("{:?}", result2.0);
        //println!("{:?}", result2.1);
        if result2.0.iter().find(|&&x| x == expected_result).is_some() {
            result_part_2 += expected_result;
        }

    }

    println!("Result: {}", result);
    println!("Result2: {}", result_part_2);
}

fn recursive_results(source: &Vec<u32>) -> Vec<u64> {
     match source.as_slice(){
        [x] => vec![*x as u64],
        [] => std::process::exit(1),
        [rest@ .., x] => {
            let rest_result = recursive_results(&rest.to_vec());
            let mut new_results = Vec::<u64>::new();

            for r in rest_result {
                new_results.push(*x as u64 + r);
                new_results.push(*x as u64  * r);
            }
            new_results
        },

    }
}

fn recursive_results_with_concat(source: &Vec<u32>) -> (Vec<u64>, Vec<String>) {
    match source.as_slice(){
        [x] => (vec![*x as u64], vec![format!("{}", x)]),
        [] => std::process::exit(1),
        [rest@ .., x] => {
            let rest_result = recursive_results_with_concat(&rest.to_vec());
            let mut new_results = Vec::<u64>::new();
            let mut new_results_str = Vec::<String>::new();

            for r in rest_result.0 {
                new_results.push(*x as u64 + r);
                new_results.push(*x as u64  * r);
                new_results.push(format!("{}{}", r, *x).parse::<u64>().unwrap());
            }

            for r in rest_result.1 {
                new_results_str.push(format!("({} + {})", *x, r));
                new_results_str.push(format!("({} * {})", *x, r));
                new_results_str.push(format!("({} || {})", r, *x));
            }
            (new_results, new_results_str)
        },

    }
}

