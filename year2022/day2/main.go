package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/wjzijderveld/adventofcode/year2022"
)

type RPS int

const (
	Rock RPS = iota
	Paper
	Scissors
)

func (r RPS) LoosesFrom() RPS {
	switch r {
	case Rock:
		return Paper
	case Paper:
		return Scissors
	case Scissors:
		return Rock
	}
	panic("invalid RPS")
}

func (r RPS) WinsFrom() RPS {
	switch r {
	case Rock:
		return Scissors
	case Paper:
		return Rock
	case Scissors:
		return Paper
	}
	panic("invalid RPS")
}

func (r RPS) String() string {
	switch r {
	case Rock:
		return "Rock"
	case Paper:
		return "Paper"
	case Scissors:
		return "Scissors"
	}
	panic("invalid RPS")

}

// A = Rock
// B = Paper
// C = Scissors

// X = Lose
// Y = Draw
// Z = Win
func main() {
	totalScore := 0
	for line := range year2022.GetInputLines(2022, 2) {
		rps := strings.Fields(line)
		if len(rps) != 2 {
			panic("invalid rock, paper scissor input")
		}

		op := normalize(rps[0])
		var needs RPS
		switch rps[1] {
		case "X":
			needs = op.WinsFrom()
		case "Y":
			needs = op
		case "Z":
			needs = op.LoosesFrom()
		default:
			panic("invalid: " + rps[1])
		}

		score := getScore(op, needs)
		if os.Getenv("DEBUG") == "true" {
			fmt.Printf("Round %s (%s vs %s): %d\n", line, op.String(), needs.String(), score)
		}
		totalScore += score
	}

	fmt.Printf("Total score: %d\n", totalScore)
}

func getScore(op RPS, you RPS) int {
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

func scoreForSign(sign RPS) int {
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

func normalize(given string) RPS {
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
