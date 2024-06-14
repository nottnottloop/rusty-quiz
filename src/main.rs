use player_input::parse_player_input;
use rand::thread_rng;
use rand::seq::SliceRandom;

use player_input::read_input;
use player_input::InputError;

mod file_input;
mod player_input;

fn main() {
    println!("Reading 'quiz_questions.txt'...");
    let mut questions = file_input::read_quiz_file();
    questions.shuffle(&mut thread_rng());
    
    println!("*************************");
    println!("Welcome to the Rusty quiz!");
    println!("*************************");
    println!("\nThis quiz will test you on all kinds of really cool questions about the world and stuff. It's amazing.");
    println!("\nThere are {} questions. Every question you get right you get a point. At the end of the quiz you will get a quiz master ranking.", questions.len());
    println!("\nPress enter to start");

    match read_input() {
        _ => ()
    };

    print!("{}[2J", 27 as char);

    let mut current_question = 0;
    let mut correct_answers = 0;

    for question in &questions {
        current_question += 1;
        let user_answer: i32;
        loop {
            println!("\nScore: {correct_answers}");
            println!("Question {current_question}:");
            println!("{}", question.text());
            
            let mut answers = question.answers().iter();
            println!("1. {} 2. {}", answers.next().unwrap(), answers.next().unwrap());
            println!("3. {} 4. {}", answers.next().unwrap(), answers.next().unwrap());
            println!("Answer with the number (1-4)");
            user_answer = match parse_player_input() {
                Ok(x) => x,
                Err(InputError::Empty) => {
                    println!("You need to type something!");
                    continue;
                },
                Err(InputError::Invalid) => {
                    println!("\nInvalid input!");
                    continue;
                },
            };
            break;
        }
        if user_answer == question.correct_answer() {
            println!("\nYou score a point!");
            correct_answers += 1;
        } else {
            let correct_answer_text = &question.answers()[(question.correct_answer() - 1) as usize];
            println!("\nUnlucky!");
            println!("The correct answer was {correct_answer_text}.");
        }

        println!("\nPress enter to continue");

        match read_input() {
            _ => ()
        };

        print!("{}[2J", 27 as char);
    }

    println!("Congratulations, you have finished the quiz!");
    println!("\nAfter {} gruelling questions, I am sure you are in need of a break.", &questions.len());

    let percent: f32 = (correct_answers as f32 / questions.len() as f32) * 100f32;
    println!("\nYou got {:.2}% correct! That's a score of {}!", percent, correct_answers);

    let quiz_ranking = match percent {
        x if x < 10.0 => "🦃 Total quiz noob, don't play again 🦃",
        x if x < 20.0 => "🤬 Wow, that was pretty poor 🤬",
        x if x < 30.0 => "😬 Definitely some improvement needed 😬",
        x if x < 40.0 => "🍻 Ok ok, could be worse in a pub quiz 🍻",
        x if x < 50.0 => "😐 Not at all shabby and maybe a bit handsome 😐",
        x if x < 60.0 => "😏 Hey, that's pretty good 😏",
        x if x < 70.0 => "📚 Ok you definitely get around a book or two 📚",
        x if x < 80.0 => "🎓 Quiz boffin 🎓",
        x if x < 90.0 => "🧠 Quiz lord 🧠",
        x if x == 100.0 => "🔥👏🔥👏 OMEGA QUIZ CHAMPION ULTRA LORD 👏🔥👏🔥",
        _ => "???"
    };

    println!("\nYour ranking for today is");
    println!("\n{quiz_ranking}\n");
}