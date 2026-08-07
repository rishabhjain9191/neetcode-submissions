impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
let len = nums.len();
    
    // calculate prefix and suffix array
    let mut prefix: Vec<i32> = vec![1;len];
    let mut suffix: Vec<i32> = vec![1;len];
    let mut result: Vec<i32> = vec![1;len];
    
    
    for i in 1..nums.len() {
        prefix[i] = prefix[i - 1] * nums[i - 1];
    }
    for i in (0..len-1).rev() {
        suffix[i] = suffix[i + 1] * nums[i+1];
    }
    
    for i in 0..len {
        result[i] = prefix[i] * suffix[i];
    }
    result
}
}
