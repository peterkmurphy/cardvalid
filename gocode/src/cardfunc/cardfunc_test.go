package cardfunc_test

import "testing"
import "fmt"
import "cardfunc"

var TESTDATA = []string {
    "4111111111111111",
    "4111111111111",
    "4012888888881881",
    "378282246310005",
    "6011111111111117",
    "5105105105105100",
    "5105 1051 0510 5106",
    "9111111111111111",
    "4408 0412 3456 7893",
    "4417 1234 5678 9112"}

var EXPECTEDOUTPUT = []string {
    "VISA: 4111111111111111       (valid)",
    "VISA: 4111111111111          (invalid)",
    "VISA: 4012888888881881       (valid)",
    "AMEX: 378282246310005        (valid)",
    "Discover: 6011111111111117   (valid)",
    "MasterCard: 5105105105105100 (valid)",
    "MasterCard: 5105105105105106 (invalid)",
    "Unknown: 9111111111111111    (invalid)",
    "VISA: 4408041234567893       (valid)",
    "VISA: 4417123456789112       (invalid)"}


// Prints out a pretty printed message

func TCardValErr (t *testing.T, testno int, expected string, received string) {
    if expected != received {
        t.Error(fmt.Sprintf(
            "For test no %d: expected %s; got %s.\n", testno, expected,
            received))
    }
}

func TestCardValid(t *testing.T) {
    for i := 0; i < len(TESTDATA); i++ {
        expected := EXPECTEDOUTPUT[i]
        received := cardfunc.Cardnoanalyse(TESTDATA[i])
        TCardValErr(t, i, expected, received)
    }
}
