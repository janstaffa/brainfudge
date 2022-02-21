use brainfudge::get_args;
use std::io::{self, BufRead, Write};
use std::fs;

const DEFAULT_MEMORY_SIZE: usize = 30000; 
fn main() {
    // take user input
    let args = get_args();
    if args.len() < 2 {
        panic!("{}", String::from("Err: Not enough arguments!\nPlease provide a path to the file you want to run."));
    }
    
    println!("Interpreting {}", &args[1]);

    let memory_size: usize = if let Some(custom_memory_size) = args.get(2) {
        let parsed: usize = custom_memory_size.parse().unwrap_or(DEFAULT_MEMORY_SIZE);
        if parsed >= 10 {
            parsed
        } else {
            DEFAULT_MEMORY_SIZE
        }
    } else {
        DEFAULT_MEMORY_SIZE
    };

    // parse the input file(remove newlines, convert to vector of chars)
    let contents = fs::read_to_string(args[1].clone()).expect("Failed to read file.");
    let contents = contents.replace("\r", "");
    let contents = contents.replace("\n", "");
    let chars: Vec<char> = contents.chars().collect();


    // allocate the memmory space for the program
    let mut memory = vec![0u8; memory_size];
    

    // this is the cursor which will be moved through the memmory
    let mut cursor = 0;

    //            (cursor_idx, loop_start)
    let mut loops: Vec<(usize, usize)> = Vec::new();
    let mut input_count = 1;

    // current position in the program
    let mut reader_pos = 0;
    loop {  
        if reader_pos >= chars.len() {
            break;
        }
        let token = chars[reader_pos];

        // handle individual instructions
        match token{
            // increment the current memory address
            '+' => {
                if memory[cursor] > u8::MAX - 1 {
                    println!("Err: number overflow! character: {}\nYour program tried to add with overflow. The maximum number is 2^8-1.", &reader_pos);
                    return;
                }
                memory[cursor] += 1;
                reader_pos += 1;
            },
            // decrement the current memory address
            '-' => {
                if memory[cursor] == 0 {
                    println!("Err: negative numbers not allowed! character: {}\nYour program tried to substract from zero.", &reader_pos);
                    return;
                }
                memory[cursor] -= 1;
                reader_pos += 1;
            },
            // move the cursor forward
            '>' => {
                if cursor == memory.len() - 1 {
                    println!("Err: stack overflow! character: {}\nYour program tried to exceed the maximum memory address. Maximum valid address: {}.", &reader_pos, &memory_size-1);
                    return;
                }
                cursor += 1;
                reader_pos += 1;
            },
            // move the cursor back
            '<' => {
                if cursor == 0 {
                    println!("Err: invalid memory adress! character: {}\nYour program tried to access a negative memory address. Address range is 0-{}.", &reader_pos, &memory_size-1);
                    return;
                }
                cursor -= 1;
                reader_pos += 1;
            },
            // start a loop
            '[' => {
                loops.push((cursor, reader_pos + 1));
                reader_pos += 1;
            },
            // end a loop
            ']' => {
                io::stdout().flush().unwrap();

                if loops.len() == 0 {
                    println!("Err: syntax error! character: {}\nYou first have to create loop before ending it...", &reader_pos);
                    return;
                }
                let (cursor_idx, loop_start) = loops[loops.len() - 1];
                if memory[cursor_idx] > 0 {
                    reader_pos = loop_start;
                    continue;
                }
                loops.pop();
                
                reader_pos += 1;
            },
            // print ASCII character at the current memory address
            '.' => {
                let value = memory[cursor];
                print!("{}", value as char);
                reader_pos += 1;
            },
            // take user input
            ',' => {
                loop {
                    print!("Program input {}: ", input_count);
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    let stdin = io::stdin();
                    stdin.lock().read_line(&mut input).unwrap();


                    if input.len() > 0 {
                        let byte = input.chars().next().unwrap();
                        let number = byte as u8;
                        memory[cursor] = number;
                        reader_pos += 1;
                        input_count += 1;
                        break;
                    }
                }
            },
            // any unknown characters are treated as comments
            _ => {
                reader_pos += 1;
                continue;
            }
        }
    }
    print!("\n");
}

