use player_input::parse_player_input;
use rand::thread_rng;
use rand::seq::SliceRandom;
use colored::*;

use player_input::read_input;
use player_input::InputError;

mod file_input;
mod player_input;

fn main() {
    //println!("Reading 'quiz_questions.txt'...");
    let mut questions = file_input::read_quiz_file();
    questions.shuffle(&mut thread_rng());
    
    println!("**************************");
    println!("{}", "Welcome to the Rusty quiz!".yellow());
    println!("**************************");
    println!("\nThis quiz will test you on all kinds of really cool questions about the 🌍 world and stuff.");
    println!("\nThere are {} questions. Every question you get right you get a point.\nAt the end of the quiz you will get a quiz master ranking.", questions.len());
    println!("\nPress Enter to start");

    wait_for_continue();

    clear_console();

    let mut current_question = 0;
    let mut correct_answers = 0;
    let num_of_questions = questions.len();

    for question in &mut questions {
        current_question += 1;
        let user_answer: usize;

        let correct_answer = &question.answers()[0].clone();
        question.randomize_answers();

        loop {
            println!("{} {}", "Score:".bright_green(), correct_answers.to_string().bright_green());
            println!("Question {}/{}:", current_question, num_of_questions);
            println!("\n{}\n", question.text());

            let mut answers = question.answers().iter();
            //println!("1. {} 2. {}", answers.next().unwrap(), answers.next().unwrap());
            //println!("3. {} 4. {}", answers.next().unwrap(), answers.next().unwrap());
            println!("{}", "---".yellow());
            for i in 1..=4 {
                println!("{}. {}", i, answers.next().unwrap());
            }
            println!("{}", "---".yellow());
            println!("\nAnswer with the number (1-4)");
            user_answer = match parse_player_input() {
                Ok(x) => x,
                Err(InputError::Empty) => {
                    println!("You need to type something!");
                    println!("Press Enter to retry");
                    wait_for_continue();
                    clear_console();
                    continue;
                },
                Err(InputError::Invalid) => {
                    println!("\nInvalid input!");
                    println!("Press Enter to retry");
                    wait_for_continue();
                    clear_console();
                    continue;
                },
            };
            break;
        }
        if &question.answers()[user_answer - 1] == correct_answer {
            println!("\nYou score a point!");
            correct_answers += 1;
        } else {
            println!("\nUnlucky!");
            println!("The correct answer was {correct_answer}.");
        }

        println!("\nPress Enter to continue");

        wait_for_continue();

        clear_console();
    }

    println!("{}", "Congratulations, you have finished the quiz!".bright_yellow());
    //println!("\nAfter {} gruelling questions, I am sure you are in need of a break.", &questions.len());

    let percent: f32 = (correct_answers as f32 / num_of_questions as f32) * 100f32;
    print!("\nYou got {:.2}% correct! ", percent);
    print!("{}{}{}{}{}", "That's ".bright_cyan(), correct_answers.to_string().bright_cyan(), " out of ".bright_cyan(), num_of_questions.to_string().bright_cyan(), "!".bright_cyan());

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

    println!("\n\nYour ranking for today is");
    println!("\n\n{quiz_ranking}");
    wait_for_continue();
}

fn clear_console() {
    clearscreen::clear().unwrap();
}

fn wait_for_continue() {
    match read_input() {
        _ => ()
    };
}
