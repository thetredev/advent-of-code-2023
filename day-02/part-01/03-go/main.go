package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var MAX_CUBES map[string]int = map[string]int{
	"red":   12,
	"green": 13,
	"blue":  14,
}

func isPossible(groups []string) bool {
	for _, group := range groups {
		groupSlices := strings.Split(group, ", ")

		for _, value := range groupSlices {
			groupSlice := strings.Split(value, " ")
			cubesString := groupSlice[0]
			color := groupSlice[1]

			if strings.HasSuffix(color, "\n") {
				color = strings.TrimRight(color, "\n")
			}

			cubes, err := strconv.Atoi(cubesString)

			if err != nil {
				panic(err)
			}

			if cubes > MAX_CUBES[color] {
				return false
			}
		}
	}

	return true
}

func lex(file *os.File) int {
	defer file.Close()

	result := 0
	gameId := 0
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		gameId++

		dataStart := strings.Index(line, ":") + 2
		data := line[dataStart:]
		groups := strings.Split(data, "; ")

		if isPossible(groups) {
			result += gameId
		}
	}

	return result
}

func main() {
	file, err := os.Open(os.Args[1])

	if err != nil {
		panic(err)
	}

	fmt.Println(lex(file))
}
