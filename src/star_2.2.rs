use std::fs;

const INPUT_FILE: &str = "src/inputs/day_2.txt";
const TARGET_OUTPUT: i32 = 19690720;


fn get_intcode_output(mut intcode: Vec<i32>, noun: i32, verb: i32) -> i32 {
    intcode[1] = noun;
    intcode[2] = verb;

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

    intcode[0]
}


fn main() {
    let contents = fs::read_to_string(INPUT_FILE)
        .expect("Something went wrong reading the file");

    println!("Input text: {}", contents);

    let intcode: Vec<i32> = contents.split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut noun: i32 = -1;
    let mut verb: i32 = -1;
    for n in 0..99 {
        for v in 0..99 {
            if get_intcode_output(intcode.clone(), n, v) == TARGET_OUTPUT {
                noun = n;
                verb = v;
                break;
            }
        }
    }

    let result: i32 = (100 * noun) + verb;

    println!("Target output {} found with the input: {}", TARGET_OUTPUT, result);
}
