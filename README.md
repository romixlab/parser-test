This crate allows you to write parser's tests like follows:
```rust
let input = "#( a b #items x y )*";
let spans = "   ^ ^ ^----^ ^ ^   ";
let expected = [
    Rule::token,
    Rule::token,
    Rule::interpolate,
    Rule::token,
    Rule::token,
];
```
Highlight each token with `^`, `^^` or `^---^` to indicate it's span. `parser_test::test()` will make sure that
parser output containts excactly those tokens in specified positions.

Any parser can be used, as long as you implement `Token` trait for it's output.
Generic `TestToken` is provided and can probably be used for all use cases.

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
