class Solution {
    /**
     * @param {number[]} nums
     * @return {boolean}
     */
    hasDuplicate(nums) {
        const count = {}
        for(let index in nums) {
            console.log(index);
            let num = nums[index];
            count[num] = count[num] || 0;
            count[num]++;
            if(count[num] == 2) {
                return true;
            }
        }
        return false;
    }
}
