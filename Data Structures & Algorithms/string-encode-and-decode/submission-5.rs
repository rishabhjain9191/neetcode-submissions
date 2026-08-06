impl Solution {
    /// Encodes a list of strings to a single string.
    pub fn encode(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "🚀".to_string();
        }
        strs.join("😀")
    }

    /// Decodes a single string back to a list of strings.
    pub fn decode(str: String) -> Vec<String> {
        if str == "🚀" {
            return Vec::new();
        }
        str.split("😀")
           .map(|s| s.to_string())
           .collect()
    }
}