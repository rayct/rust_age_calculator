
## Here's an example program in Rust that calculates a person's age and the number of days until their next birthday:

## This program will prompt the user for their date of birth, and then it will calculate current age, the day of the week you were born on, and the number of days until your next birthday.

## To use the program, simply run it in a IDE or code editor environment and follow the prompts.

# Version: 1.0.1
## Here's an optimized version of the Rust program:
## This version uses the chrono library to handle date/time calculations, which simplifies the code significantly. It also eliminates the need for the std::str::FromStr trait, as chrono provides its own parsing functions. Finally, it combines the two functions for calculating age and days until next birthday into one main function for readability.

# Version: 1.0
## This program prompts the user to input their birthdate in the format "DD/MM/YYYY", calculates their age in years, and then calculates the number of days until their next birthday. It uses the Rust standard library's io module for input/output and the chrono crate for date/time calculations 

## Ray C. Turner