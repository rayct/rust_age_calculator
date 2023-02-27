// Version: 1.0
// This program prompts the user to input their birthdate in the format "DD/MM/YYYY", calculates their age in years,
// and then calculates the number of days until their next birthday.
// It uses the Rust standard library's io module for input/output and the chrono crate for date/time calculations.

use std::io;

fn main() {
    println!("Enter your date of birth (format: DD/MM/YYYY):");
    let mut input_date = String::new();

    io::stdin().read_line(&mut input_date).expect("Failed to read input");

    let birthdate = match chrono::NaiveDate::parse_from_str(&input_date.trim(), "%d/%m/%Y") {
        Ok(date) => date,
        Err(_) => {
            println!("Invalid date format. Please enter date in the format DD/MM/YYYY.");
            return;
        }
    };

    let age = chrono::Utc::today().year() - birthdate.year();
    println!("You are {} years old.", age);

    let next_birthday = match chrono::NaiveDate::from_ymd(chrono::Local::now().year(), birthdate.month(), birthdate.day()) {
        date if date < chrono::Local::today().naive_local() => chrono::NaiveDate::from_ymd(chrono::Local::now().year() + 1, birthdate.month(), birthdate.day()),
        date => date,
    };

    let days_to_birthday = (next_birthday - chrono::Local::today().naive_local()).num_days();
    println!("There are {} days until your next birthday.", days_to_birthday);
}


// Version: 1.0.1
// Here's an optimized version of the Rust program:
// This version uses the chrono library to handle date/time calculations, which simplifies the code significantly.
// It also eliminates the need for the std::str::FromStr trait, as chrono provides its own parsing functions.
// Finally, it combines the two functions for calculating age and days until next birthday into one main function for readability.

use std::io;

fn main() {
    // Get user input for birthdate
    println!("Enter your date of birth (format: DD/MM/YYYY):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let birthdate = input.trim();

    // Calculate age in years
    let age = {
        let today = chrono::Utc::today();
        let birthdate = chrono::DateTime::parse_from_str(birthdate, "%d/%m/%Y")
            .expect("Invalid date format")
            .date();
        let age = today.signed_duration_since(birthdate);
        age.num_days() / 365
    };

    // Calculate days until next birthday
    let days_to_birthday = {
        let today = chrono::Utc::today();
        let birthdate = chrono::DateTime::parse_from_str(birthdate, "%d/%m/%Y")
            .expect("Invalid date format")
            .date();
        let mut next_birthday = birthdate.with_year(today.year()).unwrap();
        if next_birthday < today {
            next_birthday = birthdate.with_year(today.year() + 1).unwrap();
        }
        next_birthday.signed_duration_since(today).num_days()
    };

    // Output results
    println!("You are {} years old.", age);
    println!("There are {} days until your next birthday.", days_to_birthday);
}
