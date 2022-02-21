use chrono::prelude::*;

#[derive(Debug)]
pub struct Card {
    question: String,
    answer: String,
    timestamp: DateTime<Local>,
}

impl Card {
    pub fn simple(question: &str, answer: &str) -> Card {
        let dt: DateTime<Local> = chrono::Local::now();


        Card {
            question: question.to_string(),
            answer: answer.to_string(),
            timestamp: dt,
        }
    }

    pub fn print_card(&self) {
        println!("{} answer {}", self.question, self.timestamp.format("%Y-%m-%d %R %Z"))
    }
}

