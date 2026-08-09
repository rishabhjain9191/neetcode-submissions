impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new(); 
        
        for ch in s.chars() {
            match ch {
                // 1. If it's an opening bracket, push to the stack
                '(' | '{' | '[' => stack.push(ch),
                
                // 2. If it's a closing bracket, pop the stack and check for a match
                ')' => if stack.pop() != Some('(') { return false; },
                '}' => if stack.pop() != Some('{') { return false; },
                ']' => if stack.pop() != Some('[') { return false; },
                
                // 3. Ignore anything else (optional, but good practice)
                _ => (), 
            }
        }
        
        // 4. If the stack is empty at the end, all brackets were matched!
        stack.is_empty()
    }
}