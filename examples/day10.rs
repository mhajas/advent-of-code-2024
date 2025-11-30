use std::collections::HashSet;
use std::fs;

type Dimension = (usize, usize);

struct Config {
    map: Vec<String>,
}

impl Config {
    fn size(&self) -> Dimension {
        (self.map[0].len(), self.map.len())
    }
}

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day10")
        .expect("The input file should be read.");


    let mut map_original = Vec::<String>::new();
    for (_y, line_orig) in input_file.lines().into_iter().enumerate() {
        map_original.push(line_orig.trim().to_string());
    }
    let cfg = Config { map: map_original };

    let size = cfg.size();

    // Find trailheads
    let mut trailheads = HashSet::<Dimension>::new();
    for i in 0..size.0 {
        for j in 0..size.1 {
            match at(&cfg, (i, j)) {
                Some(0) => {trailheads.insert((i, j));},
                _ => {}
            }
        }
    }

    // For each trailhead find routes to 9-height positions
    let mut result = 0;
    let mut visited = HashSet::<Dimension>::new();
    for d in &trailheads {
        let mut position = HashSet::<Dimension>::new();
        position.insert(*d);
        visited.insert(*d);
        for i in 1..10 {
            match next_steps(&cfg, &position, i) {
                Some(steps) => {position = steps},
                None => {
                    position.clear();
                    break
                },
            };
            visited.extend(&position);

        }

        result += position.len();
    }
    print_visited(&cfg, &visited);

    println!("Result: {}", result);

    println!("Result2: {}", trailheads.iter().map(|t| recursion_for_part2(&cfg, &t).unwrap_or(0)).fold(0, |a, b| a + b));
}

fn at(cfg : &Config, dimension: Dimension) -> Option<i32> {
    Some(cfg.map.get(dimension.0)?.chars().nth(dimension.1)?.to_digit(10).unwrap() as i32)
}

fn next_steps(cfg : &Config, current_positions: &HashSet<Dimension>, expected_height: i32) -> Option<HashSet<Dimension>> {
    Some(current_positions.iter()
        .flat_map(|pos| look_around(pos))
        .filter(|pos| {
            match at(&cfg, *pos) {
                Some(actual_height) if actual_height == expected_height => {true}
                _ => {false}
            }
        }).collect::<HashSet<Dimension>>())
}

fn recursion_for_part2(cfg: &Config, current_position: &Dimension) -> Option<usize> {
    let next_expected_height = at(cfg, *current_position)? + 1;
    let mut hs = HashSet::<Dimension>::new();
    hs.insert(*current_position);
    let possible_next_steps = next_steps(cfg, &hs, next_expected_height);

    if next_expected_height == 9 {
        if let Some(ns) = possible_next_steps {
            return Some(ns.len());
        }
    }

    possible_next_steps?.iter()
        .map(|p| recursion_for_part2(cfg, p).unwrap_or(0))
        .reduce(|a, b| a + b)
}

fn look_around(dimension: &Dimension) -> HashSet<Dimension> {
    let mut s = HashSet::new();
    s.insert((dimension.0 + 1, dimension.1));
    s.insert((dimension.0, dimension.1 + 1));

    if dimension.0 > 0 {
        s.insert((dimension.0 - 1, dimension.1));
    }
    if dimension.1 > 0 {
        s.insert((dimension.0, dimension.1 - 1));
    }

    s
}

fn print_visited(cfg: &Config, visited: &HashSet<Dimension>) {
    let size = cfg.size();

    for i in 0..size.0 {
        for j in 0..size.1 {
            if visited.contains(&(i, j)) {
                print!("{}", at(cfg, (i, j)).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
