This crate allows you to write parser's tests like follows:
```rust
let input = "#( a b #items x y )*";
let spans = "   ^ ^ ^----^ ^ ^ ";
let expected = [
    Rule::token,
    Rule::token,
    Rule::interpolate,
    Rule::token,
    Rule::token,
];
```
Highlight each token with `^`, `^^` or `^---^` to indicate it's span.
Any parser can be used, as long as you implement `TestToken` trait for it's output.

For example:

```rust
struct TestToken {
    start: usize,
    end: usize,
    rule: Rule,
}

impl parser_test::Token for TestToken {
    type Rule = Rule;

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn rule(&self) -> Self::Rule {
        self.rule
    }
}
```

So far this crate is used to test pest parsers:

```rust
let mut output = Lexer::parse(Rule::interpolate_repetition, input).unwrap();
let output = output.next().unwrap().into_inner().map(|t| {
    let span = t.as_span();
    TestToken {
        start: span.start(),
        end: span.end() - 1,
        rule: t.as_rule()
    }
});
assert!(parser_test::test(output, expected, spans));
```
