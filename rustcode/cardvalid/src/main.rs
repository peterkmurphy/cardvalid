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
// Without command line arguments, the program does nothing.

use std::env;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let filepath = Path::new(&args[1]);
        let filepathdisplay = filepath.display();
        let mut file = File::open(&filepath);
        let mut filestatus = match file
        {
            Ok(file) => "Good",
            Err(why) => "Bad",
        };
        if filestatus == "Bad" {
            println!("The argument at {} is not a real file", args[1]);
        }

// I'd like to avoid panics, but let's come back later.

//        let filereader = BufReader::new(&file);


        println!("The first argument is {}", args[1]);
    } else {
        println!("This is the testing code");
    }
}
