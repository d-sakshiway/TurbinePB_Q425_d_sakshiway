use std::io;

enum Operations {
    Increment,
    Decrement,
}

fn main() {
    println!("Welcome to increment/decrement program");
    let mut counter = 0;
    println!("Counter is starting at {}", counter);

    loop {
        let mut input = String::new();

        println!("Do you want to increment or decrement?  (+ to incremenet and - Decrement; type 'Q' to exit)");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to parse the sign");

        if input.trim().eq_ignore_ascii_case("Q") {
            println!("See Ya, next time!");
            break;
        }

        let ops = match input.trim() {
            "+" => Operations::Increment,
            "-" => Operations::Decrement,
            _ => panic!("Operation not supported yet!"),
        };

        match ops {
            Operations::Increment => counter += 1,
            Operations::Decrement => counter -= 1,
        }

        println!("Now counter is at {}", counter);
    }
}