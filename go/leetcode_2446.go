/**
 * Source : https://leetcode.cn/problems/determine-if-two-events-have-conflict/
 *
 * Tag :
 * (1) array
 */

package main

import (
	"fmt"
	"strconv"
)

/**
 * V1
 * Time : 	O(1)
 * Space : 	O(1)
 *
 * Time : 	0ms 100%
 * Space : 	1.9M 86.21%
 */
func convertToDate(time string) (int, int) {
	hh, _ := strconv.Atoi(time[0:2])
	mm, _ := strconv.Atoi(time[3:5])
	return hh, mm
}

func haveConflict(event1 []string, event2 []string) bool {
	begHH1, begMM1 := convertToDate(event1[0])
	endHH1, endMM1 := convertToDate(event1[1])
	begHH2, begMM2 := convertToDate(event2[0])
	endHH2, endMM2 := convertToDate(event2[1])
	fmt.Println(begHH1, begMM1, endHH1, endMM1)
	fmt.Println(begHH2, begMM2, endHH2, endMM2)

	if endHH1 < begHH2 || (endHH1 == begHH2 && endMM1 < begMM2) {
		return false
	}

	if endHH2 < begHH1 || (endHH2 == begHH1 && endMM2 < begMM1) {
		return false
	}

	return true
}

func main() {
	e1 := "10:13"
	e2 := "13:02"
	e3 := "13:17"
	e4 := "21:38"

	event1 := []string {e1, e2}
	event2 := []string {e3, e4}
	fmt.Println(haveConflict(event1, event2))
}