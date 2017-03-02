//usr/bin/go run $0 $@ ; exit
// The card validation program (Go implementation).
// Copyright (c) Peter Murphy 2017
// Executed as:
//
// go run cardvalid.go [carddata.txt]
//
// Where carddata.txt is a text file where each line is a credit card number.
// For each line, cardvalid prints out the card type, the card number, and
// whether the card is valid or not. For example, sample input could be
//
// 4111111111111111
// 4111111111111
// 4012888888881881
// 378282246310005
// 6011111111111117
// 5105105105105100
// 5105 1051 0510 5106
// 9111111111111111
//
// Sample output is:
//
// VISA: 4111111111111111       (valid)
// VISA: 4111111111111          (invalid)
// VISA: 4012888888881881       (valid)
// AMEX: 378282246310005        (valid)
// Discover: 6011111111111117   (valid)
// MasterCard: 5105105105105100 (valid)
// MasterCard: 5105105105105106 (invalid)
// Unknown: 9111111111111111    (invalid)
//
// Without command line arguments, the program does nothing.

package main
import (
    "bufio"
    "cardfunc"
    "fmt"
    "os"
)

func main() {
    argsWithoutProg := os.Args[1:]
    if (len(argsWithoutProg) == 1) {
        filename := argsWithoutProg[0]
        file, err := os.Open(filename)
        if err != nil {
            fmt.Print("No such file or directory, I'm afraid.\n")
            os.Exit(0)
        }
        defer file.Close()

        scanner := bufio.NewScanner(file)
        for scanner.Scan() {
            fmt.Println(cardfunc.Cardnoanalyse(scanner.Text()))
        }

        if err := scanner.Err(); err != nil {
            fmt.Print("Error in scanning.\n")
            os.Exit(0)
        }
    }
}
