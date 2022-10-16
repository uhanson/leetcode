pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    fn used_mask(r: usize, c: usize) -> u64 {
        1u64 << (r * 6 + c)
    }

    fn search(board: &Vec<Vec<char>>, r: usize, c: usize, used: u64, word: &[u8]) -> bool {
        if used & used_mask(r, c) > 0 {
            false
        } else if board[r][c] as u8 == word[0] {
            if word.len() == 1 {
                true
            } else {
                let used = used | used_mask(r, c);

                (r > 0 && search(board, r - 1, c, used, &word[1..])) ||
                (c > 0 && search(board, r, c - 1, used, &word[1..])) ||
                (r < board.len() - 1 && search(board, r + 1, c, used, &word[1..])) ||
                (c < board[0].len() - 1 && search(board, r, c + 1, used, &word[1..]))
            }
        } else {
            false
        }
    }

    let word = word.as_bytes();

    for (r, row) in board.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch as u8 == word[0] {
                if search(&board, r, c, 0u64, &word) {
                    return true;
                }
            }
        }
    }

    false
}


fn main() {
    println!("{}", exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCCED".to_string()));
}
