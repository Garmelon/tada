use chumsky::prelude::*;
use chumsky::text::Character;

use crate::ast::{Ident, NumLit};

fn ident() -> impl Parser<char, Ident, Error = Simple<char>> {
    text::ident().map(Ident)
}

fn num_lit_radix_digits(
    radix: u32,
    max_digits: usize,
) -> impl Parser<char, (i64, String), Error = Simple<char>> {
    filter(move |c: &char| c.is_digit(radix) || *c == '_')
        .repeated()
        .at_least(1)
        .try_map(move |chars, span| {
            let text = chars.iter().copied().collect::<String>();
            let digits = chars.into_iter().filter(|c| *c != '_').collect::<String>();
            if digits.len() > max_digits {
                return Err(Simple::custom(span, "number out of range"));
            }
            let number = u64::from_str_radix(&digits, radix).unwrap();
            if number > i64::MAX as u64 {
                return Err(Simple::custom(span, "number out of range"));
            }
            Ok((number as i64, text))
        })
}

fn num_lit_bin_digits() -> impl Parser<char, (i64, String), Error = Simple<char>> {
    // u64::MAX in binary is 0b_1111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111
    // 63 digits are necessary to represent the full range.
    const MAX_BIN_DIGITS: usize = 63;

    just("0b").ignore_then(num_lit_radix_digits(2, MAX_BIN_DIGITS))
}

fn num_lit_dec_digits() -> impl Parser<char, (i64, String), Error = Simple<char>> {
    // u64::MAX in decimal is 9_223_372_036_854_775_807
    // 19 digits are necessary to represent the full range.
    const MAX_DEC_DIGITS: usize = 19;

    num_lit_radix_digits(10, MAX_DEC_DIGITS)
}

fn num_lit_hex_digits() -> impl Parser<char, (i64, String), Error = Simple<char>> {
    // u64::MAX in hexadecimal is 0x_7fff_ffff_ffff_ffff
    // 16 digits are necessary to represent the full range.
    const MAX_HEX_DIGITS: usize = 16;

    just("0x").ignore_then(num_lit_radix_digits(16, MAX_HEX_DIGITS))
}

fn num_lit() -> impl Parser<char, NumLit, Error = Simple<char>> {
    (num_lit_bin_digits().map(|(num, text)| NumLit::Bin(num, text)))
        .or(num_lit_hex_digits().map(|(num, text)| NumLit::Hex(num, text)))
        .or(num_lit_dec_digits().map(|(num, text)| NumLit::Dec(num, text)))
}

pub fn parser() -> impl Parser<char, NumLit, Error = Simple<char>> {
    num_lit().padded().then_ignore(end())
}
