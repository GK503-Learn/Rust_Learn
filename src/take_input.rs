use std::io;

pub fn start() {
    println!("Whats your name?");

    let mut name = String::new(); // Creates new empty string

    io::stdin()
        .read_line(&mut name) // puts the read value into string name
        .expect("failed to read input");
    // read_line puts a \n because we press enter "name\n"
    
    let trimmed = name.trim(); // Use trim to remove that \n
    println!("Hello {trimmed}!");


    // Tail expression
    let y = {
        let a = 2;
        a * 3 // Don't put a ; here because its used in rust to discard after a line runs
    };
    println!("{y}")
}