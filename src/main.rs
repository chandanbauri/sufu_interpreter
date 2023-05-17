mod lexer;

use crate::lexer::interpret;

fn main() {
    let mut lexer_input:String = String::from("=+(){},;");

    interpret(&mut lexer_input)
}
