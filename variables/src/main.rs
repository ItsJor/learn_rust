use std::io;

fn main() {
   
    // program to ask a user to type a number in seconds and convert it to minutes, hours and seconds
    // will ask for another number, all until the user types q
    // 9876 to 2 hours, 44 minutes and 36 seconds

    // constants: define seconds per minute and seconds per hour as constants with type annotation
    // write hours as an expression rather than in seconds

    // shadowing: read the input into a string, then shadow the same number to trim it then shadow it again to parse in into a number
    // use the same variable name the whole way, no input_str/input_num

    // mut: a running count of conversions and a running total of seconds, both declared with let mut and updated in the loop

    // immutability - everything else should be plain let 

    const SECONDS_PER_MINUTE: u32 = 60;
    const SECONDS_PER_HOUR: u32 = 60 * SECONDS_PER_MINUTE;
    

    loop {

        let mut user_number = String::new(); 

        println!("Please enter a number: ");

        io::stdin()
            .read_line(&mut user_number)
            .expect("Failed to read line");

        println!("Your number is: {}", user_number.trim());

        if user_number.trim() == "q" {
            break;
        };

        let in_hours: u32 = user_number.trim().parse::<u32>().unwrap() / SECONDS_PER_HOUR;
        println!("Your number in hours is: {}", in_hours);

        let in_minutes: u32 = user_number.trim().parse::<u32>().unwrap() % SECONDS_PER_HOUR / SECONDS_PER_MINUTE;
        println!("Your number in minutes is: {}", in_minutes);

        let in_seconds: u32 = user_number.trim().parse::<u32>().unwrap() % SECONDS_PER_MINUTE;
        println!("Your number in seconds is: {}", in_seconds);

    }

}
