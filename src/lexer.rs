  pub type TokenType = String;

  pub struct Token {
      token_type: TokenType,
      literal: String
  }

  pub struct  TestToken {
      expected_type: TokenType,
      expected_literal: String
  }

  pub struct Lexer {
      input: String,
      position: usize,
      read_position: usize,
      ch: String,
  }
   impl Lexer {
      fn next_token(&mut self) -> Token {
          

        let ch_clone = self.ch.clone();
        let tok: Token;
        if ch_clone == ILLEGAL {
            tok = new_token(ILLEGAL.to_string(), self.ch.clone())
        } else if ch_clone == ASSIGN {
            tok = new_token(ASSIGN.to_string(), self.ch.clone())
        } else if ch_clone == PLUS {
            tok = new_token(PLUS.to_string(), self.ch.clone())
        } else if ch_clone == LPAREN {
            tok = new_token(LPAREN.to_string(), self.ch.clone())
        } else if ch_clone == RPAREN {
            tok = new_token(RPAREN.to_string(), self.ch.clone())
        } else if ch_clone == LBRACE {
            tok = new_token(LBRACE.to_string(), self.ch.clone())
        } else if ch_clone == RBRACE {
            tok = new_token(RBRACE.to_string(), self.ch.clone())
        } else if ch_clone == COMMA {
            tok = new_token(COMMA.to_string(), self.ch.clone())
        } else if ch_clone == SEMICOLON {
            tok = new_token(SEMICOLON.to_string(), self.ch.clone())
        } else if ch_clone == EOF {
            tok = new_token(EOF.to_string(), self.ch.clone())
        } else {
            tok = new_token(EOF.to_string(),EOF.to_string())
        }
        self.read_char();
        tok
                 
    }
      
    fn read_char(&mut self) {
         let len = self.input.len();
                   if self.read_position >= len {
                      self.ch = String::from("0");
                  }
          else {
     self.ch = self.input.chars().nth(self.read_position).unwrap_or('a').to_string();


          }
                               self.position = self.read_position;
                  self.read_position+= 1;
       }

  }

      fn new_token(token_type: TokenType , ch: String) -> Token {
      Token {
          token_type,
          literal: ch.to_string(),
        }
      }


	pub const ILLEGAL: &str = "ILLEGAL";
	pub const EOF: &str = "";
	pub const INDENT: &str = "INDENT";
	pub const INT: &str = "INT";
	pub const ASSIGN: &str = "=";
	pub const PLUS: &str = "+";
	pub const COMMA: &str = ",";
	pub const SEMICOLON: &str = ";";
	pub const LPAREN: &str = "(";
	pub const RPAREN: &str = ")";
	pub const LBRACE: &str = "{";
	pub const RBRACE: &str = "}";
	pub const FUNCTION: &str = "FUNCTION";
	pub const LET: &str = "LET";




    pub fn invoke_lexer() {
        println!("Lexer is live");
    }
    
    
    fn new_lexer(input:&mut String) -> &'static mut Lexer {
      if assert_eq!(input.len(),0) {
             let lexer = Box::new(Lexer {
                    input: input.to_string(),
                    position: 0,
                    read_position: 1,
                    ch: input.chars().nth(0).unwrap_or('a').to_string(),
                    });

            let lexer_ref: &mut Lexer = Box::leak(lexer);
            lexer_ref
      } else {
          let optional_value = String::from("+()");
             let lexer = Box::new(Lexer {
                    input: optional_value.clone(),
                    position: 0,
                    read_position: 1,
                    ch: optional_value.chars().nth(0).unwrap_or('a').to_string(),
                    });

            let lexer_ref: &mut Lexer = Box::leak(lexer);
            lexer_ref

      }
            }

    pub fn interpret(input:&mut String) {
      let l = new_lexer(input);


      let tests:Vec<TestToken> = vec![
    TestToken { expected_type: ASSIGN.to_string(), expected_literal: String::from("=") },
    TestToken { expected_type: PLUS.to_string(), expected_literal: String::from("+") },
    TestToken { expected_type: LPAREN.to_string(), expected_literal: String::from("(") },
    TestToken { expected_type: RPAREN.to_string(), expected_literal: String::from(")") },
    TestToken { expected_type: LBRACE.to_string(), expected_literal: String::from("{") },
    TestToken { expected_type: RBRACE.to_string(), expected_literal: String::from("}") },
    TestToken { expected_type: COMMA.to_string(), expected_literal: String::from(",") },
    TestToken { expected_type: SEMICOLON.to_string(), expected_literal: String::from(";") },
    TestToken { expected_type: EOF.to_string(), expected_literal: String::from("") },
      ];

    let tests_len = tests.len();

      for test in tests {
          let token = l.next_token();

          if token.token_type != test.expected_type {
              println!("token did not match {}",token.token_type);
          } else {
              println!("token matched {}",token.token_type);

          }

          if token.literal != test.expected_literal {
              println!("token literal did not match {}",token.literal)
          }else {
              println!("token literal matched {}",token.literal)

          }



      }
    }


