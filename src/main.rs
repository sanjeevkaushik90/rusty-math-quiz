use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();

    
    let a: i32 = rng.random_range(1..=100);
    let b: i32 = rng.random_range(1..=100);

    
    let operation: u8 = rng.random_range(0..3);

    let (question, correct_answer) = match operation {
        0 => (format!("{} + {}", a, b), a + b),
        1 => (format!("{} - {}", a, b), a - b),
        2 => (format!("{} * {}", a, b), a * b),
        3 => {
           
            let dividend = a * b;
            (format!("{} / {}", dividend, b), dividend / b)
        }
        _ => unreachable!(),
    };

    println!("Solve this question: {}", question);

    let mut input = String::new();
    println!("Enter your answer=>");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let user_answer: i32 = input.trim().parse().expect("Please enter a valid number");

    if user_answer == correct_answer {
        println!("Correct!");
    } else {
        println!("Wrong! The correct answer is {}", correct_answer);
    }
}
