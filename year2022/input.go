package year2022

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
)

func GetInputLines(year int, day int) chan string {
	var fileReader io.ReadCloser

	filename := "input.txt"
	if os.Getenv("USE_EXAMPLE") == "true" {
		filename = "example.txt"
	}

	fileReader, err := os.Open(fmt.Sprintf("year%d/day%d/%s", year, day, filename))
	if err != nil {
		log.Fatalf("failed to open input file for challenge %d-%d: %s", year, day, err.Error())
	}

	ret := make(chan string)

	go func() {
		reader := bufio.NewReader(fileReader)
		defer fileReader.Close()
		for {
			line, err := reader.ReadString('\n')
			if err == io.EOF {
				break
			}

			if err != nil {
				panic("failed to read line: " + err.Error())
			}

			ret <- strings.TrimSuffix(line, "\n")
		}
		close(ret)
	}()

	return ret
}
