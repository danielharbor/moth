pub fn is_palindrome(word: &str) -> bool {
    let char_vec: Vec<char> = word.chars().collect();
    let word_len = char_vec.len();

    for i in 0..word_len {
        if char_vec[i] != char_vec[word_len-1-i] {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use super::is_palindrome;

    #[test]
    fn is_palindrome_test() {
        let a = "madam";
        let b = "abcd";
        assert_eq!(is_palindrome(&a), true);
        assert_eq!(is_palindrome(&b), false);
    }
}
