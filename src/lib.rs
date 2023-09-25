pub struct Parser;

impl Parser {
    /// Given a string `s` representing a valid expression, implement a parser to take a string and compute its numerical value using the given rules.
    /// Operators should be applied in order of precedence from left to right. An exception to this is brackets which are used to explicitly denote precedence by grouping parts of an expression that should be evaluated first.
    ///
    /// Rules:
    /// a = `+`, b = `-`, c = `*`, d = `/`, e = `(`, f = `)`

    /// Time Complexity:  O(`len_s`)
    /// Space Complexity: O(`len_s`)
    pub fn calculate(s: String) -> i32 {
        let tokens = Self::infix_to_rpn(&s);
        let result = Self::eval_rpn(tokens);

        result
    }
}

impl Parser {
    const SIGN_PLUS: char = 'a';
    const SIGN_MINUS: char = 'b';
    const SIGN_MULTIPLY: char = 'c';
    const SIGN_DIVIDE: char = 'd';
    const PAREN_OPEN: char = 'e';
    const PAREN_CLOSED: char = 'f';

    fn is_operator(c: char) -> bool {
        c == Self::SIGN_PLUS
            || c == Self::SIGN_MINUS
            || c == Self::SIGN_MULTIPLY
            || c == Self::SIGN_DIVIDE
    }

    fn precedence(c: char) -> i32 {
        match c {
            Self::SIGN_PLUS | Self::SIGN_MINUS | Self::SIGN_MULTIPLY | Self::SIGN_DIVIDE => 1,
            _ => 0,
        }
    }

    fn infix_to_rpn(expression: &str) -> Vec<String> {
        let mut output_queue: Vec<String> = Vec::with_capacity(16);
        let mut operator_stack: Vec<char> = Vec::with_capacity(16);

        let mut num_buffer = String::with_capacity(16);

        for token in expression.chars() {
            if token.is_whitespace() {
                continue;
            }

            if token.is_digit(10) {
                num_buffer.push(token);
            } else if Self::is_operator(token) {
                if !num_buffer.is_empty() {
                    output_queue.push(num_buffer.clone());
                    num_buffer.clear();
                }

                while let Some(&top) = operator_stack.last() {
                    if top == Self::PAREN_OPEN || Self::precedence(top) < Self::precedence(token) {
                        break;
                    }
                    output_queue.push(operator_stack.pop().unwrap().to_string());
                }
                operator_stack.push(token);
            } else if token == Self::PAREN_OPEN {
                operator_stack.push(token);
            } else if token == Self::PAREN_CLOSED {
                if !num_buffer.is_empty() {
                    output_queue.push(num_buffer.clone());
                    num_buffer.clear();
                }
                while let Some(&top) = operator_stack.last() {
                    if top == Self::PAREN_OPEN {
                        break;
                    }
                    output_queue.push(operator_stack.pop().unwrap().to_string());
                }
                operator_stack.pop();
            }
        }

        if !num_buffer.is_empty() {
            output_queue.push(num_buffer);
        }

        while let Some(&_) = operator_stack.last() {
            output_queue.push(operator_stack.pop().unwrap().to_string());
        }

        output_queue
    }

    fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::with_capacity(tokens.len());

        for token in tokens {
            let sign = token.chars().nth(0).unwrap();
            match sign {
                Self::SIGN_PLUS | Self::SIGN_MINUS | Self::SIGN_MULTIPLY | Self::SIGN_DIVIDE => {
                    let right: i32 = stack.pop().unwrap();
                    let left: i32 = stack.pop().unwrap();

                    stack.push(match sign {
                        Self::SIGN_PLUS => left + right,
                        Self::SIGN_MINUS => left - right,
                        Self::SIGN_MULTIPLY => left * right,
                        Self::SIGN_DIVIDE => left / right,
                        _ => unreachable!(),
                    });
                }
                _ => stack.push(token.parse().unwrap()),
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let result = Parser::calculate("3b3a0".to_string());
        assert_eq!(result, 0);
    }

    #[test]
    fn acceptance_test_01() {
        let result = Parser::calculate("3a2c4".to_string());
        assert_eq!(result, 20);
    }

    #[test]
    fn acceptance_test_02() {
        let result = Parser::calculate("32a2d2".to_string());
        assert_eq!(result, 17);
    }

    #[test]
    fn acceptance_test_03() {
        let result = Parser::calculate("500a10b66c32".to_string());
        assert_eq!(result, 14208);
    }

    #[test]
    fn acceptance_test_04() {
        let result = Parser::calculate("3ae4c66fb32".to_string());
        assert_eq!(result, 235);
    }

    #[test]
    fn acceptance_test_05() {
        let result = Parser::calculate("3c4d2aee2a4c41fc4f".to_string());
        assert_eq!(result, 990);
    }
}
