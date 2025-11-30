use std::fs;

const RADIX: u32 = 10;

#[derive(Debug)]
enum Block {
    Empty(usize, usize, Vec<usize>),
    Fixed(usize, usize, bool)
}

impl Block {

    fn try_fit(&mut self, other: &mut Block) -> bool {
        match (self,other) {
            (Block::Fixed(..), _) => { false },
            (_, Block::Empty(..)) => { false },
            (Block::Empty(_, consumer_len, content), Block::Fixed(index, len, moved)) => {
                if (*consumer_len as i32 - content.len() as i32) < *len as i32 {
                    return false;
                }

                content.append(&mut vec![*index / 2; *len]);
                *moved = true;
                true
            },
        }
    }

    fn print(&self) {
        match self {
            Block::Fixed(_, size, true) => {
                for _ in 0..*size {
                    print!(".");
                }
            }
            Block::Empty(_, size, content) => {
                for i in 0..*size {
                    match content.get(i) {
                        Some(i) => print!("{}", i),
                        None => print!(".")
                    }
                }
            }
            Block::Fixed(index, size, false) => {
                for _ in 0..*size {
                    print!("{}", index/2)
                }
            }
        }
    }

    fn compute_result(&self, current_position: &mut usize) -> u64 {
        let mut result: u64 = 0;

        match self {
            Block::Fixed(_, size, true) => {
                for _ in 0..*size {
                    *current_position += 1;
                }
            },
            Block::Empty(_, size, content) => {
                for i in 0..*size {
                    match content.get(i) {
                        Some(i) => {
                            result = result + ((*i as u64) * (*current_position as u64));
                        },
                        None => ()
                    }
                    *current_position += 1
                }
            }
            Block::Fixed(index, size, false) => {
                for _ in 0..*size {
                    result = result + (((*index / 2) as u64) * *current_position as u64);
                    *current_position += 1;
                }
            }
        };

        result
    }
}

fn main() {
    let input_file = fs::read_to_string("./examples/inputs/day9")
        .expect("The input file should be read.");


    let chars = input_file.chars().collect::<Vec<char>>();

    let mut current_index = 0;
    let mut current_block_index = 0;
    let mut currently_defragmented_part = find_next_defragmented_part(&chars, chars.len());

    let mut result: u64 = 0;

    while current_index <= currently_defragmented_part.0 * 2 {
        if current_index / 2 == currently_defragmented_part.0 {
            for _ in 0..currently_defragmented_part.1 {
                print!("{}", currently_defragmented_part.0);
                result = result + (currently_defragmented_part.0 as u64 * current_block_index);
                current_block_index += 1;
            }
            break;
        } else if current_index % 2 == 0 {
            for _ in 0..value(&chars, current_index) {
                print!("{}", current_index / 2);
                result = result + ((current_index / 2) as u64 * current_block_index);
                current_block_index += 1;
            }
        } else {
            for _ in 0..value(&chars, current_index) {
                print!("{}", currently_defragmented_part.0);
                currently_defragmented_part.1 -= 1;

                result = result + (currently_defragmented_part.0 as u64 * current_block_index);
                current_block_index += 1;

                if currently_defragmented_part.1 == 0 {
                    currently_defragmented_part = find_next_defragmented_part(&chars, (currently_defragmented_part.0*2) - 1);
                    if currently_defragmented_part.0*2 < current_index {
                        break;
                    }
                }
            }
        }

        current_index += 1;
    }

    println!("Result: {}", result);

    let mut blocks = Vec::<Block>::new();
    // Part 2
    for (i,c) in chars.iter().enumerate() {
        if i % 2 == 0 {
            blocks.push(Block::Fixed(i, c.to_digit(RADIX).unwrap() as usize, false))
        } else {
            blocks.push(Block::Empty(i, c.to_digit(RADIX).unwrap() as usize, vec![]))
        }
    }

    println!("{:?}", blocks);

    for i in 5..0 {
        println!("{}", i);
    }

    for moved in (0..blocks.len()).rev() {
        let (blocks1, blocks2) = blocks.split_at_mut(moved);

        for block in blocks1.into_iter() {
            if block.try_fit(&mut blocks2[0]) {
                break;
            }
        }
    }

    println!("{:?}", blocks);

    blocks.iter().for_each(|b| b.print());

    let mut result2 = 0;
    let mut current_position = 0;
    for block in blocks {
        result2 = result2 + block.compute_result(&mut current_position);
    }

    println!("Result2: {}", result2);
}

fn value(system: &Vec<char>, pos: usize) -> u32 {
    system[pos].to_digit(RADIX).unwrap()
}

fn find_next_defragmented_part(system: &Vec<char>, upper_bound: usize) -> (usize, u32) {
    if upper_bound >= system.len() {
        return find_next_defragmented_part(system, upper_bound - 1);
    }
    match system[upper_bound] {
        _ if upper_bound % 2 != 0 => find_next_defragmented_part(system, upper_bound - 1),
        '0' => find_next_defragmented_part(system, upper_bound - 1),
        c => (upper_bound / 2, c.to_digit(RADIX).unwrap())
    }
}

