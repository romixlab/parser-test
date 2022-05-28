use std::fmt::Debug;

pub mod highlighter;

/// Parser output tokens should implement this trait
pub trait Token {
    type Rule;

    fn start(&self) -> usize;
    fn end(&self) -> usize;
    fn rule(&self) -> Self::Rule;
}

pub struct TestToken<R> {
    pub start: usize,
    pub end: usize,
    pub rule: R,
}

impl<R: Copy> Token for TestToken<R> {
    type Rule = R;

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

pub fn test<O, E, R>(output: O, expected: E, spans: &str) -> bool
    where O: IntoIterator,
          <O as IntoIterator>::Item: Token<Rule = R>,
          E: IntoIterator<Item = R>,
          R: Eq + Sized + Debug,
{
    let mut expected_spans = highlighter::Highlighter::new(spans);
    let mut expected = expected.into_iter();
    for actual_output in output {
        let expected_span = expected_spans.next().expect("Not enough spans defined");
        let expected_output = expected.next().expect("More output than expected");
        if actual_output.rule() != expected_output {
            panic!("Expected rule: {:?} got: {:?} at pos:{}",
                   expected_output,
                   actual_output.rule(),
                   actual_output.start()
            );
        }
        if actual_output.start() != expected_span.0 || actual_output.end() != expected_span.1 {
            panic!("Spans do not match on rule: {:?}: [{}, {}] vs [{}, {}]",
                   actual_output.rule(),
                   actual_output.start(),
                   actual_output.end(),
                   expected_span.0,
                   expected_span.1
            );
        }
    }
    assert!(expected.next().is_none(), "Expected more output");
    assert!(expected_spans.next().is_none(), "Defined more spans than expected rules");

    true
}
