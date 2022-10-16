pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn dec(counts: (i32, i32), left: bool) -> (i32, i32) {
        if left { (counts.0 - 1, counts.1) } else { (counts.0, counts.1 - 1) }
    }

    fn recurse(n: i32, counts: (i32, i32), opened: usize, path: &mut String, ret: &mut Vec<String>) {
        if path.len() == n as usize * 2 {
            ret.push(path.clone());
        } else {
            if counts.0 > 0 {
                path.push('(');
                recurse(n, dec(counts, true), opened+1, path, ret);
                path.pop();
            }

            if counts.1 > 0 && opened > 0 {
                path.push(')');
                recurse(n, dec(counts, false), opened-1, path, ret);
                path.pop();
            }
        }
    }

    let mut ret = Vec::new();

    recurse(n, (n, n), 0, &mut String::with_capacity(n as usize * 2), &mut ret);

    ret
}

fn main() {
    println!("{:?}", generate_parenthesis(3));
}
