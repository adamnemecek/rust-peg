use super::{RuleResult, Parse, ParseElem, ParseLiteral, ParseSlice};

impl<'input, T: 'input> Parse<'input> for [T] {
    type Position = usize;
    type PositionRepr = usize;
    fn start(&'input self) -> usize { 0 }

    fn position_repr(&'input self, pos: usize) -> usize { pos }
}

impl<'input, T: 'input + Clone> ParseElem<'input> for [T] {
    type Element = T;

    fn parse_elem(&'input self, pos: usize) -> RuleResult<usize, T> {
        match self[pos..].first() {
            Some(c) => RuleResult::Matched(pos + 1, c.clone()),
            None => RuleResult::Failed
        }
    }
}

impl<'input> ParseLiteral<'input> for [u8] {
    fn parse_string_literal(&'input self, pos: usize, literal: &str) -> RuleResult<usize, ()> {
        let l = literal.len();
        if self.len() >= pos + l && &self[pos..pos+l] == literal.as_bytes() {
            RuleResult::Matched(pos+l, ())
        } else {
            RuleResult::Failed
        }
    }
}

impl<'input, T: 'input> ParseSlice<'input> for [T] {
    type Slice = &'input [T];
    fn parse_slice(&'input self, p1: usize, p2: usize) -> &'input [T] {
        &self[p1..p2]
    }
}