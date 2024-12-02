package main

import (
	"bytes"
	"fmt"
	"math"
	"os"
	"strconv"
)

func main() {
	data, err := os.ReadFile("./input.txt")
	if err != nil {
		panic(err)
	}

	lines := bytes.Split(data, []byte("\n"))
	count := 0
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}

		runes := bytes.ReplaceAll(line, []byte(" "), []byte(""))
		nums := make([]int, len(runes))

		for i, r := range runes {
			num, err := strconv.Atoi(string(r))
			if err != nil {
				panic(err)
			}
			nums[i] = num
		}

		i := 0
		dir := 0
		flag := true
		for i < len(nums)-1 {
			diff := nums[i] - nums[i+1]
			if !(1 <= math.Abs(float64(diff)) && math.Abs(float64(diff)) <= 3) {
				flag = false
				break
			}

			if dir == 0 {
				dir = diff
			} else if dir >= 0 {
				if diff <= 0 {
					flag = false
					break
				}
			} else {
				if diff >= 0 {
					flag = false
					break
				}
			}
			i++
		}
		if flag {
			count++
		}
	}

	fmt.Printf("count: %d\n", count)
}
