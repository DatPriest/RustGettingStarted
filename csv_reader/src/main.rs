use std::io;

fn main() {
    let array : [char; 4] = ['+', '-', '/', '*'];
    println!("Welcome to the calculator");
    println!("Choose between '+' '-' '/' '*'");
    start(array);
}

fn start(array : [char; 4]) {
    println!("Please insert your Input for the calculator");
    loop {
        let input = reading_line();
        if array.contains(&input) {
            println!("Got it");
            matching(input);
        } else if input == 'q'{
            println!("Quitting!");
            break;
        }
        else {
            println!("Please enter a operator!");
            continue;
        }
    }
}

fn reading_line() -> char {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read Line");

    let input : char = match input.trim().parse::<char>() {
        Ok(chars) => chars,
        Err(_a) => {
            println!("Please enter a operator! {0}", _a);
            return reading_line();
        }
    };
    return input;
}

fn matching(input : char) {
    match input {
        '+' | '-' | '/' | '*' => calculation(input),
        _ => println!("Not a function"),
    };
}

fn calculate(vector : Vec<i32>, calculation_type : char) {
    let mut sum : i32 = vector[0];
    let iterator = vector.iter();
    let mut i : u32 = 0;
    for val in iterator {
        if i == 0 {
            i += 1;
            continue;
        }
        match calculation_type {
            '+' => sum += val,
            '-' => sum -= val,
            '/' => sum /= val,
            '*' => sum *= val,
            _ => println!("Not a operator!")
        };
    }
    println!();
    println!("This is your result: {0}", sum);
}

fn calculation(input : char) {
    println!("Welcome to Addition, please Enter a number;");
    let mut vector: Vec<i32> = Vec::new();
    loop {
        let mut second_input = String::new();
        io::stdin()
            .read_line(&mut second_input)
            .expect("Failed to read Line");
        if second_input.trim() == "=" {
            calculate(vector, input);
            break;
        } else {
            match second_input.trim().parse::<i32>() {
                Ok(_number) =>  {
                    vector.push(_number);
                    println!("Type another number or = to sum");    
                },
                Err(_err) => {
                    println!("Please enter a number! {0}", _err);
                    continue;
                }
            }; 
        };

    }
}