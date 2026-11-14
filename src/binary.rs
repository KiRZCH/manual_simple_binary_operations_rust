//! # **Manual, Simple, Binary Operations**
//!
//! ## List
//!
//! * **AND**
//! * **OR**
//! * **NAND**
//! * **NOR**
//! * **XOR**
//! * **XNOR**
//!
//! The module represents individual bits with [`Binary`] and applies a
//! selected [`Operation`] to pairs of bits or to two bit sequences.

use std::fmt::Formatter;

/// A single binary digit.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Binary {
    /// The binary digit zero.
    Zero,
    /// The binary digit one.
    One,
}

/// A Boolean operation supported by this module.
#[derive(Clone, Copy, Debug)]
pub enum Operation {
    /// Logical conjunction: one only when both inputs are one.
    AND,
    /// Logical disjunction: one when either input is one.
    OR,
    /// Exclusive disjunction: one when the inputs differ.
    XOR,
    /// Negated conjunction: zero only when both inputs are one.
    NAND,
    /// Negated disjunction: one only when both inputs are zero.
    NOR,
    /// Negated exclusive disjunction: one when the inputs match.
    XNOR,
}

/// Parses a trimmed string containing `0` or `1` as a [`Binary`] value.
///
/// Returns an error if the trimmed string is anything else.
impl std::str::FromStr for Binary {
    type Err = &'static str;

    /// # Parse a binary digit
    ///
    /// ## Arguments
    /// - `s`: String to parse; surrounding whitespace is ignored.
    ///
    /// ## Returns
    /// The matching [`Binary::Zero`] or [`Binary::One`] value.
    ///
    /// ## Errors
    /// Returns an error when the trimmed string is not `0` or `1`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "0" => Ok(Binary::Zero),
            "1" => Ok(Binary::One),
            _ => Err("Inserisci 0 oppure 1")
        }
    }
}

/// Parses a trimmed operation name such as `AND` or `XNOR`.
///
/// Returns an error if the string does not exactly match a supported name.
impl std::str::FromStr for Operation {
    type Err = &'static str;

    /// # Parse a logical operation
    ///
    /// ## Arguments
    /// - `s`: Operation name to parse; surrounding whitespace is ignored.
    ///
    /// ## Returns
    /// The matching [`Operation`] variant.
    ///
    /// ## Errors
    /// Returns an error when the trimmed string is not a supported operation name.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() { 
            "AND" => Ok(Operation::AND),
            "OR" => Ok(Operation::OR),
            "NAND" => Ok(Operation::NAND),
            "NOR" => Ok(Operation::NOR),
            "XOR" => Ok(Operation::XOR),
            "XNOR" => Ok(Operation::XNOR),
            _ => Err("Operation not recognized.")
        }
    }
}

/// Formats a [`Binary`] value as its single-character representation.
impl std::fmt::Display for Binary {
    /// # Format a binary digit
    ///
    /// ## Arguments
    /// - `self`: Binary digit to format.
    /// - `f`: Formatter that receives the output.
    ///
    /// ## Returns
    /// `Ok(())` when formatting succeeds, or a formatting error otherwise.
    ///
    /// ## Errors
    /// Returns the error reported by the formatter if writing fails.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Binary::Zero => write!(f, "0"),
            Binary::One => write!(f, "1"),
        }
    }
}


// Functions


/// # Apply an operation to two sequences
///
/// ## Arguments
/// - `source`: First input sequence.
/// - `key`: Second input sequence.
/// - `operation`: Logical operation to apply to corresponding bits.
///
/// The sequences are aligned at their right ends. The shorter is left-padded
/// with zeroes to match the longer sequence.
///
/// ## Returns
/// A sequence containing the operation result for each aligned pair.
///
/// ## Errors
/// Returns an error if either input sequence is empty.
/// 
/// # Examples
///
/// ```
/// use binary_operations::binary::{apply_sequences, Binary, Operation};
///
/// let result = apply_sequences(
///     &[Binary::One, Binary::Zero, Binary::One],
///     &[Binary::One],
///     Operation::XOR,
/// ).unwrap();
///
/// assert_eq!(result, vec![Binary::One, Binary::Zero, Binary::Zero]);
/// ```
pub fn apply_sequences(a: &[Binary], b: &[Binary], operation: Operation) -> Result<Vec<Binary>, &'static str> {
    if a.is_empty() || b.is_empty() {
        return Err("Arrays cannot be empty");
    }
    let mut result: Vec<Binary> = Vec::new();
    let length: usize = std::cmp::max(a.len(), b.len());
    for i in 0..length {
        let padding_source: usize = length - a.len();
        let n_source: Binary = if i < padding_source {
            Binary::Zero
        } else {
            a[i - padding_source]
        };
        let padding_key: usize = length - b.len();
        let n_key: Binary = if i < padding_key {
            Binary::Zero
        } else {
            b[i - padding_key]
        };
        let result_bit = apply(&n_source, &n_key, operation);
        result.push(result_bit);
    }
    Ok(result)
}



/// # Apply a logical operation
///
/// ## Arguments
/// - `a`: First bit.
/// - `b`: Second bit.
/// - `operation`: Operation to apply.
///
/// ## Returns
/// The result of applying `operation` to `a` and `b`.
///
/// ## Errors
/// This function does not return errors.
fn apply(a: &Binary, b: &Binary, operation: Operation) -> Binary {
    match operation {
        Operation::AND => { apply_and(a,b) },
        Operation::OR => { apply_or(a, b) },
        Operation::NAND => { apply_nand(a, b) },
        Operation::NOR => { apply_nor(a, b) },
        Operation::XNOR => { apply_xnor(a, b) },
        Operation::XOR => { apply_xor(a, b) }

    }
}

/// # Apply exclusive OR
///
/// ## Arguments
/// - `a`: First bit.
/// - `b`: Second bit.
///
/// ## Returns
/// [`Binary::One`] when the bits differ; otherwise [`Binary::Zero`].
///
/// ## Errors
/// This function does not return errors.
fn apply_xor(a: &Binary, b: &Binary) -> Binary {
    if a == b { Binary::Zero } else { Binary::One }
}

/// # Apply logical AND
///
/// ## Arguments
/// - `a`: First bit.
/// - `b`: Second bit.
///
/// ## Returns
/// [`Binary::One`] only when both bits are one.
///
/// ## Errors
/// This function does not return errors.
fn apply_and(a: &Binary, b: &Binary) -> Binary {
    if *a == Binary::One && *b == Binary::One { Binary::One } else { Binary::Zero }
}

/// # Apply logical OR
///
/// ## Arguments
/// - `a`: First bit.
/// - `b`: Second bit.
///
/// ## Returns
/// [`Binary::One`] when either bit is one.
///
/// ## Errors
/// This function does not return errors.
fn apply_or(a: &Binary, b: &Binary) -> Binary {
    if *a == Binary::One || *b == Binary::One { Binary::One } else { Binary::Zero }
}

/// # Apply logical NOR
///
/// ## Arguments
/// - `a`: First bit.
/// - `b`: Second bit.
///
/// ## Returns
/// [`Binary::One`] only when both bits are zero.
///
/// ## Errors
/// This function does not return errors.
fn apply_nor(a: &Binary, b: &Binary) -> Binary {
    if *a != Binary::One && *b != Binary::One { Binary::One } else { Binary::Zero }
}

/// # Apply logical NAND
///
/// ## Arguments
/// - `a`: First bit.
/// - `b`: Second bit.
///
/// ## Returns
/// [`Binary::Zero`] only when both bits are one; otherwise [`Binary::One`].
///
/// ## Errors
/// This function does not return errors.
fn apply_nand(a: &Binary, b: &Binary) -> Binary {
    if *a != Binary::One || *b != Binary::One { Binary::One } else { Binary::Zero }
}

/// # Apply exclusive NOR
///
/// ## Arguments
/// - `a`: First bit.
/// - `b`: Second bit.
///
/// ## Returns
/// [`Binary::One`] when the bits match; otherwise [`Binary::Zero`].
///
/// ## Errors
/// This function does not return errors.
fn apply_xnor(a: &Binary, b: &Binary) -> Binary {
    if a == b { Binary::One } else { Binary::Zero }
}

