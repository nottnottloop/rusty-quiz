use std::io;

pub enum InputError {
    Empty,
    Invalid
}

pub fn read_input() -> Result<String, InputError> {
    let mut string = String::new(); 
    match io::stdin().read_line(&mut string) {
        Ok(_) => {
            if string.trim().is_empty() {
                Err(InputError::Empty)
            } else {
                //Ok(string.as_str().trim().to_string())
                Ok(str::trim(&string).to_string())
            }
        },
        Err(_) => { Err(InputError::Invalid) }
    }
}

pub fn parse_player_input() -> Result<i32, InputError> {
	let user_answer: Result<i32, InputError> = match read_input() {
		Ok(x) => match x.parse() {
			Ok(x) => {
				if (1..=4).contains(&x) {
					Ok(x)
				} else {
					Err(InputError::Invalid)
				}
			}
			Err(_) => Err(InputError::Invalid),
		}
		Err(InputError::Empty) => Err(InputError::Empty),
		Err(InputError::Invalid) => Err(InputError::Invalid)
	};
	user_answer
}