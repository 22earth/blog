def twoSum(nums, target):
    m = {}
    for i in range(0, len(nums)):
        num = nums[i]
        if target - num in m:
            return [m[target-num], i]
        m[num] = i
    return [-1, -1]

print(twoSum([2, 7, 11, 15], 9))
