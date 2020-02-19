struct TreeNode {
    val: i32
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val: val
        }
    }
}

fn is_palindrome(word: &str) -> bool {
    let char_vec: Vec<char> = word.chars().collect();
    let word_len = char_vec.len();

    for i in 0..word_len {
        if char_vec[i] != char_vec[word_len-1-i] {
            return false;
        }
    }

    return true;
}

fn main() {
    let words = ["racecar", "madam", "badboy"];

    for word in &words {
        println!("Is \"{}\" a palindrome: {}", word, is_palindrome(word));
    }

    let t = TreeNode::new(29);
    println!("{}", t.val);
}
