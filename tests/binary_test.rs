//! # Binary operations integration tests
//!
//! Covers parsing and formatting binary values, parsing operation names,
//! logical-operation truth tables, sequence alignment, and empty-input errors.

use binary_operations::binary::{Binary, Operation, apply_sequences};

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses the string `0` as [`Binary::Zero`].
    #[test]
    fn parse_zero() {
        assert_eq!("0".parse::<Binary>(), Ok(Binary::Zero));
    }
    /// Parses the string `1` as [`Binary::One`].
    #[test]
    fn parse_one() {
        assert_eq!("1".parse::<Binary>(), Ok(Binary::One))
    }
    /// Rejects strings that do not represent a binary digit.
    #[test]
    fn reject_invalid_binary() {
        for input in ["2", "-1", "hello", "", "0.5"] {
            assert!(
                input.parse::<Binary>().is_err(), "Input {input:?} should have been refused!"
            )
        }
    }
    /// Accepts each supported uppercase logical-operation name.
    #[test]
    fn parse_ops() {
        for op in ["AND", "OR", "NAND", "NOR", "XOR", "XNOR"] {
            assert!(
                op.parse::<Operation>().is_ok(), "Operation {op:?} should have been ok!"
            )
        }
    }
    /// Rejects operation names that are not supported.
    #[test]
    fn reject_invalid_ops() {
        for op in ["XNAND", "MiaOR", "MiaoNAND", "MiaoNOR", "MiaoXOR", "MiaoXNOR"] {
            assert!(
                op.parse::<Operation>().is_err(), "Operation {op:?} should have been refused!"
            )
        }
    }
    /// Formats [`Binary::One`] as the string `1`.
    #[test]
    fn parse_display_one() {
        assert_eq!(Binary::One.to_string(), "1")
    }
    /// Formats [`Binary::Zero`] as the string `0`.
    #[test]
    fn parse_display_zero() {
        assert_eq!(Binary::Zero.to_string(), "0")
    }
    /// Checks XOR results for all four possible pairs of binary digits.
    #[test]
    fn xor_truth_table() {
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::Zero], Operation::XOR),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::One], Operation::XOR),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::Zero], Operation::XOR),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::One], Operation::XOR),
            Ok(vec![Binary::Zero])
        );
    }
    /// Checks AND results for all four possible pairs of binary digits.
    #[test]
    fn and_truth_table() {
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::Zero], Operation::AND),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::One], Operation::AND),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::Zero], Operation::AND),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::One], Operation::AND),
            Ok(vec![Binary::One])
        );
    }
    /// Checks OR results for all four possible pairs of binary digits.
    #[test]
    fn or_truth_table() {
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::Zero], Operation::OR),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::One], Operation::OR),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::Zero], Operation::OR),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::One], Operation::OR),
            Ok(vec![Binary::One])
        );
    }
    /// Checks NAND results for all four possible pairs of binary digits.
    #[test]
    fn nand_truth_table() {
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::Zero], Operation::NAND),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::One], Operation::NAND),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::Zero], Operation::NAND),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::One], Operation::NAND),
            Ok(vec![Binary::Zero])
        );
    }
    /// Checks NOR results for all four possible pairs of binary digits.
    #[test]
    fn nor_truth_table() {
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::Zero], Operation::NOR),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::One], Operation::NOR),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::Zero], Operation::NOR),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::One], Operation::NOR),
            Ok(vec![Binary::Zero])
        );
    }
    /// Checks XNOR results for all four possible pairs of binary digits.
    #[test]
    fn xnor_truth_table() {
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::Zero], Operation::XNOR),
            Ok(vec![Binary::One])
        );
        assert_eq!(
            apply_sequences(&[Binary::Zero], &[Binary::One], Operation::XNOR),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::Zero], Operation::XNOR),
            Ok(vec![Binary::Zero])
        );
        assert_eq!(
            apply_sequences(&[Binary::One], &[Binary::One], Operation::XNOR),
            Ok(vec![Binary::One])
        );
    }
    /// Checks that unequal-length sequences are right-aligned and the shorter
    /// input is left-padded with zeroes before XOR is applied.
    #[test]
    fn different_length() {
        assert_eq!(
            apply_sequences(&[Binary::One, Binary::Zero, Binary::One], &[Binary::One], Operation::XOR),
            Ok(vec![Binary::One, Binary::Zero, Binary::Zero])
        );
    }
    /// Checks that XOR is applied position by position to equal-length inputs.
    #[test]
    fn same_length() {
        assert_eq!(
            apply_sequences(&[Binary::Zero, Binary::One, Binary::Zero], &[Binary::One, Binary::Zero, Binary::One], Operation::XOR),
            Ok(vec![Binary::One, Binary::One, Binary::One])
        );
    }
    /// Checks that applying an operation to an empty first sequence fails.
    #[test]
    fn empty_a_sequence() {
        assert!(
            apply_sequences(&[], &[Binary::One, Binary::Zero, Binary::One], Operation::XOR).is_err(), "It should have thrown an error"
        )
    }
    /// Checks that applying an operation to an empty second sequence fails.
    #[test]
    fn empty_b_sequence() {
        assert!(
            apply_sequences(&[Binary::One, Binary::Zero, Binary::One], &[], Operation::XOR).is_err(), "It should have thrown an error"
        )
    }
}
