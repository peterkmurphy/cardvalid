#!/usr/bin/env python
# -*- coding: utf-8 -*-
# The credit card validation program (Python implementation).
# Copyright (c) Peter Murphy 2017
# Executed as:
#
# python cardvalid.py [carddata.txt]
#
# Where carddata.txt is a text file where each line is a credit card number.
# For each line, cardvalid prints out the card type, the card number, and
# whether the card is valid or not. For example, sample input could be
#
# 4111111111111111
# 4111111111111
# 4012888888881881
# 378282246310005
# 6011111111111117
# 5105105105105100
# 5105 1051 0510 5106
# 9111111111111111
#
# Sample output is:
#
# VISA: 4111111111111111       (valid)
# VISA: 4111111111111          (invalid)
# VISA: 4012888888881881       (valid)
# AMEX: 378282246310005        (valid)
# Discover: 6011111111111117   (valid)
# MasterCard: 5105105105105100 (valid)
# MasterCard: 5105105105105106 (invalid)
# Unknown: 9111111111111111    (invalid)
#
# Without command line arguments, the program runs in testing mode.

import re
import unittest
import sys
import io

# Constants for type of credit cards the program handles (as printed out)

CARD_VI = "VISA"
CARD_AM = "AMEX"
CARD_DI = "Discover"
CARD_MC = "MasterCard"
CARD_UN = "Unknown"

# Constants for the validity or invalidity of cards (as printed out)

STAT_VA = "valid"
STAT_IN = "invalid"

# A constant for the minimum indentation level for the [in]validity status
# as pretty printed out. This makes the output lined up.

MIN_STAT_INDENT = 29

def cardnoanalyse(cardnoin):
    """ The cardnoanalyse analyse takes a string which is meant to contain
    a credit card number. After stripping all white space and line break
    characters, it attempts to identify the type of card. It then pretty-
    prints the card type, the card number and whether it is valid or not.

    For example, the following input:

    4111111111111111

    Is analysed and returned as:

    VISA: 4111111111111111       (valid)

    The sole parameter is cardnoin: a string containing a credit card number.
    The function returns the pretty-printed result of the analysis.
    """
    cardno =  re.sub(r"\s+", "", cardnoin, flags=re.UNICODE)
    cardnolen = len(cardno)
    cardtype = CARD_UN
    state = STAT_IN
    try:
        digitarray = [int(char) for char in cardno]
        if digitarray[0] == 4 and cardnolen in [13, 16]:
            cardtype = CARD_VI
        elif digitarray[0] == 3 and digitarray[1] in [4, 7] and cardnolen == 15:
            cardtype = CARD_AM
        elif digitarray[0] == 5 and digitarray[1] in range(1, 6) \
                and cardnolen == 16:
            cardtype = CARD_MC
        elif cardno[0:4] == "6011" and cardnolen == 16:
            cardtype = CARD_DI
        checksum = 0
        for i in range(cardnolen): # We loop through the card digits...
            item = digitarray[cardnolen - i - 1] # But take items in reverse...

# Note: we start at the rightmost digit, where i is 0, but this is considered
# to be an "odd" digit. This is why i % 2 == 1 is "even", and i % 2 == 0 is
# "odd". Such is how the Luhn algorithm works.

            if i % 2 == 1: # Even digits
                if item == 9:
                    checksum += 9 # Digit 9 doubles to 18 and becomes 9
                else:
                    checksum += (item * 2) % 9 # Take modulo 9.
            else: # Odd digits
                checksum += item
        if checksum % 10 == 0:
            state = STAT_VA
    except Exception as inst:
        pass
    prologue ="{cardtype}: {cardno}".format(**vars())
    midspacing = " " * (max (MIN_STAT_INDENT - len(prologue), 1))
    return "{prologue}{midspacing}({state})".format(**vars())

TESTDATA = [
    "4111111111111111",
    "4111111111111",
    "4012888888881881",
    "378282246310005",
    "6011111111111117",
    "5105105105105100",
    "5105 1051 0510 5106",
    "9111111111111111",
    "4408 0412 3456 7893",
    "4417 1234 5678 9112"]

EXPECTEDOUTPUT = [
    "VISA: 4111111111111111       (valid)",
    "VISA: 4111111111111          (invalid)",
    "VISA: 4012888888881881       (valid)",
    "AMEX: 378282246310005        (valid)",
    "Discover: 6011111111111117   (valid)",
    "MasterCard: 5105105105105100 (valid)",
    "MasterCard: 5105105105105106 (invalid)",
    "Unknown: 9111111111111111    (invalid)",
    "VISA: 4408041234567893       (valid)",
    "VISA: 4417123456789112       (invalid)"]

class TestCardValidation(unittest.TestCase):

    def test_cardnoanalyse(self):
        for i in range(len(TESTDATA)):
            self.assertEqual(cardnoanalyse(TESTDATA[i]), EXPECTEDOUTPUT[i])

if __name__ == '__main__':

# With no command line arguments, assume that the program is in testing mode.
# With one argument, assume that this is a file with lines consisting of
# credit card numbers for validation.

    if len(sys.argv) == 1:
        unittest.main()
    else:
        with io.open(sys.argv[1], "r", encoding="utf-8") as f:
            for line in f:
                print cardnoanalyse(line)
