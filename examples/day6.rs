use std::char::from_digit;
use std::fs;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day6")
        .expect("The input file should be read.");
    
    // Not very effective to use Vec<String> but I wanted to play with Strings
    let mut map_original = Vec::<String>::new();
    let mut size = (0, 0);

    let mut starting_position = (0, 0);
    let mut current_direction = Direction::UP;
    for (y, line_orig) in input_file.lines().into_iter().enumerate() {
        let line = line_orig.trim().to_string();
        map_original.push(line);

        if let Some(x) = line_orig.find("^") {
            starting_position = (x,y);
            size.0 = line_orig.len() - 1;
        }
    }
    size.1 = map_original.len() - 1;

    let mut map = map_original.clone();
    let mut current_position = starting_position.clone();
    set_visited(&mut map, &current_position);
    let mut result = 1;

    loop {
        let (x,y) = move_forward(&current_position, &current_direction);
        if x < 0 || y < 0 || x as usize > size.0 || y as usize > size.1 {
            break
        }
        let x = x as usize;
        let y = y as usize;

        if at(&map, x, y) == '#' {
            current_direction = turn_right(&current_direction);
            continue
        }


        current_position = (x, y);
        if set_visited(&mut map, &current_position) {
            result += 1;
        }
    }

    for line in map {
        println!("{}", line);
    }

    println!("Answer 1: {}", result);

    println!("{}x{}", size.0, size.1);

    result = 0;
    for i in 0..size.0 + 1 {
        for j in 0..size.1 + 1 {
            if j % 10 == 0 {
                println!("{}x{}", i, j);
            }
            let mut map = map_original.clone();
            if at(&map, i, j) == '#' || at(&map, i, j) == '^' {
                continue;
            }
            current_position = starting_position.clone();
            current_direction = Direction::UP;

            set_at(&mut map, &(i, j), 'O');

            loop {
                let (x,y) = move_forward(&current_position, &current_direction);
                if x < 0 || y < 0 || x as usize > size.0 || y as usize > size.1 {
                    break
                }
                let x = x as usize;
                let y = y as usize;

                if at(&map, x, y) == '#' || at(&map, x, y) == 'O' {
                    current_direction = turn_right(&current_direction);
                    continue
                }


                current_position = (x, y);
                if count_visited(&mut map, &current_position) > 4 {
                    result += 1;
                    // for line in &map {
                    //     println!("{}", *line);
                    // }
                    // println!();
                    break
                }
            }
        }
    }

    println!("Answer 2: {}", result);

}

fn set_visited(map: &mut Vec<String>, pos: &(usize, usize)) -> bool {
    if at(map, pos.0, pos.1) == 'X' {
        return false
    }

    set_at(map, pos, 'X');
    true
}

fn count_visited(map: &mut Vec<String>, pos: &(usize, usize)) -> u32 {
    let (x,y) = *pos;
    if at(&map, pos.0, pos.1) == '.' || at(&map, pos.0, pos.1) == '^' {
        map[y].replace_range(x..x+1, "1");
        return 1
    }

    const RADIX: u32 = 10;
    //println!("{}", at(map, pos.0, pos.1));
    let current_value = at(&map, pos.0, pos.1).to_digit(RADIX).unwrap();

    set_at(map, pos, from_digit(current_value+1, RADIX).unwrap());
    current_value + 1
}

fn move_forward(pos: &(usize, usize), dir: &Direction) -> (i32, i32) {
    let x = pos.0 as i32;
    let y = pos.1 as i32;
    match *dir {
        Direction::UP => {(x, y - 1)},
        Direction::DOWN => {(x, y + 1)},
        Direction::LEFT => {(x - 1, y)},
        Direction::RIGHT => {(x + 1, y)},
    }
}

fn at(v: &Vec<String>, x: usize ,y: usize) -> char {
    v[y].as_bytes()[x] as char
}

fn set_at(map: &mut Vec<String>, pos: &(usize, usize), c: char) {
    let (x,y) = *pos;
    map[y].replace_range(x..x+1, c.to_string().as_str());
}

fn turn_right(dir: &Direction) -> Direction {
    match dir {
        Direction::UP => Direction::RIGHT,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
        Direction::RIGHT => Direction::DOWN
    }
}

