impl Solution {
    pub fn is_valid(s: String) -> bool {
                    
       let mut stack: Vec<char> = Vec::new();
    
    let opening: HashMap<char, char> = HashMap::from([
        (')', '('),
        ('}', '{'),
        (']', '['),
    ]);
    
    println!("{:?}", opening);
    
    for ch in s.chars() {
        // if opening brace, push into the stack
        if ch == '(' || ch == '{' || ch == '[' {
            stack.push(ch);
        }
        //closing brace
        else {
           // get the top element
           //stack.push(ch);
           if let Some(top) = stack.last() {
                if let Some(opening_bracket) = opening.get(&ch) {
                    if let Some(top_value) = stack.last() {
                            if(top_value == opening_bracket) {
                                stack.pop();    
                                //stack.pop();
                            }
                            else {
            println!("invalid");
            return false;
        }
                            
                            
                            
                        }
                }
           }
            else {
            println!("invalid");
            return false;
        }
           
        }
       
    }
    
    println!("{:?}", stack);
    
    if stack.is_empty() {
        return true;
    }
    false
}
}

