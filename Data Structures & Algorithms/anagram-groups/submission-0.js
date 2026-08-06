class Solution {
    /**
     * @param {string[]} strs
     * @return {string[][]}
     */
    groupAnagrams(strs) {
        const groups = {};
        for(let i = 0; i < strs.length; i++) {
            let stringUnderReview = strs[i];
            let sortedString = stringUnderReview.split('').sort().join('');
            groups[sortedString] = groups[sortedString] || [];
            groups[sortedString].push(stringUnderReview)
        }
        console.log(groups);
        console.log(Object.values(groups));
        return Object.values(groups);
    }
}
