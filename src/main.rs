#[macro_use]
extern crate nom;

use nom::{IResult, digit, space};

// Parser definition

use std::str;
use std::str::FromStr;
use std::env;

named!(parens<i64>,
  delimited!(
    char!('('),
    expr,
    char!(')')
  )
);

named!(i64_digit<i64>,
  map_res!(
    map_res!(
      digit,
      str::from_utf8
    ),
    FromStr::from_str
  )
);

named!(factor<i64>,
  alt!(
    i64_digit |
    parens
  )
);

named!(term<i64>,
  chain!(
    mut acc: factor ~
    many0!(
      alt!(
        tap!(mul: preceded!(tag!("*"), factor) => acc = acc * mul) |
        tap!(div: preceded!(tag!("/"), factor) => acc = acc / div)
      )
    ),
    || { return acc }
  )
);

named!(expr<i64>,
  chain!(
    mut acc: term ~
    many0!(
      alt!(
        tap!(add: preceded!(tag!("+"), term) => acc = acc + add) |
        tap!(sub: preceded!(tag!("-"), factor) => acc = acc - sub)
      )
    ),
    || { return acc }
  )
);

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("The program requires two arguments");
    return;
  }

  println!("expr: {}", args[1]);
  match expr(args[1].as_bytes()) {
    IResult::Done(_, val) => println!("result: {}", val),
    IResult::Error(error) => println!("Error: {:?}", error),
    IResult::Incomplete(needed) => println!("Incomplete: {:?}", needed),
  }
}

#[test]
fn expr_test() {
  assert_eq!(expr(b"1+2"),          IResult::Done(&b""[..], 3));
  assert_eq!(expr(b"12+6-4+3"),     IResult::Done(&b""[..], 17));
  assert_eq!(expr(b"1+2*3+4"),      IResult::Done(&b""[..], 11));

  assert_eq!(expr(b"(2)"),          IResult::Done(&b""[..], 2));
  assert_eq!(expr(b"2*(3+4)"),      IResult::Done(&b""[..], 14));
  assert_eq!(expr(b"2*2/(5-1)+3"),  IResult::Done(&b""[..], 4));
}
