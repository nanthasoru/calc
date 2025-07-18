mod parser;

use std::io::Write;

use parser::*;

fn eval(expression: String) {
    if !parser::is_correctly_wrapped(&expression) {
        eprintln!("Your parenthesis look weird...");
        return;
    }

    if expression
        .replace("(", "")
        .replace(")", "")
        .trim()
        .is_empty()
    {
        eprintln!("Your expression looks weird...");
        return;
    }

    let infix = Infix::new(&expression.trim().to_string());
    println!("{}", infix.to_postfix().value())
}

fn inf_calc() {
    loop {
        let mut input = String::new();
        if input == "quit" {
            break;
        }

        print!("> ");

        match std::io::stdout().flush() {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Failed to flush stdout");
                std::process::exit(1);
            }
        }

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        eval(input);
    }
}

fn flatten(args: Vec<String>) -> String {
    let mut res = String::new();

    for part in args {
        res.push_str(&format!("{} ", part).as_str());
    }

    res
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    args.remove(0);

    if args.len() == 0 {
        inf_calc();
    } else {
        eval(flatten(args));
    }
}
