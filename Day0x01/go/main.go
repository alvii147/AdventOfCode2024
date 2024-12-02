package main

import (
	"bufio"
	"cmp"
	"container/heap"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

// Abs computes the absolute value of an integer.
func Abs(x int) int {
	if x >= 0 {
		return x
	}

	return -1 * x
}

// MinHeap implements a min heap data structure.
type MinHeap[T cmp.Ordered] []T

// Len computes the length of a min heap.
func (h MinHeap[T]) Len() int {
	return len(h)
}

// Less compares two elements of a min heap by index and returns
// whether the first one is less than the second.
func (h MinHeap[T]) Less(i, j int) bool {
	return h[i] < h[j]
}

// Swap swaps two elements of a min heap by index.
func (h MinHeap[T]) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

// Push adds a new element to a min heap.
func (h *MinHeap[T]) Push(x any) {
	*h = append(*h, x.(T))
}

// Pop removes and returns the minimum value from a min heap.
func (h *MinHeap[T]) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]

	return x
}

// ParseIDs parses left and right IDs from a single line.
func ParseIDs(s string) (int, int, error) {
	split := strings.Fields(s)
	if len(split) != 2 {
		return 0, 0, fmt.Errorf("expected two values in \"%s\", found %d", s, len(split))
	}

	leftID, err := strconv.Atoi(split[0])
	if err != nil {
		return 0, 0, fmt.Errorf("strconv.Atoi \"%s\": %w", split[0], err)
	}

	rightID, err := strconv.Atoi(split[1])
	if err != nil {
		return 0, 0, fmt.Errorf("strconv.Atoi \"%s\": %w", split[1], err)
	}

	return leftID, rightID, nil
}

func main() {
	filePath := "../lists.txt"
	f, err := os.Open(filePath)
	if err != nil {
		log.Fatalf("ERROR - os.Open: %v\n", err)
		return
	}

	leftHeap := &MinHeap[int]{}
	heap.Init(leftHeap)
	rightHeap := &MinHeap[int]{}
	heap.Init(rightHeap)

	leftCounts := make(map[int]int)
	rightCounts := make(map[int]int)

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		leftID, rightID, err := ParseIDs(line)
		if err != nil {
			log.Fatalf("ERROR - ParseIDs: %v\n", err)
			return
		}

		heap.Push(leftHeap, leftID)
		heap.Push(rightHeap, rightID)

		leftCount, ok := leftCounts[leftID]
		if !ok {
			leftCount = 0
		}
		leftCounts[leftID] = leftCount + 1

		rightCount, ok := rightCounts[rightID]
		if !ok {
			rightCount = 0
		}
		rightCounts[rightID] = rightCount + 1
	}

	totalDistance := 0
	for leftHeap.Len() > 0 && rightHeap.Len() > 0 {
		leftID, rightID := heap.Pop(leftHeap).(int), heap.Pop(rightHeap).(int)
		totalDistance += Abs(leftID - rightID)
	}

	similarityScore := 0
	for leftID, leftCount := range leftCounts {
		rightCount, ok := rightCounts[leftID]
		if !ok {
			rightCount = 0
		}

		similarityScore += leftID * leftCount * rightCount
	}

	fmt.Printf("Part 1: %d\n", totalDistance)
	fmt.Printf("Part 2: %d\n", similarityScore)
}
