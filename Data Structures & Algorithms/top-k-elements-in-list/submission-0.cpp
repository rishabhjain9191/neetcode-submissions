class Solution {
public:
    vector<int> topKFrequent(vector<int>& nums, int k) {
        // define a node
        struct Node {
            int number;
            int frequency;

            bool operator<(const Node& other) const {
                return frequency < other.frequency;
            }
        };
        // define a priority priority_queue of this struct
        std::priority_queue<Node> maxHeap;
        
        // take a map which will point to Node
        std::unordered_map<int, int> map;

        std::vector<std::pair<int, int>> pairsVec;

        // 2. Convert vector to Max Heap based on the VALUE (pair.second) in O(N) time
    auto compareByValue = [](const std::pair<int, int>& a, const std::pair<int, int>& b) {
        return a.second < b.second; // Creates a MAX heap for values
    };

        // iterate the numbers
        for(int i = 0; i < nums.size(); i++) {
            if(map.count(nums[i])) {
                //value exis in map
                // update the value in maxHead
                map[nums[i]] = map[nums[i]] + 1;
            }
            else {
               // put the value in map
                map[nums[i]] = 1;
            }
        }
        pairsVec.reserve(map.size());
        // get all the values from map
        for (const auto& pair : map) {
            pairsVec.push_back(pair);
        }

        // make heap O(N)
        std::make_heap(pairsVec.begin(), pairsVec.end(), compareByValue);

        vector<int> result;
        for(int i = 0; i < k; i++) {
            std::pop_heap(pairsVec.begin(), pairsVec.end(), compareByValue);
            auto top = pairsVec.back();
            pairsVec.pop_back();

            result.push_back(top.first);
        }

        return result;

    }
};
