/**
 * Source : https://leetcode.cn/problems/flip-columns-for-maximum-number-of-equal-rows/
 *
 * Tag :
 * (1) array
 * (2) hashmap
 */

package main

/**
 * V1
 * Time : 	O(MN)
 * Space : 	O(M)
 *
 * Time : 	148ms 100%
 * Space : 	7.2M 88.24%
 */
func maxEqualRowsAfterFlips(m [][]int) int {
	mapData := make(map[string]int)

	for i := 0; i < len(m); i++ {
		array := make([]byte, len(m[0]), len(m[0]))
		for j := 0; j < len(m[0]); j++ {
			if m[i][0] ^ m[i][j] == 0 {
				array[j] = '0'
			} else {
				array[j] = '1'
			}
		}

		mapData[string(array)]++
	}

	result := 0
	for _, v := range mapData {
		if v > result {
			result = v
		}
	}
	return result
}