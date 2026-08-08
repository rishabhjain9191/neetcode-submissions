impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
let numbers = nums;
    let mut result = 0;
    
    let num_set: HashSet<i32> = numbers.into_iter().collect();
    let mut past_sequences: HashMap<i32, i32> = HashMap::new();
    
    let mut seen_numbers: HashSet<i32> = HashSet::new();
    
    for &num in &num_set {
        //println!("number into consideration = {}", &num);
        // if(seen_numbers.contains(&num)) {
        //     continue;
        // }
        // number in action = num;
        let mut find_next: bool = true;
        let mut next_num = num + 1;
        let mut seq_length = 1;
        seen_numbers.insert(num);
        
        // next is already calculated
        if past_sequences.contains_key(&next_num) {
          //  println!("taking if branch: next_num = {}", &next_num);
            seq_length = 1 + past_sequences.get(&next_num).unwrap();
        }
        
        else {
            // println!("taking else branch for {}, next_num = {}", &num, &next_num);
            // find for this
            while find_next {
               //  println!("starting while: {}, seq_length={} ",next_num, seq_length);
                 if past_sequences.contains_key(&next_num) {
            // println!("found past sequence inside while loop");
            seq_length += past_sequences.get(&next_num).unwrap();
            find_next = false;
            // println!("inserting into past_sequences:{}, {}", num, seq_length);
            past_sequences.insert(num, seq_length);
            // println!("{:?}", past_sequences);
            break;
        }
        
                if num_set.contains(&next_num) {
                    // this is a good number
                    seq_length += 1;
                    // mark this number
                    seen_numbers.insert(next_num);
                    next_num = next_num + 1;
                    
                }
                else {
                    find_next = false;
                }
            }
            }
            // println!("inserting into past_sequences:{}, {}", num, seq_length);
            past_sequences.insert(num, seq_length);
             //println!("{:?}", past_sequences);
            
        
        
    }
    // println!("{:?}", num_set);
    // println!("{:?}", past_sequences);

    *past_sequences.values().max().unwrap_or(&0)
    }
}
