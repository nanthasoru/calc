use core::f64;
use std::process::exit;

fn evalpriority(operator: char) -> i8 {
    match operator {
        '*' | '/' | '%' => 2,
        '+' | '-' => 1,
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

fn s_pop(stack: &mut Vec<f64>, value_if_none: f64) -> f64 {
    match stack.pop() {
        Some(n) => n,
        None => value_if_none,
    }
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
            if let Some(last_symbol) = postfix_notation.chars().last()
                && !last_symbol.is_whitespace()
                && !last_symbol.is_digit(10)
            {
                postfix_notation.push(' ');
            }

            if symbol == ' ' {
                continue;
            } else if symbol == '(' {
                operators.push(symbol);
            } else if symbol == ')' {
                while !operators.is_empty() && operators.last() != Some(&'(') {
                    postfix_notation.push(' ');
                    postfix_notation.push(operators.pop().unwrap());
                    operators.pop();
                }
                operators.pop();
            } else if symbol.is_digit(10) {
                postfix_notation.push(symbol);
            } else {
                postfix_notation.push(' ');
                while operators.len() > 0
                    && evalpriority(symbol) <= evalpriority(*operators.last().unwrap())
                {
                    postfix_notation.push(operators.pop().unwrap());
                    postfix_notation.push(' ');
                }
                operators.push(symbol);
            }
        }

        while operators.len() > 0 {
            postfix_notation.push(' ');
            postfix_notation.push(operators.pop().unwrap());
        }

        Postfix {
            content: postfix_notation.replace("  ", " "),
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

        if let Some(first) = parts.get(0)
            && first.is_empty()
        {
            parts.remove(0);
        }

        println!("{:?}", parts);

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
                "+" => s_pop(&mut numbers, 0.0) + s_pop(&mut numbers, 0.0),
                "-" => -s_pop(&mut numbers, 0.0) + s_pop(&mut numbers, 0.0),
                "*" => s_pop(&mut numbers, 1.0) * s_pop(&mut numbers, 1.0),
                "/" => {
                    let n = s_pop(&mut numbers, 1.0);
                    s_pop(&mut numbers, 1.0) / n
                }
                "%" => {
                    let n = s_pop(&mut numbers, 1.0);
                    s_pop(&mut numbers, 1.0) % n
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
    fn to_postfix_test_4() {
        to_postfix_skeleton(
            "4/1 - 4/3 + 4/5 - 4/7 + 4/9 - 4/11",
            "4 1 / 4 3 / - 4 5 / + 4 7 / - 4 9 / + 4 11 / -",
        );
    }

    #[test]
    fn to_postfix_test_5() {
        to_postfix_skeleton(
            "(4/1) - (4/3) + (4/5) - (4/7) + (4/9) - (4/11)",
            "4 1 / 4 3 / - 4 5 / + 4 7 / - 4 9 / + 4 11 / -",
        );
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

    #[test]
    fn parenthesis_test_1() {
        assert_eq!(
            Infix::new(&"(4/1) - (4/3) + (4/5) - (4/7) + (4/9) - (4/11)".to_string())
                .to_postfix()
                .value(),
            2.9760461760461765
        )
    }
}
