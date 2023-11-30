package main

import (
	"fmt"
	"sort"
	"strconv"

	"github.com/wjzijderveld/adventofcode/year2022"
)

func main() {
	lines := year2022.GetInputLines(2022, 1)

	var carriedPerElf []int
	currentElfCarries := 0
	for line := range lines {
		if line == "" {
			carriedPerElf = append(carriedPerElf, currentElfCarries)
			currentElfCarries = 0
			continue
		}

		calories, err := strconv.ParseInt(line, 10, 32)
		if err != nil {
			panic("failed to parse int from line: " + err.Error())
		}

		currentElfCarries += int(calories)
	}

	sort.Ints(carriedPerElf)
	topThree := carriedPerElf[len(carriedPerElf)-3:]

	sum := 0
	for _, x := range topThree {
		sum += x
	}

	fmt.Printf("Most that's being carried: %d\n", topThree[2])
	fmt.Printf("Top three being carried: %d\n", sum)
}
