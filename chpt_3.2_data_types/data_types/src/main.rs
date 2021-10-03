// Chapter 3 (and a bit of 4) Example Project: Online Quiz Program
use std::io;
use std::cmp::Ordering;

fn main() {
    let questions = [
        "What is 2 + 2?",
        "Rust is a cool programming language. (true or false)?",
        "How many sides does a square have?\nA: 1\nB: 2\nC: 3\nD: 4",
        "What color does the sky appear to be during the day?"
    ];
    let correct_answers = ["4", "true", "d", "blue"];
    let mut user_score: u8 = 0;
    println!("Welcome to the quiz!\n---");

    for (i, &question) in questions.iter().enumerate() {
        let answer = question_user(i, &question);
        user_score += check_answer(correct_answers[i], &answer);
    }

    println!("Quiz is finished! You got {} out of 4 questions right!", user_score);
}

fn question_user(question_number: usize, question: &str) -> String {
    let mut user_answer = String::new();
    println!("Question {}: {}", question_number + 1, question);
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Error reading user input.");
    String::from(user_answer.trim())
}

fn check_answer(correct_answer: &str, user_answer: &str) -> u8 {
    match correct_answer.to_lowercase().cmp(&user_answer.to_lowercase()) {
        Ordering::Equal => { println!("Correct!"); 1 }
        _ => { println!("Incorrect! Correct answer was {}", correct_answer); 0} 
    }
}
