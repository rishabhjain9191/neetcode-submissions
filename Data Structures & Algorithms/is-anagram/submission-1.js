class Solution {
    /**
     * @param {string} s
     * @param {string} t
     * @return {boolean}
     */
    isAnagram(s, t) {
        const sCount = {};
        const tCount = {};
        // If length not same, return false right away
        if(!(s.length === t.length)) {
            return false;
        }
        // note all the counts of alphabets
        for(let i = 0; i < s.length; i++) {
            sCount[s[i]] = sCount[s[i]] || 0;
            sCount[s[i]]++;
            tCount[t[i]] = tCount[t[i]] || 0;
            tCount[t[i]]++;
        }
        // console.log(sCount, tCount);
        for(let char in sCount) {
            if((sCount[char] !== tCount[char]) || !tCount[char]) {
                return false;
            }
        }
        return true;
    }
}
