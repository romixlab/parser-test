use std::str::{CharIndices};
use std::iter::Peekable;

/// Helper type that create (start, end) iterator over strings like "^ ^--^ ^^".
/// Allows to write parser tests as follows:
/// test(
///      "struct X { field: u32 }",
///      "       ^   ^---^  ^-^  ",
///      type_name("My"),
///      ident_name("field"),
///      any_ty("u32")
/// );
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
                assert!(c == '^' || c.is_whitespace());

                while c.is_whitespace() {
                    start += 1;
                    c = match self.spans.next() {
                        Some((_, c)) => c,
                        None => return None,
                    };
                }

                let mut end = start;
                loop {
                    match self.spans.next() {
                        Some((pos, c)) => {
                            if c.is_whitespace() {
                                break;
                            } else if c == '-' {
                                match self.spans.peek() {
                                    Some((_, c)) => {
                                        if c.is_whitespace() {
                                            panic!("Wrong highlighter string: ^--^ sequence unterminated");
                                        }
                                    }
                                    None => {
                                        panic!("Wrong highlighter string: ^--^ sequence unterminated");
                                    }
                                }
                                continue;
                            } else if c == '^' {
                                end = pos;
                            } else {
                                panic!("Wrong highlighter string: only '^', '-' and ' ' are allowed");
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }
                Some((start, end))
            }
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Highlighter;

    #[test]
    fn single_caret_span() {
        let mut hl = Highlighter::new("^");
        assert_eq!(hl.next(), Some((0, 0)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn single_caret_span_after_whitespace() {
        let mut hl = Highlighter::new("   ^");
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
    fn two_single_carets_spans() {
        let mut hl = Highlighter::new("^ ^");
        assert_eq!(hl.next(), Some((0, 0)));
        assert_eq!(hl.next(), Some((2, 2)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn many_spans() {
        let mut hl = Highlighter::new("^-^ ^^ ^--^ ^ ^----^  ");
        assert_eq!(hl.next(), Some((0, 2)));
        assert_eq!(hl.next(), Some((4, 5)));
        assert_eq!(hl.next(), Some((7, 10)));
        assert_eq!(hl.next(), Some((12, 12)));
        assert_eq!(hl.next(), Some((14, 19)));
        assert_eq!(hl.next(), None);
    }

    #[test]
    fn unterminated_span() {
        let mut hl = Highlighter::new("^--");
        let r = std::panic::catch_unwind(move || hl.next());
        assert!(r.is_err());
    }

    #[test]
    fn interminated_span2() {
        let mut hl = Highlighter::new("^-- ");
        let r = std::panic::catch_unwind(move || hl.next());
        assert!(r.is_err());
    }
}