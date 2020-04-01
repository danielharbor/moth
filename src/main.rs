pub mod first;
pub mod second;
pub mod third;
pub mod palindrome;

use palindrome::is_palindrome;

fn main() {
    let words = ["racecar", "madam", "badboy"];

    for word in &words {
        println!("Is \"{}\" a palindrome: {}", word, is_palindrome(word));
    }
}
