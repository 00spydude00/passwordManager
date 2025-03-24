use std::io::{stdin, Write}; // Stdin for user input and Write for writing to the output file
use std::fs::OpenOptions; // OpenOptions for file creating and modifying
use rand::Rng; // For random password
fn main() {
    fn append_to_file(password: String) {
        // Open file if it exists, Create if it doesn't.
        let result_file = OpenOptions::new().create(true).append(true).open("result.txt");  
        // Write password appended to the file. \n for different lines for each password.
        write!(result_file.expect("Something went wrong with creating/opening the file."), "{}\n", password).expect("Something went wrong while writing to the file.");
        println!("Your password is: {}", password);
    }
    println!("Welcome to the password manager!");
    let mut random_password_y_n = String::new();
    println!("Would you like to generate a random password? y/n:");
    stdin().read_line(&mut random_password_y_n).expect("Unable to read y/n user input.");
    if random_password_y_n.trim_end() == "y" { // If the user wants to have a random password
        println!("How long would you like your random password to be?");
        // Get length of password
        let mut length_of_password = String::new();
        stdin().read_line(&mut length_of_password).expect("Failed! you might not have put the wrong format. Try just using the number, no spaces or anything.");
        // Change the string input to integer
        let length_of_password = length_of_password.trim_end().parse::<i32>().unwrap();
        // Create the randomly generated password
        let mut password: String = String::new();
        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890`~!@#$%^&*()-_=+[{]}|;:',<.>/?"; // Randomly select from
        for _x in 0..length_of_password {
            // Add one random character at a time
            let character_to_add = characters.chars().nth(rand::rng().random_range(0..characters.len())).unwrap();
            password.push(character_to_add);
        }
        append_to_file(password);
    }
    else if random_password_y_n.trim_end() == "n" { // If the user wants to make their own password
        println!("What would you like your password to be?:"); 
        let mut password = String::new();
        stdin().read_line(&mut password).expect("Unable to set password variable to your input.");
        append_to_file(password);
    }
}
