package main

import (
	"fmt"
	"strconv"

	"github.com/wjzijderveld/adventofcode/year2022"
)

func main() {
	lines := year2022.GetInputLines(2022, 1)

	maxBeingCarried, currentElfCarries := 0, 0
	for line := range lines {
		if line == "" {
			if currentElfCarries > maxBeingCarried {
				maxBeingCarried = currentElfCarries
			}
			currentElfCarries = 0
			continue
		}

		calories, err := strconv.ParseInt(line, 10, 32)
		if err != nil {
			panic("failed to parse int from line: " + err.Error())
		}

		currentElfCarries += int(calories)
	}

	fmt.Printf("Most that's being carried: %d\n", maxBeingCarried)
}
