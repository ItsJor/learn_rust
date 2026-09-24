use std::io;

fn main() {
   
    // program to ask a user to type a number in seconds and convert it to minutes, hours and seconds
    // will ask for another number, all until the user types q

    // constants: define seconds per minute and seconds per hour as constants with type annotation
    // write hours as an expression rather than in seconds

    // shadowing: read the input into a string, then shadow the same number to trim it then shadow it again to parse in into a number
    // use the same variable name the whole way, no input_str/input_num

    // mut: a running count of conversions and a running total of seconds, both declared with let mut and updated in the loop

    // immutability - everything else should be plain let 

    let mut user_number = String::new();

    println!("Please enter a number: ");
    io::stdin()
    .read_line(&mut user_number) {
        Ok(n) => {
            println!("You entered: {}", user_number);
        }
        Err(error) => {
            println!("Error reading input: {}", error);
        }
    }
    .expect("Failed to read line");

}
