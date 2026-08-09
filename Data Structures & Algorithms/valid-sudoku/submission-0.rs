impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // take hashsets
    let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut columns: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    
    for i in 0..9 {
        for j in 0..9 {
            let value = board[i][j];
            if value == '.' {
                continue;
            }
            let box_idx = (i/3) * 3 +(j/3);
            
            if(!rows[i].insert(value) || !columns[j].insert(value) || !boxes[box_idx].insert(value)) {
                return false;
            }
        }
    }
    true
    }
}
