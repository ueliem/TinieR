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
    ColonState,
    EqualState,
    ExclamState,
    NotEqState,
    ColEqState,
    EqEqState
}

pub fn tokenize(inputstring: &str) {
    let mut current_state = StartState;
    let mut cur_token_start = 0u;
    let mut cur_token_end = 0u;
    let mut tokens: Vec<&str> = Vec::new();
    for (i,c) in inputstring.chars().enumerate() {
        match current_state {
            StartState => {
                match c {
                    '0'..'9' => {//Digit
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        continue;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        continue;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
                    '0'..'9' => {//Digit
                        cur_token_end = i;
                        tokens.push(inputstring.slice(cur_token_start, cur_token_end));
                        cur_token_start = i;
                        current_state = NumberState;
                    },
                    'a'..'z' => {//lowercase alphabet
                        fail!("Encountered illegal character!");
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
}
