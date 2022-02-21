use brainfudge::get_args;
use std::io::{self, BufRead, Write};
use std::fs;

fn main() {
    let args = get_args();
    if args.len() < 2 {
        panic!("{}", String::from("Not enough arguments."));
    }
    
    println!("Interpreting {}", &args[1]);

    let contents = fs::read_to_string(args[1].clone()).expect("Failed to read file.");
    let contents = contents.replace("\n", "");
    let chars: Vec<char> = contents.chars().collect();


    let mut memory = [0u8; 30000];
    

    let mut cursor = 0;

    // (cursor_idx, loop_start)
    let mut loops: Vec<(usize, usize)> = Vec::new();
    let mut is_loop = false;
    let mut loop_cell = 0;

    let mut reader_pos = 0;
    loop {  
        if reader_pos >= chars.len() {
            break;
        }
        let token = chars[reader_pos];
        match token{
            '+' => {
                memory[cursor] += 1;
                reader_pos += 1;
            },
            '-' => {
                if memory[cursor] > 0 {
                    memory[cursor] -= 1;
                }
                reader_pos += 1;
            },
            '>' => {
                if cursor + 1 < memory.len(){
                    cursor += 1;
                }
                reader_pos += 1;
            },
            '<' => {
                if cursor > 0 {
                    cursor -= 1;
                }
                reader_pos += 1;
            },
            '[' => {
                loops.push((cursor, reader_pos + 1));
                reader_pos += 1;
            },
            ']' => {
                io::stdout().flush();

                if loops.len() > 0 {
                    let (cursor_idx, loop_start) = loops[loops.len() - 1];
                    if memory[cursor_idx] > 0 {
                        reader_pos = loop_start;
                        continue;
                    }
                    loops.pop();
                }
                reader_pos += 1;
            },
            '.' => {
                let value = memory[cursor];
                if value < 0 {
                    println!("Err: ascii character for {} does not exist.", value);
                    continue;
                }
                print!("{}", value as char);
                reader_pos += 1;
            },
            ',' => {
                print!("Program input: ");
                io::stdout().flush();
                let mut input = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut input).unwrap();


                if input.len() > 0 {
                    let byte = input.chars().next().unwrap();
                    let number = byte as u8;
                    memory[cursor] = number;
                }
                reader_pos += 1;
            },
            _ => {
                reader_pos += 1;
                continue;
            }
        }
    }
    print!("\n");
}

