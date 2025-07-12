use nebula_lexer::tokenizer;

#[test]
fn test_is_whitespace() {
    assert!(tokenizer::is_whitespace(' '));
    assert!(tokenizer::is_whitespace('\n'));
}

#[test]
fn test_is_whitespace_exlude() {
    for i in 0..u16::MAX {
        if let Some(ch) = char::from_u32(i as u32) {
            if ch == ' ' || ch == '\n' {
                continue;
            }

            assert!(
                !tokenizer::is_whitespace(ch),
                "Char {} would qualify as whitespace.",
                ch
            );
        }
    }
}

#[test]
fn test_is_terminator() {
    assert!(tokenizer::is_terminator(';'));
    assert!(tokenizer::is_terminator('{'));
    assert!(tokenizer::is_terminator('}'));
}

#[test]
fn test_is_terminator_exclude() {
    for i in 0..u16::MAX {
        if let Some(ch) = char::from_u32(i as u32) {
            if ch == ';' || ch == '{' || ch == '}' {
                continue;
            }

            assert!(
                !tokenizer::is_terminator(ch),
                "Char {} would qualify as terminator.",
                ch
            );
        }
    }
}

#[test]
fn test_is_keyword() {
    assert!(tokenizer::is_keyword("var"));
}

#[test]
fn test_is_operator() {
    assert!(tokenizer::is_operator('='));
}

#[test]
fn test_is_numeric() {
    assert!(tokenizer::is_numeric('0'));
    assert!(tokenizer::is_numeric('1'));
    assert!(tokenizer::is_numeric('2'));
    assert!(tokenizer::is_numeric('3'));
    assert!(tokenizer::is_numeric('4'));
    assert!(tokenizer::is_numeric('5'));
    assert!(tokenizer::is_numeric('6'));
    assert!(tokenizer::is_numeric('7'));
    assert!(tokenizer::is_numeric('8'));
    assert!(tokenizer::is_numeric('9'));
}

#[test]
fn test_is_numeric_exclude() {
    for i in 0..u16::MAX {
        if let Some(ch) = char::from_u32(i as u32) {
            if ch == '0'
                || ch == '1'
                || ch == '2'
                || ch == '3'
                || ch == '4'
                || ch == '5'
                || ch == '6'
                || ch == '7'
                || ch == '8'
                || ch == '9'
            {
                continue;
            }

            assert!(
                !tokenizer::is_numeric(ch),
                "Char {} would qualify as numeric.",
                ch
            );
        }
    }
}
