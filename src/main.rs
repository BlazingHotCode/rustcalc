use std::io;

fn main() {
    loop {
        let input = read_input();

        if input == "exit" {
            break;
        }

        match rustcalc::parse(&input) {
            Ok(expr) => {
                println!("Parsed Expression: {:?}", expr);
                match rustcalc::eval_expression(&expr) {
                    Ok(value) => println!("Evaluated Expression: {}", value),
                    Err(err) => eprintln!("Error: {err}"),
                }
            }
            Err(err) => eprintln!("Error: {err}"),
        }
    }
}

fn read_input() -> String {
    println!();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
