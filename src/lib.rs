use std::collections::HashMap;

mod error;

use error::{Error, Result};

enum State {
    Init,
    Object,
    End,
}

pub fn parse(json: &str) -> Result<HashMap<String, String>> {
    let mut state = State::Init;

    if json.is_empty() {
        return Err(Error::Empty);
    }

    for c in json.chars() {
        match state {
            State::Init => match c {
                '{' => {
                    state = State::Object;
                }
                '}' => return Err(Error::Unmatched(c)),
                invalid => return Err(Error::Unrecognized(invalid)),
            },
            State::Object => match c {
                '}' => {
                    state = State::End;
                }
                invalid => return Err(Error::Unrecognized(invalid)),
            },

            State::End => return Err(Error::CharacterAfterEnd(c)),
        }
    }

    Ok(HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(parse("").unwrap_err(), Error::Empty);
    }

    #[test]
    fn unrecognized() {
        assert_eq!(parse("a").unwrap_err(), Error::Unrecognized('a'));
    }

    #[test]
    fn unmatched() {
        assert_eq!(parse("}").unwrap_err(), Error::Unmatched('}'));
    }

    #[test]
    fn empty_object() {
        assert_eq!(parse("{}").unwrap(), HashMap::new());
    }

    #[test]
    fn finished_object_then_another_char() {
        assert_eq!(parse("{}{").unwrap_err(), Error::CharacterAfterEnd('{'));
    }
}
