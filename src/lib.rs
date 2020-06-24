//! Library for parsing escape characters

/// Escape [ASCII escapes](https://doc.rust-lang.org/reference/tokens.html#ascii-escapes) in `input`
///
/// Turns sequences that look like escape characters into actual escape characters, i.e. a
/// backslash followed by an 'n' turns into a proper newline character.
/// The only difference between ASCII escapes and Byte escapes is that the maximum value for a hex
/// escape in `escape_ascii` is 0x7F.
pub fn escape_ascii(input: &str) -> Result<String, std::string::FromUtf8Error> {
    if input.len() < 1 {
        return Ok(String::new());
    }

    let mut v = Vec::from(input);
    for i in 0..(v.len() - 1) {
        if v[i] == '\\' as u8 && is_escapable(v[i + 1] as char) {
            v.remove(i);
            v[i] = char_to_escape_sequence(v[i] as char) as u8;
        }
    }
    String::from_utf8(v)
}

/// Escape [Byte escapes](https://doc.rust-lang.org/reference/tokens.html#byte-escapes) in `input`
///
/// Turns sequences that look like escape characters into actual escape characters, i.e. a
/// backslash followed by an 'n' turns into a proper newline character.
///
/// The only difference between Byte escapes and ASCII escapes is that the maximum value for a hex
/// escape in `escape_bytes` is 0xFF.
pub fn escape_bytes(_input: &str) -> Result<String, std::string::FromUtf8Error> {
    unimplemented!("`escape_bytes` is not yet implemented");
}

/// Escape [Unicode escapes](https://doc.rust-lang.org/reference/tokens.html#unicode-escapes) in
/// `input`
pub fn escape_unicode(_input: &str) -> Result<String, std::string::FromUtf8Error> {
    unimplemented!("`escape_unicode` is not yet implemented");
}

/// Escape [Quote escapes](https://doc.rust-lang.org/reference/tokens.html#quote-escapes) in
/// `input`
pub fn escape_quotes(_input: &str) -> Result<String, std::string::FromUtf8Error> {
    unimplemented!("`escape_quotes` is not yet implemented");
}

fn char_to_escape_sequence(chr: char) -> char {
    match chr {
        'n' => '\n',
        't' => '\t',
        'r' => '\r',
        '\\' => '\\',
        '0' => '\0',
        _ => chr,
    }
}

fn is_escapable(chr: char) -> bool {
    match chr {
        'n' | 't' | 'r' | '\\' | '0' | 'x' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_process_escapes {
        use super::*;

        #[test]
        fn test_newline() {
            assert_eq!(
                String::from("hello\nworld"),
                escape_ascii(r#"hello\nworld"#).unwrap()
            );
        }

        #[test]
        fn test_carriage_return() {
            assert_eq!(
                String::from("hello\rworld"),
                escape_ascii(r#"hello\rworld"#).unwrap()
            );
        }

        #[test]
        fn test_tab() {
            assert_eq!(
                String::from("hello\tworld"),
                escape_ascii(r#"hello\tworld"#).unwrap()
            );
        }

        #[test]
        fn test_backslash() {
            assert_eq!(
                String::from("hello\\world"),
                escape_ascii(r#"hello\\world"#).unwrap()
            );
        }

        #[test]
        fn test_null() {
            assert_eq!(
                String::from("hello\0world"),
                escape_ascii(r#"hello\0world"#).unwrap()
            );
        }

        #[test]
        fn test_ascii_byte() {
            assert_eq!(
                String::from("hello\x20world"),
                escape_ascii(r#"hello\x20world"#).unwrap()
            );
        }
    }

    mod test_char_to_escape_sequence {
        use super::*;
        #[test]
        fn test_escape_n() {
            assert_eq!('\n', char_to_escape_sequence('n'));
        }

        #[test]
        fn test_escape_t() {
            assert_eq!('\t', char_to_escape_sequence('t'));
        }

        #[test]
        fn test_escape_r() {
            assert_eq!('\r', char_to_escape_sequence('r'));
        }

        #[test]
        fn test_escape_backslash() {
            assert_eq!('\\', char_to_escape_sequence('\\'));
        }

        #[test]
        fn test_escape_0() {
            assert_eq!('\0', char_to_escape_sequence('0'));
        }

        #[test]
        fn test_esacpe_x() {
            assert_eq!('\x7f', char_to_escape_sequence(0x7f as char));
        }
    }

    mod is_escapable_tests {
        use super::*;

        #[test]
        fn test_escape_n() {
            assert!(is_escapable('n'));
        }

        #[test]
        fn test_escape_t() {
            assert!(is_escapable('t'));
        }

        #[test]
        fn test_escape_r() {
            assert!(is_escapable('r'));
        }

        #[test]
        fn test_escape_backslash() {
            assert!(is_escapable('\\'));
        }

        #[test]
        fn test_escape_0() {
            assert!(is_escapable('0'));
        }

        #[test]
        fn test_escape_x() {
            assert!(is_escapable('x'));
        }
    }
}
