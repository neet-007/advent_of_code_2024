package main

import (
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func main() {
	part2()
}

func part2() {
	data, err := os.ReadFile("./input.txt")
	if err != nil {
		panic(err)
	}

	heapLeft := []int{}
	heapRigth := []int{}

	digitsLeft := []byte{}
	digitsRigth := []byte{}

	i := 0
	lenData := len(data)
	for i < lenData {
		if unicode.IsSpace(rune(data[i])) {
			for unicode.IsSpace(rune(data[i])) {
				i++
			}

			for data[i] != '\n' {
				digitsRigth = append(digitsRigth, data[i])
				i++
			}

			num, err := strconv.Atoi(string(digitsRigth))
			if err != nil {
				panic(err)
			}

			digitsRigth = digitsRigth[:0]

			heapRigth = append(heapRigth, num)
			heapRigth = heapifyUp(heapRigth, len(heapRigth)-1)
			i++
			continue
		}

		if data[i] == '\n' {
			i++
			continue
		}

		for !unicode.IsSpace(rune(data[i])) {
			digitsLeft = append(digitsLeft, data[i])
			i++
		}

		num, err := strconv.Atoi(string(digitsLeft))
		if err != nil {
			panic(err)
		}

		digitsLeft = digitsLeft[:0]

		heapLeft = append(heapLeft, num)
		heapLeft = heapifyUp(heapLeft, len(heapLeft)-1)
		i++
	}

	lenHeap := len(heapLeft)
	sum := 0
	track := map[int]int{}
	i = 0
	for i < lenHeap && len(heapRigth) > 0 {
		hl, a := heapMin(heapLeft)
		b := heapRigth[0]

		heapLeft = hl
		for a <= b && len(heapRigth) > 0 {
			fmt.Printf("a %v b %v\n", a, b)

			if a == b {
				val, ok := track[a]
				if !ok {
					track[a] = val
				} else {
					track[a] += i
				}
				heapRigth = heapDelete(heapRigth, b)
				if len(heapRigth) > 0 {
					b = heapRigth[0]
				}
				continue
			}

			index := heapNextMax(heapRigth, b)
			if index == -1 {
				return
			}
			b = heapRigth[index]
		}

		index := heapNextMax(heapRigth, b)
		if index == -1 {
			return
		}
		b = heapRigth[index]
		sum += a * track[a]
		i++
	}

	fmt.Printf("%v\n", sum)

}

func part1() {
	data, err := os.ReadFile("./input.txt")
	if err != nil {
		panic(err)
	}

	heapLeft := []int{}
	heapRigth := []int{}

	digitsLeft := []byte{}
	digitsRigth := []byte{}

	i := 0
	lenData := len(data)
	for i < lenData {
		if unicode.IsSpace(rune(data[i])) {
			for unicode.IsSpace(rune(data[i])) {
				i++
			}

			for data[i] != '\n' {
				digitsRigth = append(digitsRigth, data[i])
				i++
			}

			num, err := strconv.Atoi(string(digitsRigth))
			if err != nil {
				panic(err)
			}

			digitsRigth = digitsRigth[:0]

			heapRigth = append(heapRigth, num)
			heapRigth = heapifyUp(heapRigth, len(heapRigth)-1)
			i++
			continue
		}

		if data[i] == '\n' {
			i++
			continue
		}

		for !unicode.IsSpace(rune(data[i])) {
			digitsLeft = append(digitsLeft, data[i])
			i++
		}

		num, err := strconv.Atoi(string(digitsLeft))
		if err != nil {
			panic(err)
		}

		digitsLeft = digitsLeft[:0]

		heapLeft = append(heapLeft, num)
		heapLeft = heapifyUp(heapLeft, len(heapLeft)-1)
		i++
	}

	lenHeap := len(heapLeft)
	sum := 0
	for _ = range lenHeap {
		hl, a := heapMin(heapLeft)
		hr, b := heapMin(heapRigth)

		heapLeft = hl
		heapRigth = hr

		fmt.Printf("a %v b %v\n", a, b)

		if a < b {
			sum += b - a
		} else {
			sum += a - b
		}
	}

	fmt.Printf("%v\n", sum)
}

func heapMin(heap []int) ([]int, int) {
	returnVal := heap[0]
	heap[0], heap[len(heap)-1] = heap[len(heap)-1], heap[0]

	heap = heap[:len(heap)-1]

	return heapifyDown(heap, 0), returnVal
}

func heapifyUp(heap []int, index int) []int {
	curr := index
	parent := heapParent(curr)

	for curr > 0 && heap[curr] < heap[parent] {
		heap[curr], heap[parent] = heap[parent], heap[curr]
		curr = parent
		parent = heapParent(curr)
	}

	return heap
}

func heapifyDown(heap []int, index int) []int {
	curr := index
	minChild := curr
	lenHeap := len(heap)
	leftChild := heapChild(curr, 'l')

	if leftChild < lenHeap && heap[leftChild] < heap[minChild] {
		minChild = leftChild
	}

	rigthChild := heapChild(curr, 'r')
	if rigthChild < lenHeap && heap[rigthChild] < heap[minChild] {
		minChild = rigthChild
	}

	for minChild != curr {
		heap[curr], heap[minChild] = heap[minChild], heap[curr]

		curr = minChild
		minChild = curr

		leftChild = heapChild(curr, 'l')

		if leftChild < lenHeap && heap[leftChild] < heap[minChild] {
			minChild = leftChild
		}

		rigthChild = heapChild(curr, 'r')
		if rigthChild < lenHeap && heap[rigthChild] < heap[minChild] {
			minChild = rigthChild
		}
	}

	return heap
}

func heapDelete(heap []int, val int) []int {
	index := 0
	for index > -1 && heap[index] != val {
		if heap[index] < val {
			index = heapChild(index, 'l')
		} else {
			index = heapChild(index, 'r')
		}
	}

	if index == -1 {
		panic("not found")
	}

	heap[len(heap)-1], heap[index] = heap[index], heap[len(heap)-1]
	heap = heap[:len(heap)-1]

	return heapifyDown(heap, index)
}

func heapNextMin(heap []int, index int) int {
	left := heapChild(index, 'l')

	if left > len(heap) {
		return -1
	}

	return left
}

func heapNextMax(heap []int, index int) int {
	rigth := heapChild(index, 'r')

	if rigth > len(heap) {
		return -1
	}

	return rigth

}

func heapParent(i int) int {
	return (i - 1) / 2
}

func heapChild(i int, dir rune) int {
	if dir == 'l' {
		return i*2 + 1
	}
	if dir == 'r' {
		return i*2 + 2
	}

	panic("l or r only")
}
