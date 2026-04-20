class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        a = [0] * 26
        b = [0] * 26
        for _s in s:
            a[ord(_s) - ord('a')] +=1
        for _t in t:
            b[ord(_t) - ord('a')] += 1
        for i in range(26):
            if a[i] != b[i]:
                return False
        return True