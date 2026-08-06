class Solution {
    /**
     * @param {number[]} nums
     * @param {number} target
     * @return {number[]}
     */
    twoSum(nums, target) {
        const numbers = {};
        const diffs = {};

        numbers[nums[0]] = 0;
        let diff = target - nums[0];
        diffs[diff] = 0;

        for(let i = 1; i < nums.length; i++) {
            // the current number is equal to diff
            // so we found the pair
            if(diffs['' + nums[i]] !== undefined) {
                return [diffs[nums[i]], i];
            }
            // otherwise
            diff = target - nums[i];
            diffs[diff] = i;
        }
    }
}
