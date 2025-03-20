use std::io::stdin;
use rand::Rng;
fn main() {
    println!("How long would you like your password to be?");
    let mut length_of_password_str = String::new();
    stdin().read_line(&mut length_of_password_str).expect("Failed! you might not have put the wrong format. Try just using the number, no spaces or anything.");
    let length_of_password = length_of_password_str.trim_end().parse::<i32>().unwrap();
    let mut password: String = String::new();
    let characters = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890`~-_=+;:'?.>,<";
    for _x in 0..length_of_password {
        let character_to_add = characters.chars().nth(rand::rng().random_range(0..characters.len())).unwrap();
        password.push(character_to_add);
    }
    println!("{}", password);
}
