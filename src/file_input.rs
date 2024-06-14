use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Question {
	text: String,
	answers: Vec<String>,
}

impl Question {
	pub fn text(&self) -> &String {
		&self.text
	}

	pub fn answers(&self) -> &Vec<String> {
		&self.answers
	}

	pub fn randomize_answers(&mut self) {
		&self.answers.shuffle(&mut thread_rng());
	}
}

pub fn read_quiz_file() -> Vec<Question> {
	let file = match File::open("quiz_questions.txt") {
		Ok(file) => file,
		Err(_) => {
			println!("Failed to open \'quiz_questions.txt\'!");
			exit(1);
		}
	};

	let reader = BufReader::new(file);
	let mut strings: Vec<String> = Vec::new();

	for line in reader.lines() {
		let result = line.unwrap().trim().to_string();
		if !result.is_empty() {
			strings.push(result);
		}
	}

	let mut questions: Vec<Question> = Vec::new();
	// questions have:
	// question (1), the answers (4)
	// so 5 lines of text
	let number_of_questions = strings.len() / 5;
	for question_index in 0..number_of_questions {
		questions.push(Question{
			text: strings.get(question_index * 5).expect("Invalid question text").clone(),
			answers: vec![
				strings.get(1 + question_index * 5).expect("Invalid answer text").clone(),
				strings.get(2 + question_index * 5).expect("Invalid answer text").clone(),
				strings.get(3 + question_index * 5).expect("Invalid answer text").clone(),
				strings.get(4 + question_index * 5).expect("Invalid answer text").clone(),
			],
		});
	}

	println!("{:#?}", questions);
	questions
}