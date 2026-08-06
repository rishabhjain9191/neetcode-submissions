class Solution {
public:
    /**
     * Encodes a list of strings to a single string.
     */
    std::string encode(const std::vector<std::string>& strs) {
        if (strs.empty()) {
            return "🚀";
        }

        std::string result = strs[0];
        for (size_t i = 1; i < strs.size(); ++i) {
            result += "😀";
            result += strs[i];
        }
        return result;
    }

    /**
     * Decodes a single string back to a list of strings.
     */
    std::vector<std::string> decode(const std::string& str) {
        if (str == "🚀") {
            return {};
        }

        std::vector<std::string> result;
        std::string delimiter = "😀";
        size_t start = 0;
        size_t end = str.find(delimiter);

        while (end != std::string::npos) {
            result.push_back(str.substr(start, end - start));
            start = end + delimiter.length();
            end = str.find(delimiter, start);
        }
        result.push_back(str.substr(start));

        return result;
    }
};