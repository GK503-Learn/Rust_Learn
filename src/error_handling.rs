use std::io;

pub fn start() {
    println!("Enter a whole number:");
    
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");
    
        let num: i32 = input.trim().parse().expect("that wasn't a whole number");
        // ::parse() takes in a String and turns it into the correct data type
    println!("Twice {} is {}", num, num * 2);
}