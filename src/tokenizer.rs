enum TokenizeState {
    StartState,
    NumberState,
    WordState,
    SemicolonState,
    CommaState,
    LParState,
    RParState,
    PlusState,
    MinusState,
    StarState,
    SlashState,
    ColonState,
    EqualState,
    ExclamState,
    NotEqState,
    ColEqState,
    EqEqState
}

pub fn tokenize<'a>(inputstring: &'a str) -> Vec<&'a str> {
    let mut current_state = StartState;
    let mut cur_token_start = 0u;
    let mut cur_token_end = 0u;
    let mut tokens: Vec<&str> = Vec::new();
    for (i,c) in inputstring.chars().enumerate() {
        match current_state {
            StartState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            NumberState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        continue;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            WordState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        continue;
                    },
                    'a'..'z' => {//lowercase alphabet
                        continue;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        continue;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            SemicolonState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            CommaState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            LParState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            RParState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            PlusState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            MinusState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            StarState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
    	    },
    	    SlashState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
    	    },
    	    ColonState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            EqualState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            ExclamState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            NotEqState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            ColEqState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            },
            EqEqState => {
                match c {
                    ' ' | '\t' | '\n' => {//Whitespace
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StartState;
                    },
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    'A'..'Z' => {//uppercase alphabet
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = WordState;
                    },
                    ';' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SemicolonState;
                    },
                    ',' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = CommaState;
                    },
                    '(' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = LParState;
                    },
                    ')' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = RParState;
                    },
                    '+' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = PlusState;
                    },
                    '-' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = MinusState;
                    },
                    '*' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = StarState;
                    },
                    '/' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = SlashState;
                    },
                    ':' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ColonState;
                    },
                    '=' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = EqualState;
                    },
                    '!' => {
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = ExclamState;
                    },
                    _ => {
                        fail!("Encountered illegal character!");
                    }
                }
            }//,
            // _ => fail!()
        }
    }
    return tokens;
}
