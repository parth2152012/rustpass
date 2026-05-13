use std::error::Error;
use std::io::{self, Write};

use rpassword::{ConfigBuilder, prompt_password_with_config};

// --------------------------------------------------
// Normal terminal input
// --------------------------------------------------
pub fn get_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();

    print!("{}", prompt);

    io::stdout().flush()?;

    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}

// --------------------------------------------------
// Hidden password input
// --------------------------------------------------
pub fn get_secure_input(prompt: &str) -> Result<String, Box<dyn Error>> {
    let config = ConfigBuilder::new().password_feedback_mask('*').build();

    let password = prompt_password_with_config(prompt, config)?;

    Ok(password)
}
