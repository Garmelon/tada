use chumsky::prelude::*;

#[derive(Debug)]
pub enum Lit {
    Num(i64),
}

fn lit_num() -> impl Parser<char, i64, Error = Simple<char>> {
    let sign = just('+')
        .or(just('-'))
        .or_not()
        .map(|s| if s == Some('-') { -1_i128 } else { 1_i128 });

    let digits = text::int(10);

    sign.then(digits).try_map(|(sign, digits), span| {
        // u64::MIN and u32::MAX have 19 digits in base 10
        if digits.len() > 19 {
            return Err(Simple::custom(span, "number out of range"));
        }

        let number = sign * digits.parse::<i128>().unwrap();
        if number < i64::MIN.into() || number > u64::MAX.into() {
            return Err(Simple::custom(span, "number out of range"));
        }

        Ok(number as i64)
    })
}

fn lit() -> impl Parser<char, Lit, Error = Simple<char>> {
    lit_num().map(Lit::Num)
}

pub fn parser() -> impl Parser<char, Lit, Error = Simple<char>> {
    lit().padded().then_ignore(end())
}
