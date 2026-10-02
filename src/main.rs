fn main() {
    //scalrs

    let _age: u32 = 18; // non negative 32 bit integer

    let _num: i32 = -100; // default for numbers if you don't specify a type

    let _temp: f64 = 36.6; // 64 bit floating point number for decimals

    let _is_active: bool = true; // true or false

    let _grade: char = 'A'; // a single character

    // println!("age: {}, num: {}, temp: {}, is_active: {}, grade: {}", _age , _num , _temp, _is_active, _grade);

    // Compount type

    // TUPLE
    let person: (&str, u32) = ("Maya", 39);
    println!("{} is {} years old", person.0, person.1);

    // Array
    let scores: [i32; 3] = [90, 85, 100];
    println!("scores: {} {} {}", scores[0], scores[1], scores[2]);
}

