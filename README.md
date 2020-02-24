# in-any-case

A Rust library for analyzing and evaluating domain-specific pattern-matching case-analyses.

Use this library if you need to analyze match statements in Rust, or things like this:

```rust
enum Boolean {
    True,
    False,
}

enum Fruit {
    Citrus,
    Berry,
    Other,
}

match (Boolean::False, Fruit::Other) {
    (Boolean::True, Fruit::Berry) => 1,
    (Boolean::True, x) => 2,
    _ => 3,
}
```

## Example Use

```rust
let boolean = VariantDef {
    variants: vec!["True", "False"],
};

let fruit = VariantDef {
    variants: vec!["Citrus", "Berry", "Other"],
};

let _cases = CaseAnalysis(vec![
    Case {
        pattern: Pattern::Tuple(vec![
            Pattern::Variant("True", &boolean),
            Pattern::Variant("Berry", &fruit),
        ]),
        result: 1,
    },
    Case {
        pattern: Pattern::Tuple(vec![
            Pattern::Variant("True", &boolean),
            Pattern::Variable("x"),
        ]),
        result: 2,
    },
    Case {
        pattern: Pattern::Variable("_"),
        result: 3,
    },
]);
```

## Functionality (Implemented and Planned)

- [ ] Exhaustiveness checking
- [ ] Pattern matching and destructuring
- [ ] Variable binding
- [ ] Wildcard patterns
- [ ] Matching on tuples of values
- [ ] Ability to disallow nested patterns for slightly better efficiency
