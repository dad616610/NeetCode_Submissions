class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        c = 0
        l = len(nums)
        for i,n in enumerate(nums[::-1],start=1):
            if n != val:
                c+= 1
            else:
                nums.pop(l-i)
        return c