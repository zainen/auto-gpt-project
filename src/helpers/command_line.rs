use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_colour: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // print the agent statement in a specific colour
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        //make selected colour
        stdout
            .execute(SetForegroundColor(statement_colour))
            .unwrap();
        println!("{}", agent_statement);

        // reset colour
        let _ = stdout.execute(ResetColor);
    }
}

// get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // print questions in colour
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    //reset colour
    stdout.execute(ResetColor).unwrap();

    //read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("failed to read response");

    // trim white space
    return user_response.trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints_agent_msg() {
        PrintCommand::AICall
            .print_agent_message("Managing agent", "Testing testing, processing something");
    }
}
