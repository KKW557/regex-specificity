# regex-specificity

This crate provides a fast, heuristic-based approach to calculate the **specificity**
of a regular expression pattern against a specific string.

## Concept
Specificity measures how "precise" a match is. For example, the pattern `abc` is
more specific to the string "abc" than the pattern `a.c` or `.*`.

The calculation follows these principles:
1. **Positional Weighting**: Earlier matches contribute more to the total score than later ones.
2. **Certainty**: Literals (exact characters) score higher than character classes or wildcards.
3. **Information Density**: Narrower character classes (e.g., `[a-z]`) score higher than broader ones (e.g., `.`).
4. **Branching Penalty**: Patterns with many alternatives (alternations) are penalized as they are less specific.

## License

This project is licensed under the [MIT License](/LICENSE) © 2025 557.
