use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); 

    let notation = &args[1];

    match do_calc(notation) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Invalid expression or division by zero."),
    }
}

fn do_calc(notation: &String) -> Option<u8> {
    let mut stack: Vec<u8> = Vec::new();

    for char in notation.chars() {
        if char.is_numeric() {
            stack.push(char as u8 - '0' as u8);
            continue;
        }
        match char {
            '+' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(left + right);
            }
            '-' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(left - right);
            }
            '*' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                stack.push(left * right);
            }
            '/' => {
                let right = stack.pop()?;
                let left = stack.pop()?;
                if right == 0 { return None; } // Handle division by zero!
                stack.push(left / right);
            }
            _ => return None,
        }
    }

    return stack.pop();
}
