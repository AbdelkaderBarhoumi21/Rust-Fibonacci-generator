use std::io;
// 0 1 1 2 3 5 8 13 ....
fn main() {
    println!("Fibonacci Sequence Generator");
    println!("Enter the number of terls you want to generate");
    let num_terms = match get_input() {
        Some(Value) => Value,
        None => {
            println!("Invalid input please enter a positive integer");
            return;
        }
    };
    if num_terms == 0 {
        println!("Number of terms must be greater than zero.");
        return;
    }
    let sequence = generate_fibonacci(num_terms);
    println!("Fibonacci Sequence ({} terms): {:?}", num_terms, sequence);
}
//read user input and parse it as u32
fn get_input() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();
    if n >= 1 {
        sequence.push(0); //first term
    }
    if n >= 2 {
        sequence.push(1); //second term
    }
    for i in 2..n {
        let next = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next);
    }
    sequence
}
