class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        pref = strs[0]
        for s in strs:
            while pref not in s:
                pref = pref[:~0]
        return pref