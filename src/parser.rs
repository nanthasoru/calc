use core::f64;
use std::process::exit;

fn evalpriority(operator: char) -> u8 {
    match operator {
        '*' | '/' | '%' => 1,
        _ => 0,
    }
}

pub fn is_correctly_wrapped(expression: &String) -> bool {
    let mut counter = 0;

    for symbol in expression.chars() {
        match symbol {
            '(' => counter += 1,
            ')' => {
                counter -= 1;
                if counter < 0 {
                    return false;
                }
            }
            _ => continue,
        }
    }

    counter == 0
}

pub fn is_number(s: &str) -> bool {
    for val in s.chars() {
        if !val.is_digit(10) {
            return false;
        }
    }
    true
}

pub struct Infix {
    content: String,
}

impl Infix {
    pub fn new(expression: &String) -> Self {
        Infix {
            content: expression.clone(),
        }
    }

    pub fn to_postfix(&self) -> Postfix {
        let mut postfix_notation: String = String::new();
        let mut operators: Vec<char> = Vec::new();

        for symbol in self.content.chars() {
            match symbol {
                ' ' => continue,
                '(' => operators.push(symbol),
                ')' => {
                    while !operators.is_empty() && operators.last() != Some(&'(') {
                        postfix_notation.push(' ');
                        postfix_notation.push(operators.pop().expect(""));
                    }
                    operators.pop();
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    postfix_notation.push(symbol);
                }
                '+' | '-' | '*' | '/' | '%' => {
                    let previous_operator_priority = match operators.last() {
                        Some(operator) => evalpriority(*operator),
                        None => 0,
                    };

                    let current_operator_priority = evalpriority(symbol);

                    postfix_notation.push(' ');

                    if previous_operator_priority > current_operator_priority {
                        postfix_notation.push(operators.pop().expect(""));
                        postfix_notation.push(' ');
                        match operators.pop() {
                            Some(chr) => {
                                postfix_notation.push(chr);
                                postfix_notation.push(' ');
                            }
                            None => {}
                        }
                    }

                    operators.push(symbol);
                }
                _ => {
                    eprintln!("Unknown symbol '{}' in '{}'", symbol, self.content);
                    exit(0);
                }
            }
        }

        while !operators.is_empty() {
            postfix_notation.push(' ');
            postfix_notation.push(operators.pop().expect(""));
        }

        Postfix {
            content: postfix_notation,
        }
    }
}

pub struct Postfix {
    pub content: String,
}

impl Postfix {
    pub fn value(&self) -> f64 {
        let mut numbers: Vec<f64> = Vec::new();
        let mut parts: Vec<&str> = self.content.split(" ").collect();

        for part in parts {
            if is_number(part) {
                let n: f64 = part
                    .trim()
                    .parse()
                    .expect(&format!("Failed to parse item '{}'", part));

                numbers.push(n);
                continue;
            }

            let n = match part {
                "+" => numbers.pop().expect("") + numbers.pop().expect(""),
                "-" => -numbers.pop().expect("") + numbers.pop().expect(""),
                "*" => numbers.pop().expect("") * numbers.pop().expect(""),
                "/" => {
                    let n = numbers.pop().expect("");
                    numbers.pop().expect("") / n
                }
                "%" => {
                    let n = numbers.pop().expect("");
                    numbers.pop().expect("") % n
                }
                _ => {
                    eprintln!("Unknown part '{}'", part);
                    exit(1);
                }
            };

            numbers.push(n);
        }

        match numbers.pop() {
            Some(num) => num,
            None => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_postfix_skeleton(infix_expression: &str, postfix_expression: &str) {
        let infix = Infix::new(&infix_expression.to_string());
        let postfix = infix.to_postfix();

        assert_eq!(postfix.content, postfix_expression,);
    }

    #[test]
    fn to_postfix_test_1() {
        to_postfix_skeleton("10*3 + 25", "10 3 * 25 +");
    }

    #[test]
    fn to_postfix_test_2() {
        to_postfix_skeleton("1+2*2+3", "1 2 2 * + 3 +");
    }

    #[test]
    fn to_postfix_test_3() {
        to_postfix_skeleton("1+2", "1 2 +");
    }

    #[test]
    fn wrapper_test_1() {
        assert!(is_correctly_wrapped(&"(1+2)*3".to_string()));
    }

    #[test]
    fn wrapper_test_2() {
        assert!(is_correctly_wrapped(&"(())".to_string()));
    }

    #[test]
    fn wrapper_test_3() {
        assert!(!is_correctly_wrapped(&"()(".to_string()));
    }

    #[test]
    fn wrapper_test_4() {
        assert!(!is_correctly_wrapped(&"))((".to_string()));
    }
}
