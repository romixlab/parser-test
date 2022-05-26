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
    let hl = highlighter::Highlighter::new(spans);
    let mut expected = expected.into_iter().zip(hl);
    for o in output {
        let e = expected.next().expect("More output than expected");
        if o.rule() != e.0 {
            panic!("Expected rule: {:?} got: {:?} at pos:{}", e.0, o.rule(), o.start());
        }
        if o.start() != e.1.0 || o.end() != e.1.1 {
            panic!("Spans do not match on rule: {:?}: [{}, {}] vs [{}, {}]", o.rule(), o.start(), o.end(), e.1.0, e.1.1);
        }
    }
    assert!(expected.next().is_none(), "Expected more output");

    true
}