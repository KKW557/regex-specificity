# Regex Specificity

[![Latest version](https://img.shields.io/crates/v/regex-specificity.svg)](https://crates.io/crates/regex-specificity)
[![Github Actions](https://img.shields.io/github/actions/workflow/status/KKW557/regex-specificity/rust.yml)](https://github.com/KKW557/regex-specificity/actions)
[![Docs](https://docs.rs/regex-specificity/badge.svg)](https://docs.rs/regex-specificity/latest/regex_specificity/)
[![Downloads](https://img.shields.io/crates/d/regex-specificity)](https://crates.io/crates/regex-specificity)

A heuristic-based crate to calculate the **specificity** of a regular expression pattern against a specific string.

## Concept

Specificity measures how "precise" a match is. For example, the pattern `abc` is more specific to the string "abc" than the pattern `a.c` or `.*`.

The calculation follows these principles:
1. **Positional Weighting**: Earlier matches contribute more to the total specificity than later ones.
2. **Certainty**: Literals (exact characters) specificity higher than character classes or wildcards.
3. **Information Density**: Narrower character classes (e.g., `[a-z]`) specificity higher than broader ones (e.g., `.`).
4. **Branching Penalty**: Patterns with many alternatives (alternations) are penalized as they are less specific.

## Usage

The `get` function **assumes** that the `string` provided is already a **full match** for the `pattern`.
If the pattern does not match the string, the resulting specificity will be mathematically inconsistent and meaningless for comparison purposes.

```rust
let string = "abc";
let high = get(string, "abc").unwrap();
let low = get(string, ".*").unwrap();

assert!(high > low);
```

## Counterintuitive

Since this crate uses a greedy heuristic based on the HIR (High-level Intermediate Representation), certain patterns may yield the same specificity even if they look different.
A common example is when a wildcard `.*` "swallows" the entire string before other parts of the pattern can be evaluated.

```rust
let string = "cat";

let pattern1 = r".*";
let pattern2 = r".*a.*";

assert_eq!(
    get(string, pattern1).unwrap(),
    get(string, pattern2).unwrap()
)
```

### Recommendation

If you need to distinguish between patterns with identical specificity, we recommend using the pattern length as a secondary tie-breaker:

* **Mathematical**: A longer pattern is often less specific because it requires more redundant components to describe the same set.
* **Intuitive**: You may prefer the pattern with more literals.

```rust
if result_a == result_b {
    return pattern_a.len().cmp(&pattern_b.len()); 
}
```

## License

This project is licensed under the [MIT License](/LICENSE) © 2025 557.
