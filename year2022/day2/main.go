package main

import (
	"fmt"
	"strings"

	"github.com/wjzijderveld/adventofcode/year2022"
)

const (
	Rock = iota
	Paper
	Scissors
)

// A = Rock
// B = Paper
// C = Scissors

// X = Rock
// Y = Paper
// Z = Scissors
func main() {
	totalScore := 0
	for line := range year2022.GetInputLines(2022, 2) {
		rpc := strings.Fields(line)
		if len(rpc) != 2 {
			panic("invalid rock, paper scissor input")
		}

		totalScore += getScore(normalize(rpc[0]), normalize(rpc[1]))
	}

	fmt.Printf("Total score: %d\n", totalScore)
}

func getScore(op int, you int) int {
	base := scoreForSign(you)
	if op == you {
		return base + 3
	}

	if op == Rock && you == Paper {
		return base + 6
	}
	if op == Paper && you == Scissors {
		return base + 6
	}
	if op == Scissors && you == Rock {
		return base + 6
	}

	return base
}

func scoreForSign(sign int) int {
	switch sign {
	case Rock:
		return 1
	case Paper:
		return 2
	case Scissors:
		return 3
	}

	panic("invalid: " + string(sign))
}

func normalize(given string) int {
	switch given {
	case "A", "X":
		return Rock
	case "B", "Y":
		return Paper
	case "C", "Z":
		return Scissors
	}

	panic("invalid: " + given)
}
