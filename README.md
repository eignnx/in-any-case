# in-any-case

A Rust library for analyzing and evaluating domain-specific pattern-matching case-analyses.

Use this library if you need to analyze match statements in Rust, or things like this:

```rust
match (foo, bar, baz) {
    (Foo1, Bar1, Baz1) => op1(),
    (Foo2, _, Baz2) => op1(),
    (_, _, Baz99) => op1(),
    _ => op3(),
}
```

## Functionality (Implemented and Planned)

- [ ] Exhaustiveness checking
- [ ] Pattern matching and destructuring
- [ ] Variable binding
- [ ] Wildcard patterns
- [ ] Matching on tuples of values
- [ ] Ability to disallow nested patterns for slightly better efficiency
