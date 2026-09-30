/**
 * Source : https://leetcode.cn/problems/largest-positive-integer-that-exists-with-its-negative/
 *
 * Tag :
 * (1) array
 * (2) hash
 * (3) sort
 */

package main

import (
	"sort"
)

/**
 * V1
 * Time : 	O(N)
 * Space : 	O(N)
 *
 * Time : 	12ms 100%
 * Space : 	6.5M 53.85%
 */
func getMax(a, b int) int {
	if a < b {
		return b
	}
	return a
}

// func findMaxK(nums []int) int {
// 	result := -1
// 	mapData := make(map[int]int)

// 	for i := 0; i < len(nums); i++ {
// 		if nums[i] < 0 {
// 			if _, ok := mapData[-nums[i]]; ok {
// 				result = getMax(result, -nums[i])
// 			}
// 		} else if nums[i] > 0 {
// 			if _, ok := mapData[-nums[i]]; ok {
// 				result = getMax(result, nums[i])
// 			}
// 		}

// 		mapData[nums[i]] = 1
// 	}

// 	return result
// }

/**
 * V2
 * Time : 	O(NlogN)
 * Space : 	O(logN)
 *
 * Time : 	20ms 61.54%
 * Space : 	5.7M 80.77%
 */
func findMaxK(nums []int) int {
	sort.Ints(nums)

	left := 0
	right := len(nums) - 1
	for left < right {
		for left < right && -nums[left] > nums[right] {
			left++
		}
		if -nums[left] == nums[right] {
			return nums[right]
		}

		right--
	}

	return -1
}

