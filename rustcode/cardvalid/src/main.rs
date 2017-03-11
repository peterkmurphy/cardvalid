// The card validation program (Rust implementation).
// Copyright (c) Peter Murphy 2017
// Executed as:
//
// cargo run [carddata.txt]
//
// Or
//
// cardvalid [carddata.txt]
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
// Without command line arguments, the program does nothing except print out
// a warning message. For testing, please type:
//
// cargo test

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::cmp;

// Constants for type of credit cards the program handles (as printed out)

static CARD_VI: &'static str = "VISA";
static CARD_AM: &'static str = "AMEX";
static CARD_DI: &'static str = "Discover";
static CARD_MC: &'static str = "MasterCard";
static CARD_UN: &'static str = "Unknown";

// Constants for the validity or invalidity of cards (as printed out)

static STAT_VA: &'static str = "valid";
static STAT_IN: &'static str = "invalid";

// A constant for the minimum indentation level for the [in]validity status
// as pretty printed out. This makes the output lined up.

const MIN_STAT_INDENT: usize = 29;

/// Strips all whitespace from a string.
pub fn whitespacebegone(input: &str) -> String {
    let mut buf = String::with_capacity(input.len());
    for c in input.chars() {
        if ! c.is_whitespace() {
            buf.push(c);
        }
    }
    return buf;
}

/// Makes list of integers from string of digits.
pub fn listofintegers(input: &String) -> Option<(Vec<u32>),> {
    let mut v: Vec<u32> = Vec::new();
    for c in input.chars() {
        if c.is_digit(10) == true {
            v.push(c.to_digit(10).unwrap());
        }
        else {
            return None;
        }
    }
    return Some(v);
}

/// The cardnoanalyse analyse takes a string which is meant to contain
/// a credit card number. After stripping all white space and line break
/// characters, it attempts to identify the type of card. It then pretty-
/// prints the card type, the card number and whether it is valid or not.
///
/// For example, the following input:
///
/// 4111111111111111
///
/// Is analysed and returned as:
///
/// VISA: 4111111111111111       (valid)
///
/// The sole parameter is cardnoin: a string containing a credit card number.
/// The function returns the pretty-printed result of the analysis.
pub fn cardnoanalyse (cardnoin: &str) -> String {
    let cardno = whitespacebegone(cardnoin);
    let cardnolen = cardno.chars().count();
    let mut cardtype = CARD_UN;
    let mut state = STAT_IN;
    let mut prologue = format!("{}: {}", cardtype, cardno.to_string());
    let mut reps = cmp::max(MIN_STAT_INDENT - prologue.chars().count(), 1);
    let mut midspacing = std::iter::repeat(" ").take(reps).collect::<String>();
    let digitsout = listofintegers(&cardno);
    if digitsout.is_none() {
        return format!("{}{}({})", prologue, midspacing, state);
    }
    let digitarray = digitsout.unwrap();
    if digitarray[0] == 4 && (cardnolen == 13 || cardnolen == 16) {
        cardtype = CARD_VI;
    }
    else if digitarray[0] == 3 && (digitarray[1] == 4 ||
            digitarray[1] == 7) && cardnolen == 15 {
        cardtype = CARD_AM;
    }
    else if digitarray[0] == 5 && (digitarray[1] >= 1 || digitarray[1] <= 5)
            && cardnolen == 16 {
        cardtype = CARD_MC;
    }
    else if &cardno[..4] == "6011" && cardnolen == 16 {
        cardtype = CARD_DI;
    }
    let mut checksum = 0;
    for i in 0..cardnolen { // We loop through the card digits...
         let item = digitarray[cardnolen - i - 1]; // Take items in reverse...

// Note: we start at the rightmost digit, where i is 0, but this is considered
// to be an "odd" digit. This is why i % 2 == 1 is "even", and i % 2 == 0 is
// "odd". Such is how the Luhn algorithm works.

        if i % 2 == 1 { // Even digits
            if item == 9 {
                checksum += 9; // Digit 9 doubles to 18 and becomes 9
            }
            else {
                checksum += (item * 2) % 9; // Take modulo 9.
            }
        }
        else { // Odd digits
            checksum += item;
        }
    }
    if checksum % 10 == 0 {
        state = STAT_VA;
    }
    prologue = format!("{}: {}", cardtype, cardno.to_string());
    reps = cmp::max(MIN_STAT_INDENT - prologue.chars().count(), 1);
    midspacing = std::iter::repeat(" ").take(reps).collect::<String>();
    return format!("{}{}({})", prologue, midspacing, state);
}

#[cfg(test)]
mod tests {
    use super::cardnoanalyse;
    const TESTDATA: &'static [&'static str] = &[
        "4111111111111111",
        "4111111111111",
        "4012888888881881",
        "378282246310005",
        "6011111111111117",
        "5105105105105100",
        "5105 1051 0510 5106",
        "9111111111111111",
        "4408 0412 3456 7893",
        "4417 1234 5678 9112"];
    const EXPECTEDOUTPUT: &'static [&'static str] = &[
        "VISA: 4111111111111111       (valid)",
        "VISA: 4111111111111          (invalid)",
        "VISA: 4012888888881881       (valid)",
        "AMEX: 378282246310005        (valid)",
        "Discover: 6011111111111117   (valid)",
        "MasterCard: 5105105105105100 (valid)",
        "MasterCard: 5105105105105106 (invalid)",
        "Unknown: 9111111111111111    (invalid)",
        "VISA: 4408041234567893       (valid)",
        "VISA: 4417123456789112       (invalid)"];

    #[test]
    fn test_cardnoanalyse() {
        for i in 0..TESTDATA.len() {
            assert_eq!(cardnoanalyse(TESTDATA[i]), EXPECTEDOUTPUT[i]);
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let filename = Path::new(&args[1]);
        let filenameshow = filename.display();
        let file = File::open(&filename);
        let mut fileok = file.is_ok();
        if fileok == false {
            println!("The argument at {} is not a real file.", filenameshow);
            return;
        }
        let mut realfile = file.unwrap();
        let mut filetext = String::new();
        fileok = realfile.read_to_string(&mut filetext).is_ok();
        if fileok == false {
            println!("The argument at {} is not utf-8.", filenameshow);
            return;
        }
        for line in filetext.lines() {
            if line.chars().count() > 0 {
                println!("{}",cardnoanalyse(line));
            }
        }
    } else {
        println!("There are no arguments for this program.");
    }
}
