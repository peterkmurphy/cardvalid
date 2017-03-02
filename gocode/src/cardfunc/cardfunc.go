// cardfunc.go
// Contains functions for analysing credit cards
// Copyright (c) Peter Murphy 2017

package cardfunc
import (
    "fmt"
    "strings"
    "unicode"
)

// Constants for type of credit cards the program handles (as printed out)

const CARD_VI = "VISA"
const CARD_AM = "AMEX"
const CARD_DI = "Discover"
const CARD_MC = "MasterCard"
const CARD_UN = "Unknown"

// Constants for the validity or invalidity of cards (as printed out)

const STAT_VA = "valid"
const STAT_IN = "invalid"

// A constant for the minimum indentation level for the [in]validity status
// as pretty printed out. This makes the output lined up.

const MIN_STAT_INDENT = 29

// This returns the maximum of two numbers, same as in Python and other
// languages. (Seems in golang, you have to roll your own max function if
// you are handling integers.)

func max(fir, sec int) int { if fir > sec { return fir }; return sec }

// This function takes a card type (cardtype), a card number (cardno), and
// an indicator of whether a card is valid or not (state). The function
// returns the result in pretty printed form, with validity indicators all
// aligned with each other (as shown in the sample output above).

func prettyprintcard(cardtype string, cardno string, state string) string {
    prologue := fmt.Sprintf("%s: %s", cardtype, cardno)
    midspacing := strings.Repeat(" ", max (MIN_STAT_INDENT - len(prologue), 1))
    return fmt.Sprintf("%s%s(%s)", prologue, midspacing, state)
}

// The cardnoanalyse function takes a string which is meant to contain a credit
// card number. After stripping all white space and line break characters, it
// attempts to identify the type of card. It then pretty-prints the card type,
// the card number and whether it is valid or not.
//
// For example, the following input:
//
// 4111111111111111
//
// Is analysed and returned as:
//
// VISA: 4111111111111111       (valid)
//
// The sole parameter is cardnoin: a string containing a credit card number.
// The function returns the pretty-printed result of the analysis.

func Cardnoanalyse(cardnoin string) string {

// Idea for stripping all whitespace comes from here:
// http://stackoverflow.com/questions/32081808/strip-all-whitespace-from-a-string-in-golang

    cardno := strings.Map(func(r rune) rune {
        if unicode.IsSpace(r) {
            return -1
        }
        return r
    }, cardnoin)
    cardnolen := len(cardno)
    cardtype := CARD_UN
    state := STAT_IN
    var digitarray []int
    digitarray = make([]int, cardnolen)
    for i := 0; i < cardnolen; i++ {
        if (cardno[i] >= 48) && (cardno[i] <= 57) {
            digitarray[i] = int(cardno[i] - 48);
        } else {
            return prettyprintcard(CARD_UN, cardno, STAT_IN)
        }
    }
    if (digitarray[0] == 4) && ((cardnolen == 13) || (cardnolen ==  16)) {
        cardtype = CARD_VI
    } else if (digitarray[0] == 3) && ((digitarray[1] == 4) ||
            (digitarray[1] == 7)) && (cardnolen == 15) {
        cardtype = CARD_AM
    } else if (digitarray[0] == 5) && (digitarray[1] >= 1) && (
            digitarray[1] <= 5) && (cardnolen == 16) {
        cardtype = CARD_MC
    } else if (cardno[0:4] == "6011") && (cardnolen == 16) {
        cardtype = CARD_DI
    }
    checksum := 0
    for i := 0; i < cardnolen; i++ { // We loop through the card digits...
        item := digitarray[cardnolen - i - 1] // But take items in reverse...

// Note: we start at the rightmost digit, where i is 0, but this is considered
// to be an "odd" digit. This is why i % 2 == 1 is "even", and i % 2 == 0 is
// "odd". Such is how the Luhn algorithm works.

        if (i % 2 == 1) { // An even digit
            if (item == 9) {
                checksum += 9 // Digit 9 doubles to 18 and becomes 9
            } else {
                checksum += (item * 2) % 9 // Take modulo 9.
            }
        } else { // An odd digit
            checksum += item
        }
    }
    if (checksum % 10 == 0) {
        state = STAT_VA
    }
    return prettyprintcard(cardtype, cardno, state)
}
