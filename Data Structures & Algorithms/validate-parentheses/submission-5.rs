impl Solution {
    pub fn is_valid(s: String) -> bool {
        // 1. Early exit for odd lengths
        if s.len() % 2 != 0 {
            return false;
        }

        // 2. Pre-allocate memory to avoid reallocations
        let mut stack = Vec::with_capacity(s.len() / 2);

        // 3. Iterate over raw bytes instead of UTF-8 characters
        for &b in s.as_bytes() {
            match b {
                b'(' | b'{' | b'[' => stack.push(b),
                b')' => if stack.pop() != Some(b'(') { return false; },
                b'}' => if stack.pop() != Some(b'{') { return false; },
                b']' => if stack.pop() != Some(b'[') { return false; },
                _ => (),
            }
        }
        
        stack.is_empty()
    }
}