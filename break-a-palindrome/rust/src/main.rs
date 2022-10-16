pub fn break_palindrome(palindrome: String) -> String {
    if palindrome.len() > 1 {
        let mut ret = String::with_capacity(palindrome.len()); 
        let mut replaced = false;

        let half = palindrome.len() / 2;
        let mid = palindrome.len() % 2;
        
        for (ix, c) in palindrome.chars().enumerate() {
            if replaced {
                ret.push(c);
            } else {
                replaced = true;
                if ix < half && c > 'a' {
                    ret.push('a');
                } else if ix == palindrome.len() - 1 && c == 'a' {
                    ret.push('b');
                } else if ix >= half + mid && c > 'a' {
                    ret.push('a');
                } else {
                    ret.push(c);
                    replaced = false;
                }
            }
        }

        if replaced { ret } else { "".to_string() }
    } else {
        "".to_string()
    }
}

fn main() {
    println!("{}", break_palindrome("aba".to_string()));
}
