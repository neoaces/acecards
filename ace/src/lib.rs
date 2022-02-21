use chrono::prelude::*;

#[derive(Debug)]
enum Variant {
    Simple,
    Extended,
}

#[derive(Debug)]
pub struct Card {
    pub question: String,
    pub answer: String,
    timestamp: DateTime<Local>,
    variant: Variant,
}

impl Card {
    pub fn makeCard(question: &str, answer: &str) -> Card {
        let dt: DateTime<Local> = chrono::Local::now();

        Card {
            question: question.to_string(),
            answer: answer.to_string(),
            timestamp: dt,
            variant: Variant::Simple,
        }
    }
}

impl Card {
    pub fn print_card(&self) {
        println!(
            "{:?} answer {}",
            self.variant,
            self.timestamp.format("%Y-%m-%d %R %Z")
        )
    }

    pub fn print_ans(&self) {
        println!("{}", self.answer)
    }

    pub fn print_q(&self) {
        println!("{}", self.question)
    }
}
