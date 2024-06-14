use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

#[derive(Debug, Clone)]
pub struct Question {
	text: String,
	answers: Vec<String>,
	correct_answer: i32
}

pub fn read_quiz_file() -> Vec<Question> {
	let mut file = match File::open("quiz_questions.txt") {
		Ok(file) => file,
		Err(_) => {
			println!("Failed to open \'quiz_questions.txt\'!");
			exit(1);
		}
	};

	let mut reader = BufReader::new(file);
	let mut strings: Vec<String> = Vec::new();

	for line in reader.lines() {
		let result = line.unwrap().trim().to_string();
		if !result.is_empty() {
			strings.push(result);
		}
	}

	println!("{:?}", strings);

	let mut questions: Vec<Question> = Vec::new();
	// questions have:
	// question (1), the answers (4), and the right answer (1)
	// so 6 lines per question
	let number_of_questions = strings.len() / 6;
	for question_index in 0..number_of_questions {
		questions.push(Question{
			text: strings.get(question_index * 6).expect("Invalid question text").clone(),
			answers: vec![
				strings.get(1 + question_index * 6).expect("Invalid answer text").clone(),
				strings.get(2 + question_index * 6).expect("Invalid answer text").clone(),
				strings.get(3 + question_index * 6).expect("Invalid answer text").clone(),
				strings.get(4 + question_index * 6).expect("Invalid answer text").clone(),
			],
			correct_answer: strings.get(5 + question_index * 6).expect("Invalid answer text").parse().unwrap(),
		});
	}

	println!("{:#?}", questions);
	questions
}

#[cfg(test)] 
mod tests {
	#[test]
	fn lol() {
		assert!(true);
	}
}