use libm::{asin};

#[derive(PartialEq)] // needed for ==
enum Operation {
    Add,
    Subtract,
    Divide,
    Factorial,
    Arcsin,
    OperationThatSpamsTheNumberInYourConsole,
    Invalid
}

fn factorial(input: u8) -> Option<f64> {
    if input > 20 { return None; }
    else {
        let mut output: f64 = 1.;
        for i in 1..input {
            output *= i as f64;
        }
        Some(output)
    }
}

fn main() {
    println!("Welcome, surgeon!");
    println!("What operation would you like to do today? (Options: +, -, /, factorial, arcsin, operationthatspamsthenumberinyourconsole)");
    loop {
        print!("> ");
        // why tf does rust need a library to read text from the console :sob:
        let raw_operation: String = text_io::read!("{}\n");
        
        // there's probably a way better way to turn a string into an enum lol
        use Operation::*;
        let operation = match raw_operation.as_str() {
            "+" => Add,
            "-" => Subtract,
            "/" => Divide,
            "factorial" => Factorial,
            "arcsin" => Arcsin,
            "operationthatspamsthenumberinyourconsole" => OperationThatSpamsTheNumberInYourConsole,
            _ => Invalid
        };

        if operation == Invalid {
            println!("That's not an option!!! >:(\nfr tho what operation do you wanna do");
            continue;
        }

        loop {
            if operation == Arcsin || operation == Factorial || operation == OperationThatSpamsTheNumberInYourConsole {
                print!("put ur number here > ");
                let number: f64 = text_io::read!("{}\n");
                if (number > 1. || number < -1.) && operation == Arcsin {
                    println!("Must be between -1 and 1!");
                    continue;
                }
                if operation == OperationThatSpamsTheNumberInYourConsole {
                    for _ in 1..200000 { print!("{}", number); }
                    println!();
                    break;
                }
                let result = match operation {
                    Arcsin => Some(asin(number)),
                    Factorial => factorial(number as u8),
                    _ => None
                };
                match result {
                    Some(value) => println!("the answer... iS {}!!!!!", value),
                    None => println!("number must be <= 20!")
                }
            } else {
                print!("First number > ");
                let number_1: f64 = text_io::read!("{}\n");
                print!("second number > ");
                let number_2: f64 = text_io::read!("{}\n");

                println!("the answer........... IS {}", match operation {
                    Add => number_1 + number_2,
                    Subtract => number_1 - number_2,
                    Divide => number_1 / number_2,
                    _ => 0.
                })
            }

            break;
        }

        break;
    }
    println!("ok bye");
}
