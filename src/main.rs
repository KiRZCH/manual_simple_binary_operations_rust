//! # Interactive binary operations
//!
//! Runs a command-line program that reads a logical operation and two binary
//! sequences, applies the operation, and displays the inputs and result.

pub mod binary;

use std::cmp::max;
use std::io::stdin;
use binary::{Binary, Operation, apply_sequences};
/// # Read a sequence length
///
/// Prompts until the user enters a positive integer.
///
/// ## Arguments
/// - `question`: Prompt shown before each input attempt.
///
/// ## Returns
/// The valid positive sequence length.
///
/// ## Errors
/// Input read or parse errors are reported to the user; the function retries
/// and does not return an error.
fn ask_bits(question: &str) -> usize {
    loop {
        println!("{}", question);
        let mut answer: String = String::new();
        match stdin().read_line(&mut answer) {
            Ok(_) => {},
            Err(_) => {
                println!("Error during reading, retry");
                continue;
            }
        }
        // stdin().read_line(&mut answer).expect("Error during reading, retry");
        match answer.trim().parse::<usize>() {
            Ok(count) if count > 0 => return count,
            _ => println!("Insert a valid number")
        }
    }
}

/// # Read binary digits
///
/// Prompts for and collects the requested number of binary digits.
///
/// ## Arguments
/// - `count`: Number of digits to read.
///
/// ## Returns
/// A vector containing the entered digits in input order.
///
/// ## Errors
/// Input read and parse errors are reported to the user; invalid entries are
/// retried, so the function does not return an error.
fn insert_bits(count: usize) -> Vec<Binary> {
    let mut result: Vec<Binary> = Vec::new();
    let mut answer: String = String::new();
    for i in 0..count {
        println!("bit: {} / {}", i+1, count);
        loop {
            answer.clear();
            match stdin().read_line(&mut answer) {
                Ok(_) => {},
                Err(_) => {
                    println!("Error during reading, retry");
                    continue;
                }
            }
            match answer.trim().parse::<Binary>() {
                Ok(bit) => {
                    result.push(bit);
                    break;
                }
                Err(_) => {
                    println!("Inserisci 0 oppure 1")
                }
            }
        }
    }
    result
}

/// # Display the operation inputs and result
///
/// Prints both input sequences, left-padding them with zeroes for display so
/// their right ends align, followed by the result sequence.
///
/// ## Arguments
/// - `a1`: First input sequence.
/// - `a2`: Second input sequence.
/// - `x`: Result sequence produced by the selected operation.
///
/// ## Returns
/// Nothing. Output is written to standard output.
///
/// ## Errors
/// This function does not return errors.
fn visualizer(a1: &[Binary], a2: &[Binary], x: &[Binary]) {
    let length: usize = max(a1.len(), a2.len());
    println!("Visualizing results...");
    let source: String = a1.iter().map(|bit| bit.to_string()).collect();
    let source_padded: String = format!("{:0>width$}", source, width = length);
    println!("First Array: {source_padded}");
    let key: String = a2.iter().map(|bit| bit.to_string()).collect();
    let key_padded: String = format!("{:0>width$}", key, width = length);
    println!("Second Array: {key_padded}");
    let result: String = x.iter().map(|bit| bit.to_string()).collect();
    println!("result: {result}");
}



/// # Run the interactive program
///
/// Reads an operation and two non-empty binary sequences from standard input,
/// applies the operation, and displays the sequences and result.
///
/// ## Arguments
/// This function takes no arguments.
///
/// ## Returns
/// Returns `()` after displaying the result or reporting a handled input or
/// operation error.
///
/// ## Errors
/// Errors reading input are reported and retried. An operation error is
/// reported before the program returns early; no error is propagated to the
/// caller.
fn main() {
    let operation: Operation = loop {
        println!("Choose an operation: AND, OR, NAND, NOR, XOR, XNOR");
        let mut operation_question: String = String::new();

        match stdin().read_line(&mut operation_question) {
            Ok(_) => {}
            Err(_) => {
                println!("Error during reading.");
                continue;
            }
        }

        match operation_question.trim().parse::<Operation>() {
            Ok(operation) => break operation,
            Err(error) => println!("{error}"),
        }
    };
    let bits_first_array: usize = ask_bits("How many bits for the first array?");
    let bits_second_array: usize = ask_bits("How many bits for the second array?");
    println!("First array");
    let array1: Vec<Binary> = insert_bits(bits_first_array);
    println!("Second array");
    let array2: Vec<Binary> = insert_bits(bits_second_array);
    let result: Vec<Binary> = match apply_sequences(&array1, &array2, operation) {
        Ok(result) => result,
        Err(_) => {
            println!("Error during xor");
            return;
        }
    };
    visualizer(&array1, &array2, &result);
}
