//!
//! Creates longest possible substring with valid brackets of given infinite string using its
//! characters.
//!

///
/// Checks if provided char is a bracket.
///
fn is_bracket(val: char) -> bool {
    match val {
        '{' | '[' | '(' | ')' | ']' | '}' => true,
        _ => false,
    }
}

///
/// Attempts to map given bracket to its closing pair. Returns `None` if given char
/// isn't opening bracket.
///
fn opening_bracket_to_closing(val: char) -> Option<char> {
    match val {
        '{' => Some('}'),
        '[' => Some(']'),
        '(' => Some(')'),
        _ => None,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Encoded char size is greater than 1 byte
    NonByteChar,
}

///
/// Produces longest substring with valid brackets of infinite string `val` using its
/// characters. If string is infinite, returns "Infinite".
///
/// Returns `Err` in case if string contains char encoded with size greater than one byte.
///
/// Time complexity: O(n)
/// Space complexity: O(n)
///
pub fn create_longest_substring(val: &str) -> Result<String, Error> {
    #[derive(Debug, Copy, Clone)]
    struct CharPos {
        val: char,
        index: usize,
    }
    let mut brackets = Vec::<CharPos>::new();

    let mut max_end = 0;
    let mut max_len = 0;

    // Length of valid sequential substring predecessor
    let mut prev_valid_len = 0;

    for (index, char) in val.chars().cycle().take(2 * val.len()).enumerate() {
        if char.len_utf8() > 1 || char.len_utf16() > 1 {
            // Encoded char size is greater than one byte
            return Err(Error::NonByteChar);
        } else if let Some(len) = if is_bracket(char) {
            if let Some(bracket) = opening_bracket_to_closing(char) {
                if index >= val.len()
                    && brackets
                        .first()
                        .map(|ch| ch.index == index - val.len())
                        .unwrap_or(false)
                    || brackets.len() + 1 == val.len()
                {
                    // Break loop because longest subsequence either already found or 0
                    break;
                } else {
                    brackets.push(CharPos {
                        val: bracket,
                        index,
                    });

                    None
                }
            } else {
                match brackets.pop().and_then(|last| {
                    if last.val == char {
                        brackets
                            .last()
                            // Need to also capture characters between previous and last
                            .map(|prev| index - prev.index)
                            .or_else(|| Some(1 + index - last.index + prev_valid_len))
                    } else {
                        None
                    }
                }) {
                    // Reset brackets and prev_valid_len because current sequence is invalid
                    None => {
                        prev_valid_len = 0;
                        brackets.truncate(0);

                        // If end of the string is reached, no need to go further
                        if index >= val.len() {
                            break;
                        }

                        None
                    }
                    v => v,
                }
            }
        } else {
            brackets
                .last()
                // Calculate distance between current character and last bracket in brackets
                .map(|prev| index - prev.index)
                .or_else(|| Some(prev_valid_len + 1))
        } {
            if len > max_len {
                if len >= val.len() {
                    return Ok("Infinite".to_owned());
                }
                max_len = len;
                max_end = index + 1;
            }

            if brackets.is_empty() {
                prev_valid_len = len;
            }
        }
    }

    Ok(if max_end > val.len() {
        format!(
            "{}{}",
            &val[max_end - max_len..val.len()],
            &val[0..max_end - val.len()]
        )
    } else {
        val[max_end - max_len..max_end].to_owned()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(create_longest_substring("").unwrap(), "");
        assert_eq!(create_longest_substring("(").unwrap(), "");
        assert_eq!(create_longest_substring("(})").unwrap(), "");
        assert_eq!(create_longest_substring("([)]]})").unwrap(), "");
        assert_eq!(create_longest_substring("(((").unwrap(), "");
    }

    #[test]
    fn without_brackets() {
        assert_eq!(create_longest_substring("abc").unwrap(), "Infinite");
        assert_eq!(create_longest_substring("pasd").unwrap(), "Infinite");
        assert_eq!(create_longest_substring("zxc").unwrap(), "Infinite");
    }

    #[test]
    fn with_equal_brackets() {
        assert_eq!(create_longest_substring("(a(b)c)").unwrap(), "Infinite");
        assert_eq!(create_longest_substring("{(a[b]c)}").unwrap(), "Infinite");
        assert_eq!(create_longest_substring("a)b)(c(d").unwrap(), "Infinite");
        assert_eq!(
            create_longest_substring("[[g][f]]d))j}{k}{(l(").unwrap(),
            "Infinite"
        );
        assert_eq!(
            create_longest_substring(")p)}{q}i{((x[[]z[]y]o").unwrap(),
            "Infinite"
        );
        assert_eq!(create_longest_substring("q))]w[e((r").unwrap(), "Infinite");
    }

    #[test]
    fn finite() {
        assert_eq!(create_longest_substring("))[((").unwrap(), "(())");
        assert_eq!(create_longest_substring("])}([{}").unwrap(), "([{}])");
        assert_eq!(create_longest_substring(")}([{}]").unwrap(), "([{}])");
        assert_eq!(
            create_longest_substring("])}b(a[{efg}").unwrap(),
            "b(a[{efg}])"
        );
        assert_eq!(
            create_longest_substring(")}(m[{o}]oops").unwrap(),
            "(m[{o}]oops)"
        );
        assert_eq!(create_longest_substring("}}}a(((").unwrap(), "a");
        assert_eq!(create_longest_substring("(a(b(d").unwrap(), "a");
        assert_eq!(create_longest_substring("(a(bc(d").unwrap(), "bc");
        assert_eq!(create_longest_substring("ab()(d").unwrap(), "dab()");
        assert_eq!(
            create_longest_substring("ab()]abc()(}}dr").unwrap(),
            "drab()"
        );
        assert_eq!(
            create_longest_substring("(aaaaaaabbbbbcccccc").unwrap(),
            "aaaaaaabbbbbcccccc"
        );
        assert_eq!(
            create_longest_substring(")aaaaaaabbbbbcccccc").unwrap(),
            "aaaaaaabbbbbcccccc"
        );
    }

    #[test]
    fn invalid() {
        assert_eq!(
            create_longest_substring("(🖐)){✊}").unwrap_err(),
            Error::NonByteChar
        );
        assert_eq!(
            create_longest_substring("(🖐){✊}").unwrap_err(),
            Error::NonByteChar
        );
        assert_eq!(
            create_longest_substring("🦅").unwrap_err(),
            Error::NonByteChar
        );
    }
}
