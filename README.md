# Brainfudge

The brainfuck language interpreter written in Rust.

## run:

```bash
# compile
cargo build
# run
./target/debug/brainfudge <path to file> <optional: memory size>

# or

# run directly
cargo run <path to file> <optional: memory size>

# example
cargo run minecraft.bf 50000
```

## syntax:

This interpreter uses standard brainfuck syntax i.e:

| symbol | instruction                                             |
| ------ | ------------------------------------------------------- |
| +      | increment the current memory address                    |
| -      | decrement the current memory address                    |
| >      | move the memory cursor forward                          |
| <      | move the memory cursor backwards                        |
| [      | enter a loop                                            |
| ]      | exit a loop                                             |
| .      | print the ASCII character at the current memory address |
| ,      | take input from user                                    |

The input file's extension doesn't matter, it only has to be encoded as plain **ASCII** text. Any symbol not mentioned in the table above won't be executed and treated as a comment (including newline characters).

## memory:

The default allocated memory is 30000 bytes i.e 30000 available memory addresses\*. You can allocate a custom amount of memory by providing it as a plain number as a second argument to the program at runtime. The minimum ammount of memory is 10 bytes.

## comments:

**\*** - a memory address is esentially a single number that you can perform operations on, the memory is just an array of these, by default all alocted bytes are set to 0 (\nul in ASCII), you can move along the memory using `>` and `<` instruction
