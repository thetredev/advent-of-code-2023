package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func lex(file *os.File) int {
	defer file.Close()

	result := 0
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()
		digits := []rune{}

		for _, char := range line {
			if unicode.IsDigit(char) {
				digits = append(digits, char)
			}
		}

		strValue := fmt.Sprintf("%c%c", digits[0], digits[len(digits)-1])
		value, err := strconv.Atoi(strValue)

		if err != nil {
			panic(err)
		}

		result += value
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
