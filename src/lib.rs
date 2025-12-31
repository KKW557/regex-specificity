//! # regex-specificity
//!
//! This crate provides a heuristic-based approach to calculate the **specificity**
//! of a regular expression pattern against a specific string.
//!
//! ## Concept
//! 
//! Specificity measures how "precise" a match is. For example, the pattern `abc` is more specific to the string "abc" than the pattern `a.c` or `.*`.
//! 
//! The calculation follows these principles:
//! 1. **Positional Weighting**: Earlier matches contribute more to the total specificity than later ones.
//! 2. **Certainty**: Literals (exact characters) specificity higher than character classes or wildcards.
//! 3. **Information Density**: Narrower character classes (e.g., `[a-z]`) specificity higher than broader ones (e.g., `.`).
//! 4. **Branching Penalty**: Patterns with many alternatives (alternations) are penalized as they are less specific.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]

mod ffi;

extern crate alloc;
use alloc::boxed::Box;
use core::str;

use regex_syntax::hir::{Class, Hir, HirKind};
use regex_syntax::Parser;

const WEIGHT: u64 = 1 << SHIFT;
const STEP: u64 = 128;
const BITS_U64: u32 = 64;
const SHIFT: u32 = 16;
const MAX_SHIFT: u32 = SHIFT - 1;
const LOOK_SHIFT: u32 = SHIFT - (64 - STEP.leading_zeros() - 1);

/// Calculates the specificity of a pattern against a given string.
///
/// This function **assumes** that the `string` provided is already a **full match** for the `pattern`.
/// If the pattern does not match the string, the resulting specificity will be mathematically inconsistent and meaningless for comparison purposes.
/// 
/// ```rust
/// let string = "abc";
/// let high = get(string, "abc").unwrap();
/// let low = get(string, ".*").unwrap();
/// 
/// assert!(high > low);
/// ```
pub fn get(string: &str, pattern: &str) -> Result<u64, Box<regex_syntax::Error>> {
    let hir = Parser::new().parse(pattern).map_err(Box::new)?;
    let bytes = string.as_bytes();
    let (result, _) = calc(&hir, bytes, WEIGHT);
    Ok(result)
}

fn calc(hir: &Hir, bytes: &[u8], weight: u64) -> (u64, usize) {
    match hir.kind() {
        HirKind::Empty => (0, 0),

        HirKind::Literal(literal) => {
            let len = literal.0.len().min(bytes.len());
            (len as u64 * weight, len)
        }

        HirKind::Class(class) => {
            if bytes.is_empty() {
                return (0, 0);
            }

            let len = if let Class::Unicode(_) = class {
                str::from_utf8(bytes)
                    .ok()
                    .and_then(|s| s.chars().next())
                    .map(|c| c.len_utf8())
                    .unwrap_or(1)
            } else {
                1
            };

            let size: u64 = match class {
                Class::Unicode(u) => u.ranges().iter().map(|r| r.len() as u64).sum(),
                Class::Bytes(b) => b.ranges().iter().map(|r| r.len() as u64).sum(),
            };

            let result = if size <= 1 {
                weight
            } else {
                let shift = (BITS_U64 - size.leading_zeros()).min(MAX_SHIFT);
                (weight >> shift).max(1)
            };

            (result, len)
        }

        HirKind::Look(_) => ((weight >> LOOK_SHIFT).max(1), 0),

        HirKind::Repetition(repetition) => {
            let mut result = 0;
            let mut offset = 0;
            let mut weight = weight;

            while offset < bytes.len() {
                let (r, i) = calc(&repetition.sub, &bytes[offset..], weight);

                if i == 0 {
                    break;
                }

                result += r;
                offset += i;
                weight = weight.saturating_sub(i as u64 * STEP);
            }

            (result, offset)
        }

        HirKind::Capture(capture) => calc(&capture.sub, bytes, weight),

        HirKind::Concat(concat) => {
            let mut result = 0;
            let mut offset = 0;
            let mut weight = weight;

            for hir in concat {
                let bytes = if offset < bytes.len() {
                    &bytes[offset..]
                } else {
                    &[]
                };

                let (r, i) = calc(hir, bytes, weight);

                result += r;
                offset += i;
                weight = weight.saturating_sub(i as u64 * STEP);
            }
            (result, offset)
        }

        HirKind::Alternation(alternation) => {
            for hir in alternation {
                let (r, i) = calc(hir, bytes, weight);

                if i > 0 || matches!(hir.kind(), HirKind::Empty) {
                    return (r / (alternation.len() as u64).max(1), i);
                }
            }
            (0, 0)
        }
    }
}
