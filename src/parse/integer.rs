use std::ops::Neg;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, hex_digit1, one_of};
use nom::combinator::{map, map_res, opt, recognize};
use nom::error::ParseError;
use nom::multi::{many0, many1};
use nom::sequence::{pair, preceded};
use nom::{IResult, Parser};

fn signed<'a, O, E, F>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    O: Neg<Output = O>,
    E: ParseError<&'a str>,
    F: Parser<&'a str, O, E>,
{
    map(pair(opt(one_of("+-")), parser), |(sign, number)| {
        if sign == Some('-') {
            -number
        } else {
            number
        }
    })
}

fn dec(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(pair(digit1, many0(pair(many0(tag("_")), digit1)))),
        |digits: &str| digits.replace("_", "").parse::<i64>(),
    )(input)
}

fn hex(input: &str) -> IResult<&str, i64> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(pair(many0(tag("_")), hex_digit1))),
        ),
        |digits: &str| i64::from_str_radix(&digits.replace("_", ""), 16),
    )(input)
}

fn bin(input: &str) -> IResult<&str, i64> {
    map_res(
        preceded(
            alt((tag("0b"), tag("0B"))),
            recognize(many1(pair(many0(tag("_")), one_of("01")))),
        ),
        |digits: &str| i64::from_str_radix(&digits.replace("_", ""), 2),
    )(input)
}

pub fn integer(input: &str) -> IResult<&str, i64> {
    signed(alt((bin, hex, dec)))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dec() {
        // Normal numbers
        assert_eq!(dec("012345").unwrap().1, 12345);
        assert_eq!(dec("13_37").unwrap().1, 1337);
        assert_eq!(dec("1_____33__7").unwrap().1, 1337);
        assert_eq!(dec("2021_09_29").unwrap().1, 20210929);

        // At least one digit required
        assert!(dec("").is_err());
        assert_eq!(dec("0").unwrap().1, 0);

        // Trailing garbage
        assert_eq!(dec("123abc").unwrap(), ("abc", 123));
        assert_eq!(dec("123_abc").unwrap(), ("_abc", 123));
        assert_eq!(dec("123.456").unwrap(), (".456", 123));

        // Underscores
        assert!(dec("_123").is_err());
        assert_eq!(dec("1___2__________________________3").unwrap().1, 123);
        assert_eq!(dec("123_").unwrap(), ("_", 123));

        // No signs allowed
        assert_eq!(dec("4").unwrap(), ("", 4));
        assert!(dec("-4").is_err());
        assert!(dec("+4").is_err());
    }

    #[test]
    fn parse_hex() {
        // Normal numbers
        assert_eq!(hex("0xA83B40").unwrap().1, 0xA83B40);
        assert_eq!(hex("0X_CAFE_BABE").unwrap().1, 0x_CAFE_BABE);
        assert_eq!(hex("0x__1f__2e_c").unwrap().1, 0x1f2ec);
        assert_eq!(hex("0XaAaA_aaAAA").unwrap().1, 0xAAAAAAAAA);

        // Prefix required
        assert!(hex("1234").is_err());
        assert!(hex("fade").is_err());

        // At least one digit required
        assert!(hex("0x").is_err());
        assert_eq!(hex("0x0").unwrap().1, 0);

        // Trailing garbage
        assert_eq!(hex("0x123abcdefghi").unwrap(), ("ghi", 0x123abcdef));
        assert_eq!(hex("0x123_abc_def_ghi").unwrap(), ("_ghi", 0x123abcdef));
        assert_eq!(hex("0x12b.54c").unwrap(), (".54c", 0x12b));

        // Underscores
        assert!(hex("_0x123").is_err());
        assert_eq!(hex("0x_123").unwrap().1, 0x123);
        assert_eq!(hex("0x_1_____2__________3").unwrap().1, 0x123);
        assert_eq!(hex("0x123_").unwrap(), ("_", 0x123));

        // No signs allowed
        assert_eq!(hex("0x4").unwrap(), ("", 0x4));
        assert!(hex("-0x4").is_err());
        assert!(hex("+0x4").is_err());
        assert!(hex("0x-4").is_err());
        assert!(hex("0x+4").is_err());
    }

    #[test]
    fn parse_bin() {
        // Normal numbers
        assert_eq!(bin("0b101001").unwrap().1, 0b101001);
        assert_eq!(bin("0B_1100_1001").unwrap().1, 0b11001001);
        assert_eq!(bin("0b__10__11_0").unwrap().1, 0b10110);
        assert_eq!(bin("0B11_11_00_00").unwrap().1, 0b11110000);

        // Prefix required
        assert!(bin("1234").is_err());
        assert!(bin("fade").is_err());

        // At least one digit required
        assert!(bin("0b").is_err());
        assert_eq!(bin("0b0").unwrap().1, 0);

        // Trailing garbage
        assert_eq!(bin("0b0123").unwrap(), ("23", 0b01));
        assert_eq!(bin("0b0_1_2_3").unwrap(), ("_2_3", 0b01));
        assert_eq!(bin("0b101.1101").unwrap(), (".1101", 0b101));

        // Underscores
        assert!(bin("_0b110").is_err());
        assert_eq!(bin("0b_100").unwrap().1, 0b100);
        assert_eq!(bin("0b_1_____1__________1").unwrap().1, 0b111);
        assert_eq!(bin("0b010_").unwrap(), ("_", 0b010));

        // No signs allowed
        assert_eq!(bin("0b1").unwrap(), ("", 0x1));
        assert!(bin("-0b1").is_err());
        assert!(bin("+0b1").is_err());
        assert!(bin("0b-1").is_err());
        assert!(bin("0b+1").is_err());
    }

    #[test]
    fn parse_signed() {
        assert_eq!(signed(dec)("7_248_392").unwrap().1, 7248392);
        assert_eq!(signed(dec)("+7_248_392").unwrap().1, 7248392);
        assert_eq!(signed(dec)("-7_248_392").unwrap().1, -7248392);

        assert_eq!(signed(hex)("0xFEED_DAD").unwrap().1, 0xFEEDDAD);
        assert_eq!(signed(hex)("+0xFEED_DAD").unwrap().1, 0xFEEDDAD);
        assert_eq!(signed(hex)("-0xFEED_DAD").unwrap().1, -0xFEEDDAD);

        assert_eq!(signed(bin)("0b1110_110").unwrap().1, 0b1110110);
        assert_eq!(signed(bin)("+0b1110_110").unwrap().1, 0b1110110);
        assert_eq!(signed(bin)("-0b1110_110").unwrap().1, -0b1110110);
    }

    #[test]
    fn parse_integer() {
        assert_eq!(integer("1423805").unwrap().1, 1423805);
        assert_eq!(integer("-1_423_805").unwrap().1, -1423805);

        assert_eq!(integer("0xDAD_15_DEAD").unwrap().1, 0xdad15dead);
        assert_eq!(integer("-0x92d29f").unwrap().1, -0x92d29f);

        assert_eq!(integer("0b1001_1110").unwrap().1, 0b10011110);
        assert_eq!(integer("-0b0011001").unwrap().1, -0b0011001);
    }
}
