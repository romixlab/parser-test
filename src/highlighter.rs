use std::str::{CharIndices};
use std::iter::Peekable;

/// Helper type that create (start, end) iterator over strings like "| ^--^ ^^".
/// Allows to write parser tests as follows:
/// test(
///      "struct X { field: u32 }",
///      "       |   ^---^  ^-^  ",
///      type_name("X"),
///      ident_name("field"),
///      any_ty("u32")
/// );
///
/// There is also an alternate space symbol: I that can be helpful for visual alignment.
pub struct Highlighter<'a> {
    spans: Peekable<CharIndices<'a>>,
}
impl<'a> Highlighter<'a> {
    pub fn new(spans: &'a str) -> Highlighter<'a> {
        Self {
            spans: spans.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Highlighter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.spans.next() {
            Some((mut start, mut c)) => {
                while c.is_whitespace() || c == 'I' {
                    start += 1;
                    c = match self.spans.next() {
                        Some((_, c)) => c,
                        None => return None,
                    };
                }

                match c {
                    '^' => {
                        while let Some((pos, c)) = self.spans.next() {
                            match c {
                                '-' => {
                                    continue;
                                }
                                '^' => {
                                    return Some((start, pos));
                                }
                                _ => panic!("Unexpected symbol in ^ span: {} at: {}", c, pos)
                            }
                        }
                        panic!("Unterminated ^ span at {}", start);

                    }
                    '|' => {
                        return Some((start, start));

                    }
                    _ => {
                        panic!("Unexpected character: {}, ^, | or whitespace is allowed", c);
                    }
                }
            }
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Highlighter;

    #[test]
    fn single_span() {
        let mut hl = Highlighter::new("|");
        assert_eq!(hl.next(), Some((0, 0)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn many_single_spans() {
        let mut hl = Highlighter::new("|||");
        assert_eq!(hl.next(), Some((0, 0)));
        assert_eq!(hl.next(), Some((1, 1)));
        assert_eq!(hl.next(), Some((2, 2)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn single_span_after_whitespace() {
        let mut hl = Highlighter::new("   |");
        assert_eq!(hl.next(), Some((3, 3)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn span_after_whitespace() {
        let mut hl = Highlighter::new(" ^---^  ");
        assert_eq!(hl.next(), Some((1, 5)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn two_single_bars() {
        let mut hl = Highlighter::new("| |");
        assert_eq!(hl.next(), Some((0, 0)));
        assert_eq!(hl.next(), Some((2, 2)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn double_caret_single_bar() {
        let mut hl = Highlighter::new("^^|");
        assert_eq!(hl.next(), Some((0, 1)));
        assert_eq!(hl.next(), Some((2, 2)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn two_double_carets() {
        let mut hl = Highlighter::new("^^^^");
        assert_eq!(hl.next(), Some((0, 1)));
        assert_eq!(hl.next(), Some((2, 3)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn long_span_single_span() {
        let mut hl = Highlighter::new("^--^|");
        assert_eq!(hl.next(), Some((0, 3)));
        assert_eq!(hl.next(), Some((4, 4)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn long_span_double_caret() {
        let mut hl = Highlighter::new("^--^^^");
        assert_eq!(hl.next(), Some((0, 3)));
        assert_eq!(hl.next(), Some((4, 5)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn many_spans() {
        let mut hl = Highlighter::new("^-^ ^^ ^--^ | ^----^  ");
        assert_eq!(hl.next(), Some((0, 2)));
        assert_eq!(hl.next(), Some((4, 5)));
        assert_eq!(hl.next(), Some((7, 10)));
        assert_eq!(hl.next(), Some((12, 12)));
        assert_eq!(hl.next(), Some((14, 19)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn alternate_space() {
        let mut hl = Highlighter::new("II^^^^");
        assert_eq!(hl.next(), Some((2, 3)));
        assert_eq!(hl.next(), Some((4, 5)));
        assert_eq!(hl.next(), None);
    }


    #[test]
    fn unterminated_span() {
        let mut hl = Highlighter::new("^--");
        let r = std::panic::catch_unwind(move || hl.next());
        assert!(r.is_err());
    }

    #[test]
    fn unterminated_span2() {
        let mut hl = Highlighter::new("^-- ");
        let r = std::panic::catch_unwind(move || hl.next());
        assert!(r.is_err());
    }

    #[test]
    fn unterminated_span3() {
        let mut hl = Highlighter::new("^^^");
        let _ = hl.next();
        let r = std::panic::catch_unwind(move || hl.next());
        assert!(r.is_err());
    }

    #[test]
    fn unterminated_span4() {
        let mut hl = Highlighter::new("^");
        let r = std::panic::catch_unwind(move || hl.next());
        assert!(r.is_err());
    }

    #[test]
    fn space_after_caret() {
        let mut hl = Highlighter::new("^ ");
        let r = std::panic::catch_unwind(move || hl.next());
        assert!(r.is_err());
    }

    #[test]
    fn bar_after_caret() {
        let mut hl = Highlighter::new("^|");
        let r = std::panic::catch_unwind(move || hl.next());
        assert!(r.is_err());
    }
}
