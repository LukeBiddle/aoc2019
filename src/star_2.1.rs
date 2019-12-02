use std::fs;

const INPUT_FILE: &str = "src/inputs/day_2.txt";

fn main() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Something went wrong reading the file");

    println!("Input text: {}", contents);

    let mut intcode: Vec<i32> = contents.split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    intcode[1] = 12;
    intcode[2] = 2;

    let mut index = 0;
    while index < intcode.len() {
        if intcode[index] == 99 {
            break;
        }
        let a = intcode[intcode[index + 1] as usize];
        let b = intcode[intcode[index + 2] as usize];
        let pos = intcode[index + 3] as usize;

        if intcode[index] == 1 {
            intcode[pos] = a + b
        }
        else if intcode[index] == 2 {
            intcode[pos] = a * b
        }
        else {
            println!("Invalid opcode: {}", intcode[index]);
            break;
        }

        index += 4;
    }

    println!("Value at position 0 is: {}", intcode[0]);
}
