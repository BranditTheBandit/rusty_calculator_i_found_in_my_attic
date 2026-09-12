#[derive(PartialEq)]
enum Operation {
    Add,
    Subtract,
    Divide,
    LogBase124,
    Arcsin,
    Invalid
}

fn main() {
    println!("Welcome, surgeon!");
    println!("What operation would you like to do today? (Options: +, -, /, logbase124, arcsin)");
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
            "logbase124" => LogBase124,
            "arcsin" => Arcsin,
            _ => Invalid
        };

        if operation == Invalid {
            println!("That's not an option!!! >:(\nfr tho what operation do you wanna do");
        } else {
            println!("valid option! no way!");
            break;
        }
    }
    println!("ok bye");
}
