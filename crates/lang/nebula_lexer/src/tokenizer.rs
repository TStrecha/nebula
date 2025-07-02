pub fn is_terminator(ch: char) -> bool {
    matches!(ch, ';' | '{' | '}')
}

pub fn is_whitespace(ch: char) -> bool {
    matches!(ch, ' ' | '\n')
}

pub fn is_keyword(token_str: &str) -> bool {
    matches!(token_str, "var")
}

pub fn is_operator(token_str: &str) -> bool {
    matches!(token_str, "=")
}

pub fn is_numeric(curr_char: char) -> bool {
    curr_char.is_digit(10)
}
